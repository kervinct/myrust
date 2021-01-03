use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F> FnBox for F where F: FnOnce() {
    fn call_box(self: Box<Self>) {
        (*self) ()
    }
}

type Thunk<'a> = Box<dyn FnBox + Send + 'a>;

struct ThreadPoolSharedData {
    name: Option<String>,  // 标记线程名称
    job_receiver: Mutex<Receiver<Thunk<'static>>>,  // 存储从Channel接收任务的接收端
    empty_trigger: Mutex<()>,  // 空锁，用于实现线程池的join
    empty_condvar: Condvar,  // 空条件变量，用于实现线程池的join
    queued_count: AtomicUsize,  // 代表线程池中总队列数，多线程用原子类型保证原子性
    active_count: AtomicUsize,  // 正在执行任务的工作线程数
    max_thread_count: AtomicUsize,  // 线程池允许的最大线程数
    panic_count: AtomicUsize,  // 记录发生恐慌的工作线程数
    stack_size: Option<usize>,  // 设置工作线程栈大小，默认8MB
}

impl ThreadPoolSharedData {
    /// 判断线程池是否正在工作
    fn has_work(&self) -> bool {
        self.queued_count.load(Ordering::SeqCst) > 0 ||
        self.active_count.load(Ordering::SeqCst) > 0
    }

    /// 通知所有阻塞线程解除阻塞
    fn no_work_notify_all(&self) {
        if !self.has_work() {
            *self.empty_trigger.lock()
                .expect("Unable to notify all joining threads");
            self.empty_condvar.notify_all();
        }
    }
}

pub struct ThreadPool {
    jobs: Sender<Thunk<'static>>,  // 存储发送端，用于给工作线程发送具体任务
    shared_data: Arc<ThreadPoolSharedData>,  // 记录工作线程共享的数据
}

impl ThreadPool {
    /// 初始化线程池
    pub fn new(num_threads: usize) -> ThreadPool {
        Builder::new().num_threads(num_threads).build()
    }
    /// 添加任务到队列
    pub fn execute<F>(&self, job: F)
        where F: FnOnce() + Send + 'static
    {
        self.shared_data
            .queued_count.fetch_add(1, Ordering::SeqCst);
        self.jobs.send(Box::new(job))
            .expect("unable to send job into queue.");
    }
    /// 在需要时阻塞主线程等待线程池中所有任务执行完毕
    pub fn join(&self) {
        if !self.shared_data.has_work() {  // 线程池闲置则提前返回
            return ();
        }
        let mut lock = self.shared_data.empty_trigger.lock().unwrap();
        while self.shared_data.has_work() {  // 若线程池中工作线程正在执行，则调用条件变量阻塞当前线程等待
            lock = self.shared_data
                .empty_condvar.wait(lock).unwrap();
        }
    }
}

#[derive(Clone, Default)]
pub struct Builder {
    num_threads: Option<usize>,  // 工作线程数
    thread_name: Option<String>,  // 线程名称
    thread_stack_size: Option<usize>,  // 线程栈大小
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            num_threads: None,
            thread_name: None,
            thread_stack_size: None,
        }
    }
    // 配置工作线程数
    pub fn num_threads(mut self, num_threads: usize) -> Builder {
        assert!(num_threads > 0);
        self.num_threads = Some(num_threads);
        self
    }
    // 初始化最终线程池
    pub fn build(self) -> ThreadPool {
        let (tx, rx) = channel::<Thunk<'static>>();  // 创建无界队列
        let num_threads = self.num_threads
            .unwrap_or_else(num_cpus::get);
        let shared_data = Arc::new(ThreadPoolSharedData {
            name: self.thread_name,
            job_receiver: Mutex::new(rx),
            empty_condvar: Condvar::new(),
            empty_trigger: Mutex::new(()),
            queued_count: AtomicUsize::new(0),
            active_count: AtomicUsize::new(0),
            max_thread_count: AtomicUsize::new(num_threads),
            panic_count: AtomicUsize::new(0),
            stack_size: self.thread_stack_size,
        }); 
        for _ in 0..num_threads {
            spawn_in_pool(shared_data.clone());
        }
        ThreadPool {
            jobs: tx,
            shared_data: shared_data,
        }
    }
}

fn spawn_in_pool(shared_data: Arc<ThreadPoolSharedData>) {
    let mut builder = thread::Builder::new();
    if let Some(ref name) = shared_data.name {
        builder = builder.name(name.clone());
    }

    if let Some(ref stack_size) = shared_data.stack_size {
        builder = builder.stack_size(stack_size.to_owned());
    }

    builder.spawn(move || {
        let sentinel = Sentinel::new(&shared_data);  // 对具体线程进行监控
        loop {  // 阻塞线程并从工作队列获取任务
            let thread_counter_val = shared_data
                .active_count.load(Ordering::Acquire);
            let max_thread_count_val = shared_data
                .max_thread_count.load(Ordering::Relaxed);
            if thread_counter_val >= max_thread_count_val {
                break;
            }
            let message = {
                let lock = shared_data.job_receiver.lock()
                    .expect("unable to lock job_receiver");
                lock.recv()
            };
            let job = match message {
                Ok(job) => job,
                Err(..) => break,
            };
            shared_data.queued_count.fetch_sub(1, Ordering::SeqCst);  // 获取到任务
            shared_data.active_count.fetch_add(1, Ordering::SeqCst);  // 工作线程执行
            job.call_box();
            shared_data.active_count.fetch_sub(1, Ordering::SeqCst);  // 工作线程空闲
            shared_data.no_work_notify_all();  // 通知阻塞线程恢复
        }
        sentinel.cancel();  // 设置实力状态，表示该线程正常执行完所有任务
    }).unwrap();
}

struct Sentinel<'a> {
    shared_data: &'a Arc<ThreadPoolSharedData>,
    active: bool,
}

impl<'a> Sentinel<'a> {
    fn new(shared_data: &'a Arc<ThreadPoolSharedData>) -> Sentinel<'a> {
        Sentinel {
            shared_data: shared_data,
            active: true,
        }
    }
    fn cancel(mut self) {
        self.active = false;
    }
}
impl<'a> Drop for Sentinel<'a> {
    /// 处理非正常状态的工作线程
    fn drop(&mut self) {
        if self.active {
            self.shared_data.active_count.fetch_sub(1, Ordering::SeqCst);
            if thread::panicking() {
                self.shared_data.panic_count.fetch_add(1, Ordering::SeqCst);
            }
            self.shared_data.no_work_notify_all();
            spawn_in_pool(self.shared_data.clone())
        }
    }
}

fn main() {
    let pool = ThreadPool::new(8);
    let test_count = Arc::new(AtomicUsize::new(0));
    for _ in 0..42 {
        let test_count = test_count.clone();
        pool.execute(move || {
            test_count.fetch_add(1, Ordering::Relaxed);
        });
    }
    pool.join();
    assert_eq!(42, test_count.load(Ordering::Relaxed));
}
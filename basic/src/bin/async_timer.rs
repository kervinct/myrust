use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex, mpsc::{
            sync_channel, Receiver, SyncSender
        }},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    }
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,  // 线程间通信Arc<Mutex<..>>
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,  // 运行中Future的waker，在值设为true后可以用它来通知Task可被唤醒
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut self>, cx: &mut Contet<'_>) -> Poll<Self::Output> {  // 实现
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;  // 值被设置为true，就可以调用wake通知Executor调用poll来推动
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }    
        });

        TimerFuture { shared_state }
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spwaner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued!");
    }
}

// ArcWake提供了简单构造Waker的方式
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued!");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if let Poll::Pending = future.as_mut().poll(context) {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("start!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    drop(spawner);

    executor.run();
}
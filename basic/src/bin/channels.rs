use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

static NTHREADS: usize = 3;

fn main() {
    let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();

    for id in 0..NTHREADS {
        let thread_tx = Sender::clone(&tx);

        thread::spawn(move || {
            thread_tx.send(id).unwrap();

            println!("thread {} finished", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS);
    for _ in 0..NTHREADS {
        ids.push(rx.recv().unwrap());
    }

    println!("{:?}", ids);
}
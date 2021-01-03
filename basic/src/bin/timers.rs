use std::time::Duration;
use std::thread;
use std::sync::mpsc;

fn main() {
    let interval = Duration::from_millis(1000);
    println!("Sleep for {} ms...", interval.as_millis());
    thread::sleep(interval);
    println!("Done");
}
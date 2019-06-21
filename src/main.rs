mod scheduler;

use scheduler::Scheduler;
use std::{thread, time};

fn main() {
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_process();
    thread::sleep(time::Duration::from_secs(5));
    scheduler.add_process();

    let scheduler_handle = thread::spawn(move || scheduler.start());
    thread::sleep(time::Duration::from_secs(10));
    // scheduler.stop();
    scheduler_handle.join().expect("Couldn't join scheduler thread with main thread");
    // scheduler.pause();
}

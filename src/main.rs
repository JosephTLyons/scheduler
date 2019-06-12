mod scheduler;

use scheduler::Scheduler;
use std::{thread, time};

fn main() {
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_process();
    scheduler.add_process();
    scheduler.add_process();
    scheduler.add_process();

    let scheduler_handle = thread::spawn(move || scheduler.start());
    thread::sleep(time::Duration::from_millis(10000));
    // scheduler.stop();
    scheduler_handle.join().expect("Couldn't join scheduler thread with main thread");
    // scheduler.pause();
}

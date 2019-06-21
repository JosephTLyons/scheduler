mod scheduler;

use scheduler::Scheduler;
use scheduler::process_block::worker_functions;
use std::{thread, time};

fn main() {
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_process(worker_functions::loop_print_text_1);
    thread::sleep(time::Duration::from_secs(5));
    scheduler.add_process(worker_functions::loop_print_text_2);

    let scheduler_handle = thread::spawn(move || scheduler.start());
    thread::sleep(time::Duration::from_secs(10));
    // scheduler.stop();
    scheduler_handle.join().expect("Couldn't join scheduler thread with main thread");
    // scheduler.pause();
}

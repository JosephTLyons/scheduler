mod scheduler;

use scheduler::Scheduler;

fn main() {
    let mut scheduler: Scheduler = Scheduler::new();
    scheduler.add_process();
    scheduler.add_process();
    scheduler.add_process();
    scheduler.start();

    //
    // println!();
    //
    // if ! scheduler.block_process(6) {
    //     println!("Couldn't block process.");
    // }
    //
    // scheduler.print_all_process_details();
    //
    // println!("Number of processs in process table: {}", scheduler.get_number_of_processs());
}

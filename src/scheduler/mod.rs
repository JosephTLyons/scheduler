pub mod process;

use std::{thread, time};
pub use process::ProcessState;
use process::Process;

enum SchedulerState {
    Running,
    Paused,
    Stopped
}

pub struct Scheduler {
    process_table: Vec<Process>,
    total_number_of_processes: u32,
    current_running_process_id: u32,
    scheduler_state: SchedulerState,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            process_table: Vec::new(),
            total_number_of_processes: 0,
            current_running_process_id: 0,
            scheduler_state: SchedulerState::Stopped,
        }
    }

    pub fn start(&mut self) {
        // Launch a new thread that runs this function, let main thread go back to main
        // thread::spawn(move |self| {
        // });

        self.scheduler_state = SchedulerState::Running;

        loop {
            for process in &mut self.process_table {
                process.set_state(ProcessState::Running);

                // Have thread do work here in place of printing the details
                self.print_all_process_details();
                println!();
                thread::sleep(time::Duration::from_millis(1000));
                process.set_state(ProcessState::Ready);

                match self.scheduler_state {
                    SchedulerState::Paused => {
                        // Pause thread or block it with a mutex
                    }
                    SchedulerState::Stopped => return,
                    _ => {},
                }
            }
        }
    }

    pub fn pause(&mut self) {
        self.scheduler_state = SchedulerState::Paused;
    }

    pub fn add_process(&mut self) {
        self.process_table.push(Process::new(ProcessState::Ready, self.total_number_of_processes));
        self.total_number_of_processes += 1;
    }

    pub fn print_all_process_details(&self) {
        for process in &self.process_table {
            process.print_process_details();
        }
    }

    // Returns true if it found and blocked the process with requested id, otherwise, reeturns false
    pub fn block_process(&mut self, id_of_process_to_block: u32) -> bool {
        for process in &mut self.process_table {
            if process.get_process_id() == id_of_process_to_block {
                process.set_state(ProcessState::Blocked);
                return true;
            }
        }

        false
    }

    pub fn get_number_of_processes_in_table(&self) -> usize {
        self.process_table.len()
    }
}

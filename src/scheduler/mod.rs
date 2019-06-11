pub mod process;

use std::{thread, time};
pub use process::ProcessState;
use process::Process;

pub struct Scheduler {
    process_table: Vec<Process>,
    number_of_processs: u32,
    current_running_process_id: u32,
    is_running: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            process_table: Vec::new(),
            number_of_processs: 0,
            current_running_process_id: 0,
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        self.is_running = true;

        loop {
            for process in &mut self.process_table {
                process.set_state(ProcessState::Running);
                self.print_all_process_details();
                println!();
                thread::sleep(time::Duration::from_millis(1000));
                process.set_state(ProcessState::Ready);

                if ! self.is_running {
                    return;
                }
            }
        }
    }

    pub fn add_process(&mut self) {
        self.process_table
            .push(Process::new(ProcessState::Ready, self.number_of_processs));
        self.number_of_processs += 1;
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

    pub fn get_number_of_processs(&self) -> usize {
        self.process_table.len()
    }
}

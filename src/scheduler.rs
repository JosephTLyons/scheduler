pub mod process_block;

use process_block::worker_functions;
use process_block::ProcessBlock;
pub use process_block::ProcessState;
use std::thread::JoinHandle;
// use std::thread::Thread;
use std::{thread, time};
// use std::sync::atomic::AtomicBool;

enum SchedulerState {
    Running,
    Paused,
    Stopped,
}

pub struct Scheduler {
    process_table: Vec<ProcessBlock>,
    total_number_of_processes: u32,
    current_running_process_id: u32,
    scheduler_state: SchedulerState,
    scheduler_thread_handle_opt: Option<JoinHandle<()>>,
    scheduler_speed_ms: u32,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            process_table: Vec::new(),
            total_number_of_processes: 0,
            current_running_process_id: 0,
            scheduler_state: SchedulerState::Stopped,
            scheduler_thread_handle_opt: None,
            scheduler_speed_ms: 1000,
        }
    }

    pub fn start(&mut self) {
        // self.scheduler_thread_handle_option = Some(thread::current());
        self.scheduler_state = SchedulerState::Running;

        loop {
            // This can go back to iterating when the debugging / printing_all_process_details()
            // function is replaced by actual thread code
            // for process in &self.process_table {
            for i in 0..self.process_table.len() {
                self.process_table[i].set_state(ProcessState::Running);


                // Delete this when process launching works
                self.print_all_process_details();
                println!();
                thread::sleep(time::Duration::from_millis(self.scheduler_speed_ms.into()));
                self.process_table[i].set_state(ProcessState::Ready);

                match self.scheduler_state {
                    SchedulerState::Paused => {
                        loop {
                            thread::sleep(time::Duration::from_millis(
                                self.scheduler_speed_ms.into(),
                            ));
                            println!("Scheduler is paused.");
                        }

                        // Pause this thread or block it with a mutex
                    }
                    SchedulerState::Stopped => return,
                    SchedulerState::Running => {},
                }
            }
        }
    }

    pub fn pause(&mut self) {
        self.scheduler_state = SchedulerState::Paused;
    }

    pub fn stop(&mut self) {
        self.scheduler_state = SchedulerState::Stopped;
    }

    // fn scheduler_join(&mut self) {
    //     if let Some(thread) = &mut self.scheduler_thread_handle_option {
    //         thread.
    //     }
    // }

    pub fn add_process(&mut self, f: fn(),) {
        self.process_table.push(ProcessBlock::new(
            ProcessState::Ready,
            self.total_number_of_processes,
            f,
        ));

        self.total_number_of_processes += 1;
        self.process_table[self.process_table.len() - 1].launch();
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
                if let ProcessState::Running = process.get_state() {
                    process.set_state(ProcessState::Blocked);
                    // Block process
                    return true;
                }
            }
        }

        false
    }

    pub fn get_number_of_processes_in_table(&self) -> usize {
        self.process_table.len()
    }
}

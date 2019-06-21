pub mod worker_functions;

use std::sync::Arc;
// use worker_functions::*;
use std::thread::{self, *};
// use std::time;
use std::sync::atomic::AtomicBool;

#[derive(Clone)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Dead,
}

pub struct ProcessBlock  {
    process_state: ProcessState,
    process_id: u32,
    thread_handle_option: Option<JoinHandle<(fn())>>,
    should_run_atomic_bool: Arc<AtomicBool>,
}

impl ProcessBlock {
    pub fn new(state: ProcessState, id: u32, f: fn()) -> Self {
        ProcessBlock {
            process_state: state,
            process_id: id,
            thread_handle_option: None,
            should_run_atomic_bool: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn launch(&mut self, f: fn()) {
        self.thread_handle_option = Some(thread::spawn(move || f));
    }

    pub fn pause() {

    }

    pub fn kill(&mut self) {
        // Switch this to a non-panicking method
        // self.thread_handle_option.expect("Doesn't have a running thread").thread().
        self.process_state = ProcessState::Dead;
    }

    // pub fn join(&self) {
    //     if let Some(handle) = self.thread_handle_option {
    //         handle.join();
    //     }
    // }

    pub fn print_process_details(&self) {
        print!("Process {}: ", self.process_id);

        match self.process_state {
            ProcessState::Ready => println!("ready"),
            ProcessState::Running => println!("running"),
            ProcessState::Blocked => println!("blocked"),
            ProcessState::Dead => println!("dead"),
        }
    }

    pub fn get_process_id(&self) -> u32 {
        self.process_id
    }

    pub fn set_state(&mut self, state: ProcessState) {
        self.process_state = state;
    }

    pub fn get_state(&self) -> ProcessState {
        self.process_state.clone()
    }
}

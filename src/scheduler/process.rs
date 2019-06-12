use std::thread::*;

#[derive(Clone)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Dead,
}

pub struct Process {
    process_state: ProcessState,
    process_id: u32,
    thread_handle_option: Option<JoinHandle<()>>,
}

impl Process {
    pub fn new(state: ProcessState, id: u32) -> Self {
        Process {
            process_state: state,
            process_id: id,
            thread_handle_option: None,
        }
    }

    pub fn launch(&mut self) {
        self.thread_handle_option =  Some(spawn(|| println!("Hello")));
    }

    pub fn pause() {
    }

    pub fn kill(&mut self) {
        // Switch this to a non-panicking method
        // self.thread_handle_option.expect("Doesn't have a running thread").thread().
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

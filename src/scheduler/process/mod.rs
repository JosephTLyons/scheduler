pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Dead,
}

pub struct Process {
    process_state: ProcessState,
    process_id: u32,
}

impl Process {
    pub fn new(state: ProcessState, id: u32) -> Self {
        Process {
            process_state: state,
            process_id: id,
        }
    }

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
}

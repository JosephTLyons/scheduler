use std::{thread, time};

pub fn loop_print_text_1() {
    loop {
        if true {
            println!("Hello, World!");
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}

pub fn loop_print_text_2() {
    loop {
        if true {
            println!("D'oh!");
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}

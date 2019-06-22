use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Input {
    handle: thread::JoinHandle<()>,
}

impl Input {
    pub fn new(conn: Arc<Mutex<(u8)>>) -> Input {
        let handle = thread::spawn(move || {
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            let i = trimmed.parse::<u8>().unwrap();
            let mut num = conn.lock().unwrap();
            *num = i;
            std::mem::drop(num);
            println!("your integer input: {}", i);
        });

        return Input { handle };
    }

    pub fn wait(self) {
        self.handle.join().unwrap_or_else(|_error| {
            return;
        });
    }
}

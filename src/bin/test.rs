use std::{thread, time};
use std::io::Write;
use std::io::stdout;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    loop {
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        print!("\rTEST LOG | {:?}", timestamp);
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(1000));
    }
}

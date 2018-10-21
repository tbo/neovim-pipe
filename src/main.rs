extern crate neovim_lib;
extern crate regex;
use neovim_lib::{Neovim, NeovimApi, Session};
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let re = Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[m|K]").unwrap();
    let mut session = Session::new_unix_socket(env!("NVIM_LISTEN_ADDRESS")).unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);
    nvim.command("call DWM_New() | setlocal buftype=log nobuflisted noswapfile readonly modifiable").unwrap();
    let current_buffer = nvim.get_current_buf().unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // nvim.command("setlocal modifiable").unwrap();
        let line = line.expect("Could not read line from standard in");
        let text = re.replace_all(&line, "");
        current_buffer.set_lines(&mut nvim, -1, -1, true, vec![text.to_string()]).unwrap();
        // nvim.command("setlocal nomodifiable").unwrap();
    }
    
    // loop {
    //     nvim.command("setlocal modifiable").unwrap();
    //     let mut buffer = String::new();
    //     io::stdin().read_to_string(&mut buffer).unwrap();
    //     let text = buffer.split("\n").map(|t| t.to_owned()).collect();
    //     current_buffer.set_lines(&mut nvim, -1, 1, true, text).unwrap();
    //     nvim.command("1d | $d | setlocal nomodifiable").unwrap();
    // }
    
    // loop {
    //     let mut input = String::new();
    //     let stdin = io::stdin();
    //     let mut stdin = stdin.lock(); 
    //     stdin
    //         // .read_line(&mut input)
    //         .read_to_string(&mut input)
    //         .expect("failed to read from pipe");
    //     let text = input.split("\n").map(|t| t.to_owned()).collect();
    //     if input == "" {
    //         break;
    //     }
    //     current_buffer.set_lines(&mut nvim, -1, -1, true, text).unwrap();
    // }
    // let stdin = io::stdin();
    // let mut stdin = stdin.lock(); // locking is optional
    //
    // let mut line = String::new();
    //
    // // Could also `match` on the `Result` if you wanted to handle `Err` 
    // while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
    //     if n_bytes == 0 { break }
    //     let text = line.split("\n").map(|t| t.to_owned()).collect();
    //     current_buffer.set_lines(&mut nvim, -1, 1, true, text).unwrap();
    //     line.clear();
    // }
}

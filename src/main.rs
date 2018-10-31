extern crate neovim_lib;
extern crate regex;
use neovim_lib::{Neovim, NeovimApi, Session, Handler};
use std::io::{self, BufRead};
use regex::Regex;

// struct EventHandler;
// impl Handler for EventHandler {
//     fn handle_notify(&mut self, _name: &str, _args: Vec<String>) {
//     }
//     // fn handle_request(&mut self, name: &str, args: Vec<String>) { 
//     //     Ok("Hello")
//     // } 
// }

fn main() {
    let re = Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[m|K]").unwrap();
    let mut session = Session::new_unix_socket(env!("NVIM_LISTEN_ADDRESS")).unwrap();
    // session.start_event_loop_channel_handler(EventHandler);
    session.start_event_loop();
    let mut nvim = Neovim::new(session);
    nvim.command("call DWM_New() | setlocal signcolumn=no nonumber norelativenumber filetype=log noswapfile readonly modifiable").unwrap();
    let current_buffer = nvim.get_current_buf().unwrap();
    let current_window = nvim.get_current_win().unwrap();
    nvim.subscribe("nvim_buf_detach_event").unwrap();
    let stdin = io::stdin();
    let mut pos = 0;
    for line in stdin.lock().lines() {
        nvim.command("setlocal modifiable").unwrap();
        let line = line.expect("Could not read line from standard in");
        let text = re.replace_all(&line, "");
        current_buffer.set_lines(&mut nvim, pos, -1, true, vec![text.to_string()]).unwrap();
        current_window.set_cursor(&mut nvim, (pos + 1, 1)).unwrap();
        pos += 1;
        nvim.command("setlocal nomodifiable").unwrap();
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

extern crate tokio_stdin_stdout;
extern crate tokio_batch;
extern crate neovim_lib;
extern crate regex;
extern crate tokio;
extern crate tokio_codec;
extern crate chrono;
extern crate futures;

use regex::Regex;
use std::process;
use std::time::{Duration, Instant};
use neovim_lib::{Neovim, NeovimApi, Session, Value, RequestHandler, Handler};
use tokio::runtime::Runtime;
use tokio::prelude::{Stream};
use tokio_codec::{FramedRead, LinesCodec};
use tokio_batch::*;
use chrono::prelude::*;
use futures::Future;

fn get_current_datetime() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn get_separator() -> std::vec::Vec<String> {
    vec!["".into(), format!("················ {} ················", get_current_datetime()), "".into()]
}

const SCROLL_BACK: i64 = 10000;

// struct NeovimHandler();
//
// impl RequestHandler for NeovimHandler {
//     fn handle_request(&mut self, name: &str, _args: Vec<Value>) -> Result<Value, Value> {
//         println!("This happend: {}", name);
//         // process::exit(0x0100);
//         Ok(Value::Boolean(true))
//     }
// }
//
// impl Handler for NeovimHandler {
//     fn handle_notify(&mut self, name: &str, _args: Vec<Value>) {
//         if let "nvim_buf_detach_event" = name {
//             println!("it worked!");
//             process::exit(0);
//             // process::abort();
//             // panic!();
//         }
//     }
// }

fn main() {
    let re = Regex::new(r"^.*\r|\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[m|K]").unwrap();
    let mut session = Session::new_unix_socket(env!("NVIM_LISTEN_ADDRESS")).unwrap();
    let receiver = session.start_event_loop_channel();
    // session.start_event_loop_handler(NeovimHandler());
    // session.start_event_loop_channel_handler(NeovimHandler());
    let mut nvim = Neovim::new(session);
    nvim.command("call DWM_New() | setlocal signcolumn=no nonumber norelativenumber filetype=log noswapfile readonly").unwrap();
    let current_buffer = nvim.get_current_buf().unwrap();
    let current_window = nvim.get_current_win().unwrap();
    let stdin = tokio_stdin_stdout::stdin(0);
    let framed_stdin = FramedRead::new(stdin, LinesCodec::new()).and_then(move |line| {
        Ok(re.replace_all(&line, "").into())
    });
    let batched_framed_stdin  = Chunks::new(framed_stdin, 2500, Duration::new(0, 320000));
    let mut last_update = Instant::now();
    current_buffer.attach(&mut nvim, true, vec![]).unwrap();
    nvim.subscribe("mut cursor-moved").unwrap();
    nvim.subscribe("insert-enter").expect("error: cannot subscribe to event: insert-enter");
    let future = batched_framed_stdin
        .map(move |lines| {
            if last_update.elapsed() > Duration::from_secs(10) { 
                last_update = Instant::now();
                lines.into_iter().chain(get_separator()).collect() 
            } else { 
                lines 
            }
        })
        .map(move |lines| {
            current_buffer.set_option(&mut nvim, "modifiable", Value::Boolean(true)).unwrap();
            let pos = current_buffer.line_count(&mut nvim).unwrap();
            let mut new_position = pos + (lines.len() as i64);
            let cursor_position = current_window.get_cursor(&mut nvim).unwrap().0;
            current_buffer.set_lines(&mut nvim, pos, -1, true, lines).unwrap();
            if new_position > SCROLL_BACK {
                current_buffer.set_lines(&mut nvim, 0, new_position - SCROLL_BACK, true, vec![]).unwrap();
                new_position = SCROLL_BACK;
            }
            if cursor_position >= pos {
                current_window.set_cursor(&mut nvim, (new_position, 1)).unwrap();
            }
            // current_buffer.set_option(&mut nvim, "modifiable", Value::Boolean(false)).unwrap();
            // Ok(tokio_batch::Error(()))
            // Err(())
        }).collect();
    // tokio::run(future.then(|res| {
    //     // match res {
    //     //     Err(_) => assert!(false),
    //     //     Ok(v) => assert_eq!(vec![vec![5]], v),
    //     // };
    //     Ok(())
    // }));
    // tokio::run(future);
    // tokio::run(future);
    // tokio::executor::current_thread::block_on_all(future);
    // tokio::executor::current_thread::block_on_all(future).unwrap();
    tokio::runtime::current_thread::Runtime::new().unwrap().block_on(future).unwrap();
    // process::exit(0);
    //
    // let mut rt = Runtime::new().unwrap();
    // fn stop() {
    //     rt.shutdown_now().map(|_| process::exit(0));
    // }
    // rt.spawn(future);
    // for (event, values) in receiver {
    //     println!("Something happend?: {}", event);
    //     if "nvim_buf_detach_event" == event {
    //         // println!("I'm supposed to shutdown now");
    //         // rt.shutdown_now().wait().unwrap();
    //         // stop();
    //         // process::abort();
    //         process::exit(0);
    //     }
    // }
}

extern crate tokio_stdin_stdout;
extern crate tokio_batch;
extern crate neovim_lib;
extern crate regex;
extern crate tokio;
extern crate tokio_codec;

use regex::Regex;
use std::time::Duration;
use neovim_lib::{Neovim, NeovimApi, Session};
use tokio::prelude::{Stream};
use tokio_codec::{FramedRead, LinesCodec};
use tokio_batch::*;

fn main() {
    let re = Regex::new(r"^.*\r|\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[m|K]").unwrap();
    let mut session = Session::new_unix_socket(env!("NVIM_LISTEN_ADDRESS")).unwrap();
    session.start_event_loop();
    let mut nvim = Neovim::new(session);
    nvim.command("call DWM_New() | setlocal signcolumn=no nonumber norelativenumber filetype=log noswapfile readonly modifiable").unwrap();
    let current_buffer = nvim.get_current_buf().unwrap();
    let current_window = nvim.get_current_win().unwrap();
    nvim.subscribe("nvim_buf_detach_event").unwrap();
    let mut pos = 0;
    let stdin = tokio_stdin_stdout::stdin(0);
    let framed_stdin = FramedRead::new(stdin, LinesCodec::new()).and_then(move |line| {
        Ok(re.replace_all(&line, "").into())
    });
    let batched_framed_stdin  = Chunks::new(framed_stdin, 500, Duration::new(0, 160000));
    let future = batched_framed_stdin.for_each(move |lines| {
            nvim.command("setlocal modifiable").unwrap();
            let new_position = pos + (lines.len() as i64);
            current_buffer.set_lines(&mut nvim, pos, -1, true, lines).unwrap();
            let cursor_position = current_window.get_cursor(&mut nvim).unwrap().0;
            if cursor_position >= pos {
                current_window.set_cursor(&mut nvim, (new_position, 1)).unwrap();
            }
            pos = new_position;
            nvim.command("setlocal nomodifiable").unwrap();
            Ok(())
        });
    tokio::runtime::current_thread::Runtime::new().unwrap().block_on(future).unwrap();
}

extern crate k_board;
extern crate serde;
extern crate serde_yaml;
extern crate libc;

mod terminal;
mod keyboard;
mod configurer;
mod base;
mod enums;

use terminal::tty::*;
use keyboard::buffer::*;
use keyboard::manager::*;
use std::io;
use std::io::*;
use enums::key_mapper;
use enums::Key;


fn main() {
    terminal::executor::execute("pwd");
    base::init();
    loop
    {
        let key:Key = watch();
        map_activity(key);
        let _ = write_tty(get_buffer().as_bytes());
    }
}

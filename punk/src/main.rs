extern crate libc;
extern crate serde;
extern crate serde_yaml;

mod base;
mod configurer;
mod enums;
mod keyboard;
mod terminal;

use enums::Key;
use keyboard::buffer::*;
use keyboard::manager::*;
use terminal::tty::*;

fn main() {
    terminal::executor::execute("pwd");
    base::init();
    loop {
        let key: Key = watch();
        map_activity(key);
        let _ = write_tty(get_buffer().as_bytes());
    }
}

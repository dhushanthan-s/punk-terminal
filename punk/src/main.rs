extern crate k_board;
extern crate serde;
extern crate serde_yaml;

mod terminal;
mod keyboard;
mod configurer;

use k_board::keys::Keys;
use terminal::tty::*;
use keyboard::buffer::*;
use keyboard::manager::*;



fn main() {
    terminal::executor::execute("pwd");
    loop
    {
        let key:Keys = watch();
        if key.eq(&Keys::Null)
        {
            continue;
        }
        map_activity(key);
        let _ = write_tty(get_buffer().as_bytes());
    }
}

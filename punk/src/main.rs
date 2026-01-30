mod base;
mod configurer;
mod keyboard;

use punk_terminal::event::keyboard::enums::Key;
use keyboard::buffer::*;
use punk_terminal::event::keyboard::manager::*;
use punk_terminal::tty::*;

fn main() {
    execute("pwd");
    base::init();
    loop {
        let key: Key = watch();
        map_activity(key);
        let _ = write_tty(get_buffer().as_bytes());
    }
}

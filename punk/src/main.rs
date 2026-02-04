mod base;

use punk_terminal::tty::*;
use punk_utils::input::Key;
use punk_utils::input::*;

fn main() {
    execute("pwd");
    base::init();
    loop {
        let key: Key = watch();
        map_activity(key);
        let _ = write_tty(get_buffer().as_bytes());
    }
}

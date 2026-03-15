mod base;

use punk_terminal::tty::{send_to_shell, write_tty};
use punk_utils::input::Key;
use punk_utils::input::*;

fn main() {
    base::init();
    loop {
        let key: Key = watch();
        match key {
            Key::Enter => {
                let _ = write_tty(b"\r\n");
                let cmd = get_buffer();
                clear_buffer();
                if !cmd.is_empty() {
                    send_to_shell(&cmd);
                }
            }
            Key::Letter(c) => {
                map_activity(Key::Letter(c));
                let mut buf = [0u8; 4];
                let s = c.encode_utf8(&mut buf);
                let _ = write_tty(s.as_bytes());
            }
            Key::Delete => {
                map_activity(Key::Delete);
                let _ = write_tty(b"\x08 \x08");
            }
            other => {
                map_activity(other);
            }
        }
    }
}

mod base;

use punk_terminal::tty::{send_to_shell, write_tty};
use punk_utils::input::Key;
use punk_utils::input::*;

fn main() {
    base::init();
    loop {
        let key_event = watch();
        match &key_event.key {
            Key::Enter => {
                let _ = write_tty(b"\r\n");
                let cmd = get_buffer();
                clear_buffer();
                if !cmd.is_empty() {
                    send_to_shell(&cmd);
                }
            }
            Key::Letter(c) => {
                map_activity(key_event.clone());
                if !key_event.modifiers.has_chord_modifier() {
                    let mut buf = [0u8; 4];
                    let s = c.encode_utf8(&mut buf);
                    let _ = write_tty(s.as_bytes());
                }
            }
            Key::Backspace => {
                map_activity(key_event.clone());
                let _ = write_tty(b"\x08 \x08");
            }
            _ => {
                map_activity(key_event);
            }
        }
    }
}

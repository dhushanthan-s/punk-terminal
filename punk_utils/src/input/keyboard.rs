use super::Key;
use super::KeyEvent;

/// Canonical key name for the base key (no modifiers). Used for keybind strings.
fn key_name(key: &Key) -> String {
    match key {
        Key::Letter(c) => {
            if *c == ' ' {
                "space".to_string()
            } else {
                c.to_lowercase().collect()
            }
        }
        Key::Arrow("up") => "up".to_string(),
        Key::Arrow("down") => "down".to_string(),
        Key::Arrow("left") => "left".to_string(),
        Key::Arrow("right") => "right".to_string(),
        Key::Enter => "enter".to_string(),
        Key::Backspace => "backspace".to_string(),
        Key::Tab => "tab".to_string(),
        Key::Esc => "escape".to_string(),
        Key::Delete => "delete".to_string(),
        Key::F(n) => format!("f{}", n),
        Key::Home => "home".to_string(),
        Key::End => "end".to_string(),
        Key::Unknown => String::new(),
        _ => String::new(),
    }
}

/// Build canonical keybind string from KeyEvent: e.g. "ctrl+shift+d", "f1", "alt+enter".
/// Order: ctrl, shift, alt, super, then key name.
pub fn key_fn_name_mapper(key_event: KeyEvent) -> String {
    let name = key_name(&key_event.key);
    if name.is_empty() {
        return String::new();
    }
    let prefix = key_event.modifiers.to_prefix();
    if prefix.is_empty() {
        name
    } else {
        format!("{}{}", prefix, name)
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn key_activity_mapper(key_event: KeyEvent) {
    unsafe {
        super::with_buffer(|buffer| match &key_event.key {
            Key::Arrow("left") => buffer.handle_left(),
            Key::Arrow("right") => buffer.handle_right(),
            Key::Backspace => buffer.backspace(),
            Key::Delete => buffer.delete(),
            Key::Enter => {}
            Key::Letter(c) if !key_event.modifiers.has_chord_modifier() => buffer.push(*c),
            Key::Letter(_) => buffer.pass(key_event),
            _ => buffer.pass(key_event),
        });
    }
}

#[cfg(target_os = "windows")]
pub fn key_activity_mapper(key_event: KeyEvent) {
    unsafe {
        super::with_buffer(|buffer| match &key_event.key {
            Key::Arrow("left") => buffer.handle_left(),
            Key::Arrow("right") => buffer.handle_right(),
            Key::Backspace => buffer.backspace(),
            Key::Delete => buffer.delete(),
            Key::Enter => {}
            Key::Letter(c) if !key_event.modifiers.has_chord_modifier() => buffer.push(*c),
            Key::Letter(_) => buffer.pass(key_event),
            _ => buffer.pass(key_event),
        });
    }
}

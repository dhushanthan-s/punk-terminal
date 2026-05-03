use crate::config::keybinder;
use std::io;
use std::io::Read;

pub mod keyboard;
pub mod parse;

/// Modifier keys (Ctrl, Shift, Alt, Super). Used for key combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub super_: bool,
}

impl Modifiers {
    pub const fn new() -> Self {
        Modifiers {
            ctrl: false,
            shift: false,
            alt: false,
            super_: false,
        }
    }

    pub const fn ctrl() -> Self {
        Modifiers {
            ctrl: true,
            shift: false,
            alt: false,
            super_: false,
        }
    }

    pub const fn shift() -> Self {
        Modifiers {
            ctrl: false,
            shift: true,
            alt: false,
            super_: false,
        }
    }

    pub const fn alt() -> Self {
        Modifiers {
            ctrl: false,
            shift: false,
            alt: true,
            super_: false,
        }
    }

    pub const fn super_() -> Self {
        Modifiers {
            ctrl: false,
            shift: false,
            alt: false,
            super_: true,
        }
    }

    /// True when Ctrl, Alt, or Super is held. Those combinations are key chords (keybinder), not
    /// literal characters for the line buffer—even though the base key may be `Key::Letter`.
    pub const fn has_chord_modifier(&self) -> bool {
        self.ctrl || self.alt || self.super_
    }

    /// Canonical prefix for keybind string, e.g. "ctrl+shift+" (order: ctrl, shift, alt, super).
    pub fn to_prefix(&self) -> String {
        let mut parts = Vec::with_capacity(4);
        if self.ctrl {
            parts.push("ctrl");
        }
        if self.shift {
            parts.push("shift");
        }
        if self.alt {
            parts.push("alt");
        }
        if self.super_ {
            parts.push("super");
        }
        if parts.is_empty() {
            String::new()
        } else {
            parts.join("+") + "+"
        }
    }
}

/// A key event: base key plus optional modifiers.
#[derive(Debug, Clone)]
pub struct KeyEvent<'a> {
    pub modifiers: Modifiers,
    pub key: Key<'a>,
}

impl<'a> KeyEvent<'a> {
    pub const fn new(modifiers: Modifiers, key: Key<'a>) -> Self {
        KeyEvent { modifiers, key }
    }
}

// TODO: Need to add mouse events
#[derive(Debug, Clone, PartialEq)]
pub enum Key<'a> {
    Letter(char),
    Arrow(&'a str),
    Enter,
    Backspace,
    Tab,
    Esc,
    Delete,
    Unknown,
    /// Function key F1..=F12 (value 1..=12).
    F(u8),
    Home,
    End,
}

pub struct Buffer {
    buffer: String,
    /// Cursor position as byte offset (UTF-8 safe).
    pointer: usize,
    /// Length of buffer in bytes.
    size: usize,
}

impl Buffer {
    /// Move cursor backward by one codepoint (previous UTF-8 character start).
    fn handle_left(&mut self) {
        if self.pointer == 0 {
            return;
        }
        let bytes = self.buffer.as_bytes();
        let mut i = self.pointer;
        while i > 0 && (bytes[i - 1] & 0xC0) == 0x80 {
            i -= 1;
        }
        i = i.saturating_sub(1);
        self.pointer = i;
    }

    /// Move cursor forward by one codepoint.
    fn handle_right(&mut self) {
        if self.pointer >= self.size {
            return;
        }
        if let Some(c) = self.buffer[self.pointer..].chars().next() {
            let step = c.len_utf8();
            if self.pointer + step <= self.size {
                self.pointer += step;
            }
        }
    }

    /// Insert character at cursor; pointer and size are byte offsets.
    fn push(&mut self, ch: char) {
        let left = &self.buffer[..self.pointer];
        let right = &self.buffer[self.pointer..];
        self.buffer = format!("{}{}{}", left, ch, right);
        let n = ch.len_utf8();
        self.pointer += n;
        self.size += n;
    }

    /// Delete the codepoint before the cursor (backward delete).
    fn backspace(&mut self) {
        if self.pointer == 0 {
            return;
        }
        let bytes = self.buffer.as_bytes();
        let mut prev_start = self.pointer;
        while prev_start > 0 && (bytes[prev_start - 1] & 0xC0) == 0x80 {
            prev_start -= 1;
        }
        if prev_start == 0 {
            return;
        }
        prev_start -= 1;
        let char_len = self.pointer - prev_start;
        self.buffer = format!(
            "{}{}",
            &self.buffer[..prev_start],
            &self.buffer[self.pointer..]
        );
        self.pointer = prev_start;
        self.size -= char_len;
    }

    /// Delete the codepoint at the cursor (forward delete).
    fn delete(&mut self) {
        if self.pointer >= self.size {
            return;
        }
        let char_len = self.buffer[self.pointer..]
            .chars()
            .next()
            .map(|c| c.len_utf8())
            .unwrap_or(1);
        self.buffer = format!(
            "{}{}",
            &self.buffer[..self.pointer],
            &self.buffer[self.pointer + char_len..]
        );
        self.size -= char_len;
    }

    pub const fn new() -> Buffer {
        Buffer {
            buffer: String::new(),
            pointer: 0,
            size: 0,
        }
    }

    fn get_buffer(&mut self) -> String {
        self.buffer.clone()
    }

    fn clear(&mut self) {
        self.buffer.clear();
        self.pointer = 0;
        self.size = 0;
    }

    fn pass(&mut self, key: KeyEvent) {
        keybinder::handle_and_call(key);
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

static mut BUFFER: Buffer = Buffer::new();

// Helper function to safely access the mutable static using raw pointers
#[inline]
unsafe fn with_buffer<F, R>(f: F) -> R
where
    F: FnOnce(&mut Buffer) -> R,
{
    let buffer_ptr: *mut Buffer = core::ptr::addr_of_mut!(BUFFER);
    unsafe { f(&mut *buffer_ptr) }
}

pub fn map_activity(key: KeyEvent) {
    keyboard::key_activity_mapper(key);
}

pub fn get_buffer() -> String {
    unsafe { with_buffer(|buffer| buffer.get_buffer()) }
}

pub fn clear_buffer() {
    unsafe { with_buffer(|buffer| buffer.clear()) }
}

/// Input buffer for reading keys. UTF-8 and CSI sequences may need multiple bytes.
const INPUT_BUF_CAP: usize = 32;
static mut INPUT_BUF: [u8; INPUT_BUF_CAP] = [0; INPUT_BUF_CAP];
static mut INPUT_LEN: usize = 0;

/// Read the next key from stdin. Accumulates bytes until a full key (control, CSI, or UTF-8
/// codepoint) is available. Uses only std. Handles pending Alt (ESC then key).
static mut PENDING_ALT: bool = false;

/// Read the next key from stdin. Accumulates bytes until a full key (control, CSI, or UTF-8
/// codepoint) is available. Uses only std.
pub fn watch() -> KeyEvent<'static> {
    const READ_SIZE: usize = 8;
    let mut read_buf = [0u8; READ_SIZE];
    let stdin = io::stdin();

    loop {
        unsafe {
            let buf_ptr: *mut [u8; INPUT_BUF_CAP] = core::ptr::addr_of_mut!(INPUT_BUF);
            let len_ptr: *mut usize = core::ptr::addr_of_mut!(INPUT_LEN);
            let buf = &mut *buf_ptr;
            let len = &mut *len_ptr;

            match parse::parse_key(&buf[..*len]) {
                parse::ParseResult::Consumed(mut key_event, n) => {
                    buf.copy_within(n..*len, 0);
                    *len -= n;
                    if PENDING_ALT {
                        PENDING_ALT = false;
                        key_event.modifiers = Modifiers {
                            alt: true,
                            ..key_event.modifiers
                        };
                    }
                    if key_event.key == Key::Esc
                        && !key_event.modifiers.ctrl
                        && !key_event.modifiers.shift
                        && !key_event.modifiers.super_
                    {
                        PENDING_ALT = true;
                        continue;
                    }
                    return key_event;
                }
                parse::ParseResult::NeedMore => {}
            }

            let n = stdin
                .lock()
                .read(&mut read_buf)
                .expect("Failed to read input");
            if n == 0 {
                return KeyEvent::new(Modifiers::new(), Key::Unknown);
            }
            if *len + n <= INPUT_BUF_CAP {
                buf[*len..*len + n].copy_from_slice(&read_buf[..n]);
                *len += n;
            } else {
                *len = 0;
                return KeyEvent::new(Modifiers::new(), Key::Unknown);
            }
        }
    }
}

#[cfg(test)]
mod modifiers_tests {
    use super::Modifiers;

    #[test]
    fn has_chord_modifier_detects_ctrl_alt_super() {
        assert!(!Modifiers::new().has_chord_modifier());
        assert!(!Modifiers::shift().has_chord_modifier());
        assert!(Modifiers::ctrl().has_chord_modifier());
        assert!(Modifiers::alt().has_chord_modifier());
        assert!(Modifiers::super_().has_chord_modifier());
    }
}

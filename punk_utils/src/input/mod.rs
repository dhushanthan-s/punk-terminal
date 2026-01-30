use crate::config::keybinder;
use std::io;
use std::io::Read;

pub mod keyboard;

// TODO: Need to add mouse events and functional keys
#[derive(Debug, Clone)]
pub enum Key<'a> {
    Letter(char),
    Ctrl(char),
    Arrow(&'a str),
    Backspace,
    Tab,
    Esc,
    Delete,
    Unknown,
}

pub struct Buffer {
    buffer: String,
    pointer: u32,
    size: u32,
}

impl Buffer {
    fn handle_right(&mut self) {
        if self.pointer < self.size {
            self.pointer += 1;
        }
    }

    fn handle_left(&mut self) {
        if self.pointer > 0 {
            self.pointer -= 1;
        }
    }

    fn push(&mut self, ch: char) {
        let mut left_str: String = self.buffer[..self.pointer as usize].to_string();
        let right_str: String =
            self.buffer[self.pointer as usize..(self.size) as usize].to_string();

        left_str.push(ch);
        left_str.push_str(right_str.as_str());
        self.buffer = left_str;
        self.pointer += 1;
        self.size += 1;
    }

    fn backspace(&mut self) {
        if self.pointer == 0 {
            return;
        }
        let mut left_str: String = self.buffer[..(self.pointer - 1) as usize].to_string();
        let right_str: String =
            self.buffer[self.pointer as usize..(self.size) as usize].to_string();

        left_str.push_str(right_str.as_str());
        self.buffer = left_str;
        if self.pointer > 0 {
            self.size -= 1;
            self.pointer -= 1;
        }
    }

    fn delete(&mut self) {
        if self.pointer >= self.size {
            return;
        }
        let mut left_str: String = self.buffer[..self.pointer as usize].to_string();
        let right_str: String =
            self.buffer[(self.pointer + 1) as usize..(self.size) as usize].to_string();

        left_str.push_str(right_str.as_str());
        self.buffer = left_str;
        self.size -= 1;
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

    fn pass(&mut self, key: Key) {
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

pub fn map_activity(key: Key) {
    keyboard::key_activity_mapper(key);
}

pub fn get_buffer() -> String {
    unsafe { with_buffer(|buffer| buffer.get_buffer()) }
}

static mut INPUT: [u8; 5] = [0; 5];

pub fn watch() -> Key<'static> {
    let stdin = io::stdin();
    unsafe {
        let input_ptr: *mut [u8; 5] = core::ptr::addr_of_mut!(INPUT);
        let input = &mut *input_ptr;
        let input_len = stdin.lock().read(input).expect("Failed to read input");
        let key = &input[0..input_len];
        keyboard::matcher(key)
    }
}

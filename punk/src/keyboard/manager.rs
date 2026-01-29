use crate::enums::Key;
use crate::keyboard::binder;

pub struct Buffer {
    buffer: String,
    pointer: u32,
    size: u32,
}

#[cfg(target_os = "linux")]
pub fn KEY_ACTIVITY_MAPPER(key: Key) {
    unsafe {
        with_buffer(|buffer| {
            match key {
                Key::Arrow("left") => buffer.handle_left(),
                Key::Arrow("right") => buffer.handle_right(),
                Key::Backspace => buffer.delete(),
                Key::Delete => buffer.backspace(),
                Key::Letter(' ') => buffer.push(' '),
                Key::Letter('!') => buffer.push('!'),
                Key::Letter('"') => buffer.push('"'),
                Key::Letter('#') => buffer.push('#'),
                Key::Letter('$') => buffer.push('$'),
                Key::Letter('%') => buffer.push('%'),
                Key::Letter('&') => buffer.push('&'),
                Key::Letter('\'') => buffer.push('\''),
                Key::Letter('(') => buffer.push('('),
                Key::Letter(')') => buffer.push(')'),
                Key::Letter('*') => buffer.push('*'),
                Key::Letter('+') => buffer.push('+'),
                Key::Letter(',') => buffer.push(','),
                Key::Letter('-') => buffer.push('-'),
                Key::Letter('.') => buffer.push('.'),
                Key::Letter('/') => buffer.push('/'),
                Key::Letter('0') => buffer.push('0'),
                Key::Letter('1') => buffer.push('1'),
                Key::Letter('2') => buffer.push('2'),
                Key::Letter('3') => buffer.push('3'),
                Key::Letter('4') => buffer.push('4'),
                Key::Letter('5') => buffer.push('5'),
                Key::Letter('6') => buffer.push('6'),
                Key::Letter('7') => buffer.push('7'),
                Key::Letter('8') => buffer.push('8'),
                Key::Letter('9') => buffer.push('9'),
                Key::Letter(':') => buffer.push(':'),
                Key::Letter(';') => buffer.push(';'),
                Key::Letter('<') => buffer.push('<'),
                Key::Letter('=') => buffer.push('='),
                Key::Letter('>') => buffer.push('>'),
                Key::Letter('?') => buffer.push('?'),
                Key::Letter('@') => buffer.push('@'),
                Key::Letter('A') => buffer.push('A'),
                Key::Letter('B') => buffer.push('B'),
                Key::Letter('C') => buffer.push('C'),
                Key::Letter('D') => buffer.push('D'),
                Key::Letter('E') => buffer.push('E'),
                Key::Letter('F') => buffer.push('F'),
                Key::Letter('G') => buffer.push('G'),
                Key::Letter('H') => buffer.push('H'),
                Key::Letter('I') => buffer.push('I'),
                Key::Letter('J') => buffer.push('J'),
                Key::Letter('K') => buffer.push('K'),
                Key::Letter('L') => buffer.push('L'),
                Key::Letter('M') => buffer.push('M'),
                Key::Letter('N') => buffer.push('N'),
                Key::Letter('O') => buffer.push('O'),
                Key::Letter('P') => buffer.push('P'),
                Key::Letter('Q') => buffer.push('Q'),
                Key::Letter('R') => buffer.push('R'),
                Key::Letter('S') => buffer.push('S'),
                Key::Letter('T') => buffer.push('T'),
                Key::Letter('U') => buffer.push('U'),
                Key::Letter('V') => buffer.push('V'),
                Key::Letter('W') => buffer.push('W'),
                Key::Letter('X') => buffer.push('X'),
                Key::Letter('Y') => buffer.push('Y'),
                Key::Letter('Z') => buffer.push('Z'),
                Key::Letter('[') => buffer.push('['),
                Key::Letter('\\') => buffer.push('\\'),
                Key::Letter(']') => buffer.push(']'),
                Key::Letter('^') => buffer.push('^'),
                Key::Letter('_') => buffer.push('_'),
                Key::Letter('`') => buffer.push('`'),
                Key::Letter('a') => buffer.push('a'),
                Key::Letter('b') => buffer.push('b'),
                Key::Letter('c') => buffer.push('c'),
                Key::Letter('d') => buffer.push('d'),
                Key::Letter('e') => buffer.push('e'),
                Key::Letter('f') => buffer.push('f'),
                Key::Letter('g') => buffer.push('g'),
                Key::Letter('h') => buffer.push('h'),
                Key::Letter('i') => buffer.push('i'),
                Key::Letter('j') => buffer.push('j'),
                Key::Letter('k') => buffer.push('k'),
                Key::Letter('l') => buffer.push('l'),
                Key::Letter('m') => buffer.push('m'),
                Key::Letter('n') => buffer.push('n'),
                Key::Letter('o') => buffer.push('o'),
                Key::Letter('p') => buffer.push('p'),
                Key::Letter('q') => buffer.push('q'),
                Key::Letter('r') => buffer.push('r'),
                Key::Letter('s') => buffer.push('s'),
                Key::Letter('t') => buffer.push('t'),
                Key::Letter('u') => buffer.push('u'),
                Key::Letter('v') => buffer.push('v'),
                Key::Letter('w') => buffer.push('w'),
                Key::Letter('x') => buffer.push('x'),
                Key::Letter('y') => buffer.push('y'),
                Key::Letter('z') => buffer.push('z'),
                Key::Letter('{') => buffer.push('{'),
                Key::Letter('|') => buffer.push('|'),
                Key::Letter('}') => buffer.push('}'),
                Key::Letter('~') => buffer.push('~'),
                _ => buffer.pass(key),
            }
        });
    }
}
#[cfg(target_os = "macos")]
pub fn KEY_ACTIVITY_MAPPER(key: Key) {
    unsafe {
        with_buffer(|buffer| {
            match key {
                Key::Arrow("left") => buffer.handle_left(),
                Key::Arrow("right") => buffer.handle_right(),
                Key::Backspace => buffer.delete(),
                Key::Delete => buffer.backspace(),
                Key::Letter(' ') => buffer.push(' '),
                Key::Letter('!') => buffer.push('!'),
                Key::Letter('"') => buffer.push('"'),
                Key::Letter('#') => buffer.push('#'),
                Key::Letter('$') => buffer.push('$'),
                Key::Letter('%') => buffer.push('%'),
                Key::Letter('&') => buffer.push('&'),
                Key::Letter('\'') => buffer.push('\''),
                Key::Letter('(') => buffer.push('('),
                Key::Letter(')') => buffer.push(')'),
                Key::Letter('*') => buffer.push('*'),
                Key::Letter('+') => buffer.push('+'),
                Key::Letter(',') => buffer.push(','),
                Key::Letter('-') => buffer.push('-'),
                Key::Letter('.') => buffer.push('.'),
                Key::Letter('/') => buffer.push('/'),
                Key::Letter('0') => buffer.push('0'),
                Key::Letter('1') => buffer.push('1'),
                Key::Letter('2') => buffer.push('2'),
                Key::Letter('3') => buffer.push('3'),
                Key::Letter('4') => buffer.push('4'),
                Key::Letter('5') => buffer.push('5'),
                Key::Letter('6') => buffer.push('6'),
                Key::Letter('7') => buffer.push('7'),
                Key::Letter('8') => buffer.push('8'),
                Key::Letter('9') => buffer.push('9'),
                Key::Letter(':') => buffer.push(':'),
                Key::Letter(';') => buffer.push(';'),
                Key::Letter('<') => buffer.push('<'),
                Key::Letter('=') => buffer.push('='),
                Key::Letter('>') => buffer.push('>'),
                Key::Letter('?') => buffer.push('?'),
                Key::Letter('@') => buffer.push('@'),
                Key::Letter('A') => buffer.push('A'),
                Key::Letter('B') => buffer.push('B'),
                Key::Letter('C') => buffer.push('C'),
                Key::Letter('D') => buffer.push('D'),
                Key::Letter('E') => buffer.push('E'),
                Key::Letter('F') => buffer.push('F'),
                Key::Letter('G') => buffer.push('G'),
                Key::Letter('H') => buffer.push('H'),
                Key::Letter('I') => buffer.push('I'),
                Key::Letter('J') => buffer.push('J'),
                Key::Letter('K') => buffer.push('K'),
                Key::Letter('L') => buffer.push('L'),
                Key::Letter('M') => buffer.push('M'),
                Key::Letter('N') => buffer.push('N'),
                Key::Letter('O') => buffer.push('O'),
                Key::Letter('P') => buffer.push('P'),
                Key::Letter('Q') => buffer.push('Q'),
                Key::Letter('R') => buffer.push('R'),
                Key::Letter('S') => buffer.push('S'),
                Key::Letter('T') => buffer.push('T'),
                Key::Letter('U') => buffer.push('U'),
                Key::Letter('V') => buffer.push('V'),
                Key::Letter('W') => buffer.push('W'),
                Key::Letter('X') => buffer.push('X'),
                Key::Letter('Y') => buffer.push('Y'),
                Key::Letter('Z') => buffer.push('Z'),
                Key::Letter('[') => buffer.push('['),
                Key::Letter('\\') => buffer.push('\\'),
                Key::Letter(']') => buffer.push(']'),
                Key::Letter('^') => buffer.push('^'),
                Key::Letter('_') => buffer.push('_'),
                Key::Letter('`') => buffer.push('`'),
                Key::Letter('a') => buffer.push('a'),
                Key::Letter('b') => buffer.push('b'),
                Key::Letter('c') => buffer.push('c'),
                Key::Letter('d') => buffer.push('d'),
                Key::Letter('e') => buffer.push('e'),
                Key::Letter('f') => buffer.push('f'),
                Key::Letter('g') => buffer.push('g'),
                Key::Letter('h') => buffer.push('h'),
                Key::Letter('i') => buffer.push('i'),
                Key::Letter('j') => buffer.push('j'),
                Key::Letter('k') => buffer.push('k'),
                Key::Letter('l') => buffer.push('l'),
                Key::Letter('m') => buffer.push('m'),
                Key::Letter('n') => buffer.push('n'),
                Key::Letter('o') => buffer.push('o'),
                Key::Letter('p') => buffer.push('p'),
                Key::Letter('q') => buffer.push('q'),
                Key::Letter('r') => buffer.push('r'),
                Key::Letter('s') => buffer.push('s'),
                Key::Letter('t') => buffer.push('t'),
                Key::Letter('u') => buffer.push('u'),
                Key::Letter('v') => buffer.push('v'),
                Key::Letter('w') => buffer.push('w'),
                Key::Letter('x') => buffer.push('x'),
                Key::Letter('y') => buffer.push('y'),
                Key::Letter('z') => buffer.push('z'),
                Key::Letter('{') => buffer.push('{'),
                Key::Letter('|') => buffer.push('|'),
                Key::Letter('}') => buffer.push('}'),
                Key::Letter('~') => buffer.push('~'),
                _ => buffer.pass(key),
            }
        });
    }
}
#[cfg(target_os = "windows")]
pub fn KEY_ACTIVITY_MAPPER(key: Key) {
    unsafe {
        with_buffer(|buffer| {
            match key {
                Key::Arrow("left") => buffer.handle_left(),
                Key::Arrow("right") => buffer.handle_right(),
                Key::Backspace => buffer.delete(),
                Key::Delete => buffer.backspace(),
                Key::Letter(' ') => buffer.push(' '),
                Key::Letter('!') => buffer.push('!'),
                Key::Letter('"') => buffer.push('"'),
                Key::Letter('#') => buffer.push('#'),
                Key::Letter('$') => buffer.push('$'),
                Key::Letter('%') => buffer.push('%'),
                Key::Letter('&') => buffer.push('&'),
                Key::Letter('\'') => buffer.push('\''),
                Key::Letter('(') => buffer.push('('),
                Key::Letter(')') => buffer.push(')'),
                Key::Letter('*') => buffer.push('*'),
                Key::Letter('+') => buffer.push('+'),
                Key::Letter(',') => buffer.push(','),
                Key::Letter('-') => buffer.push('-'),
                Key::Letter('.') => buffer.push('.'),
                Key::Letter('/') => buffer.push('/'),
                Key::Letter('0') => buffer.push('0'),
                Key::Letter('1') => buffer.push('1'),
                Key::Letter('2') => buffer.push('2'),
                Key::Letter('3') => buffer.push('3'),
                Key::Letter('4') => buffer.push('4'),
                Key::Letter('5') => buffer.push('5'),
                Key::Letter('6') => buffer.push('6'),
                Key::Letter('7') => buffer.push('7'),
                Key::Letter('8') => buffer.push('8'),
                Key::Letter('9') => buffer.push('9'),
                Key::Letter(':') => buffer.push(':'),
                Key::Letter(';') => buffer.push(';'),
                Key::Letter('<') => buffer.push('<'),
                Key::Letter('=') => buffer.push('='),
                Key::Letter('>') => buffer.push('>'),
                Key::Letter('?') => buffer.push('?'),
                Key::Letter('@') => buffer.push('@'),
                Key::Letter('A') => buffer.push('A'),
                Key::Letter('B') => buffer.push('B'),
                Key::Letter('C') => buffer.push('C'),
                Key::Letter('D') => buffer.push('D'),
                Key::Letter('E') => buffer.push('E'),
                Key::Letter('F') => buffer.push('F'),
                Key::Letter('G') => buffer.push('G'),
                Key::Letter('H') => buffer.push('H'),
                Key::Letter('I') => buffer.push('I'),
                Key::Letter('J') => buffer.push('J'),
                Key::Letter('K') => buffer.push('K'),
                Key::Letter('L') => buffer.push('L'),
                Key::Letter('M') => buffer.push('M'),
                Key::Letter('N') => buffer.push('N'),
                Key::Letter('O') => buffer.push('O'),
                Key::Letter('P') => buffer.push('P'),
                Key::Letter('Q') => buffer.push('Q'),
                Key::Letter('R') => buffer.push('R'),
                Key::Letter('S') => buffer.push('S'),
                Key::Letter('T') => buffer.push('T'),
                Key::Letter('U') => buffer.push('U'),
                Key::Letter('V') => buffer.push('V'),
                Key::Letter('W') => buffer.push('W'),
                Key::Letter('X') => buffer.push('X'),
                Key::Letter('Y') => buffer.push('Y'),
                Key::Letter('Z') => buffer.push('Z'),
                Key::Letter('[') => buffer.push('['),
                Key::Letter('\\') => buffer.push('\\'),
                Key::Letter(']') => buffer.push(']'),
                Key::Letter('^') => buffer.push('^'),
                Key::Letter('_') => buffer.push('_'),
                Key::Letter('`') => buffer.push('`'),
                Key::Letter('a') => buffer.push('a'),
                Key::Letter('b') => buffer.push('b'),
                Key::Letter('c') => buffer.push('c'),
                Key::Letter('d') => buffer.push('d'),
                Key::Letter('e') => buffer.push('e'),
                Key::Letter('f') => buffer.push('f'),
                Key::Letter('g') => buffer.push('g'),
                Key::Letter('h') => buffer.push('h'),
                Key::Letter('i') => buffer.push('i'),
                Key::Letter('j') => buffer.push('j'),
                Key::Letter('k') => buffer.push('k'),
                Key::Letter('l') => buffer.push('l'),
                Key::Letter('m') => buffer.push('m'),
                Key::Letter('n') => buffer.push('n'),
                Key::Letter('o') => buffer.push('o'),
                Key::Letter('p') => buffer.push('p'),
                Key::Letter('q') => buffer.push('q'),
                Key::Letter('r') => buffer.push('r'),
                Key::Letter('s') => buffer.push('s'),
                Key::Letter('t') => buffer.push('t'),
                Key::Letter('u') => buffer.push('u'),
                Key::Letter('v') => buffer.push('v'),
                Key::Letter('w') => buffer.push('w'),
                Key::Letter('x') => buffer.push('x'),
                Key::Letter('y') => buffer.push('y'),
                Key::Letter('z') => buffer.push('z'),
                Key::Letter('{') => buffer.push('{'),
                Key::Letter('|') => buffer.push('|'),
                Key::Letter('}') => buffer.push('}'),
                Key::Letter('~') => buffer.push('~'),
                _ => buffer.pass(key),
            }
        });
    }
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
            return ();
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
            return ();
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
        return self.buffer.clone();
    }

    fn pass(&mut self, key: Key) {
        binder::handle_and_call(key);
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
    unsafe {
        f(&mut *buffer_ptr)
    }
}

pub fn map_activity(key: Key) {
    KEY_ACTIVITY_MAPPER(key);
}

pub fn get_buffer() -> String {
    unsafe {
        with_buffer(|buffer| buffer.get_buffer())
    }
}

pub fn windows_key_vs_activity_mapper(key: Key) {
    unsafe {
        with_buffer(|buffer| {
            match key {
                Key::Arrow("left") => buffer.handle_left(),
                Key::Arrow("right") => buffer.handle_right(),
                Key::Backspace => buffer.delete(),
                Key::Delete => buffer.backspace(),
                Key::Letter(' ') => buffer.push(' '),
                Key::Letter('!') => buffer.push('!'),
                Key::Letter('"') => buffer.push('"'),
                Key::Letter('#') => buffer.push('#'),
                Key::Letter('$') => buffer.push('$'),
                Key::Letter('%') => buffer.push('%'),
                Key::Letter('&') => buffer.push('&'),
                Key::Letter('\'') => buffer.push('\''),
                Key::Letter('(') => buffer.push('('),
                Key::Letter(')') => buffer.push(')'),
                Key::Letter('*') => buffer.push('*'),
                Key::Letter('+') => buffer.push('+'),
                Key::Letter(',') => buffer.push(','),
                Key::Letter('-') => buffer.push('-'),
                Key::Letter('.') => buffer.push('.'),
                Key::Letter('/') => buffer.push('/'),
                Key::Letter('0') => buffer.push('0'),
                Key::Letter('1') => buffer.push('1'),
                Key::Letter('2') => buffer.push('2'),
                Key::Letter('3') => buffer.push('3'),
                Key::Letter('4') => buffer.push('4'),
                Key::Letter('5') => buffer.push('5'),
                Key::Letter('6') => buffer.push('6'),
                Key::Letter('7') => buffer.push('7'),
                Key::Letter('8') => buffer.push('8'),
                Key::Letter('9') => buffer.push('9'),
                Key::Letter(':') => buffer.push(':'),
                Key::Letter(';') => buffer.push(';'),
                Key::Letter('<') => buffer.push('<'),
                Key::Letter('=') => buffer.push('='),
                Key::Letter('>') => buffer.push('>'),
                Key::Letter('?') => buffer.push('?'),
                Key::Letter('@') => buffer.push('@'),
                Key::Letter('A') => buffer.push('A'),
                Key::Letter('B') => buffer.push('B'),
                Key::Letter('C') => buffer.push('C'),
                Key::Letter('D') => buffer.push('D'),
                Key::Letter('E') => buffer.push('E'),
                Key::Letter('F') => buffer.push('F'),
                Key::Letter('G') => buffer.push('G'),
                Key::Letter('H') => buffer.push('H'),
                Key::Letter('I') => buffer.push('I'),
                Key::Letter('J') => buffer.push('J'),
                Key::Letter('K') => buffer.push('K'),
                Key::Letter('L') => buffer.push('L'),
                Key::Letter('M') => buffer.push('M'),
                Key::Letter('N') => buffer.push('N'),
                Key::Letter('O') => buffer.push('O'),
                Key::Letter('P') => buffer.push('P'),
                Key::Letter('Q') => buffer.push('Q'),
                Key::Letter('R') => buffer.push('R'),
                Key::Letter('S') => buffer.push('S'),
                Key::Letter('T') => buffer.push('T'),
                Key::Letter('U') => buffer.push('U'),
                Key::Letter('V') => buffer.push('V'),
                Key::Letter('W') => buffer.push('W'),
                Key::Letter('X') => buffer.push('X'),
                Key::Letter('Y') => buffer.push('Y'),
                Key::Letter('Z') => buffer.push('Z'),
                Key::Letter('[') => buffer.push('['),
                Key::Letter('\\') => buffer.push('\\'),
                Key::Letter(']') => buffer.push(']'),
                Key::Letter('^') => buffer.push('^'),
                Key::Letter('_') => buffer.push('_'),
                Key::Letter('`') => buffer.push('`'),
                Key::Letter('a') => buffer.push('a'),
                Key::Letter('b') => buffer.push('b'),
                Key::Letter('c') => buffer.push('c'),
                Key::Letter('d') => buffer.push('d'),
                Key::Letter('e') => buffer.push('e'),
                Key::Letter('f') => buffer.push('f'),
                Key::Letter('g') => buffer.push('g'),
                Key::Letter('h') => buffer.push('h'),
                Key::Letter('i') => buffer.push('i'),
                Key::Letter('j') => buffer.push('j'),
                Key::Letter('k') => buffer.push('k'),
                Key::Letter('l') => buffer.push('l'),
                Key::Letter('m') => buffer.push('m'),
                Key::Letter('n') => buffer.push('n'),
                Key::Letter('o') => buffer.push('o'),
                Key::Letter('p') => buffer.push('p'),
                Key::Letter('q') => buffer.push('q'),
                Key::Letter('r') => buffer.push('r'),
                Key::Letter('s') => buffer.push('s'),
                Key::Letter('t') => buffer.push('t'),
                Key::Letter('u') => buffer.push('u'),
                Key::Letter('v') => buffer.push('v'),
                Key::Letter('w') => buffer.push('w'),
                Key::Letter('x') => buffer.push('x'),
                Key::Letter('y') => buffer.push('y'),
                Key::Letter('z') => buffer.push('z'),
                Key::Letter('{') => buffer.push('{'),
                Key::Letter('|') => buffer.push('|'),
                Key::Letter('}') => buffer.push('}'),
                Key::Letter('~') => buffer.push('~'),
                _ => buffer.pass(key),
            }
        });
    }
}

pub fn macos_key_vs_activity_mapper(key: Key) {
    unsafe {
        with_buffer(|buffer| {
            match key {
                Key::Arrow("left") => buffer.handle_left(),
                Key::Arrow("right") => buffer.handle_right(),
                Key::Backspace => buffer.delete(),
                Key::Delete => buffer.backspace(),
                Key::Letter(' ') => buffer.push(' '),
                Key::Letter('!') => buffer.push('!'),
                Key::Letter('"') => buffer.push('"'),
                Key::Letter('#') => buffer.push('#'),
                Key::Letter('$') => buffer.push('$'),
                Key::Letter('%') => buffer.push('%'),
                Key::Letter('&') => buffer.push('&'),
                Key::Letter('\'') => buffer.push('\''),
                Key::Letter('(') => buffer.push('('),
                Key::Letter(')') => buffer.push(')'),
                Key::Letter('*') => buffer.push('*'),
                Key::Letter('+') => buffer.push('+'),
                Key::Letter(',') => buffer.push(','),
                Key::Letter('-') => buffer.push('-'),
                Key::Letter('.') => buffer.push('.'),
                Key::Letter('/') => buffer.push('/'),
                Key::Letter('0') => buffer.push('0'),
                Key::Letter('1') => buffer.push('1'),
                Key::Letter('2') => buffer.push('2'),
                Key::Letter('3') => buffer.push('3'),
                Key::Letter('4') => buffer.push('4'),
                Key::Letter('5') => buffer.push('5'),
                Key::Letter('6') => buffer.push('6'),
                Key::Letter('7') => buffer.push('7'),
                Key::Letter('8') => buffer.push('8'),
                Key::Letter('9') => buffer.push('9'),
                Key::Letter(':') => buffer.push(':'),
                Key::Letter(';') => buffer.push(';'),
                Key::Letter('<') => buffer.push('<'),
                Key::Letter('=') => buffer.push('='),
                Key::Letter('>') => buffer.push('>'),
                Key::Letter('?') => buffer.push('?'),
                Key::Letter('@') => buffer.push('@'),
                Key::Letter('A') => buffer.push('A'),
                Key::Letter('B') => buffer.push('B'),
                Key::Letter('C') => buffer.push('C'),
                Key::Letter('D') => buffer.push('D'),
                Key::Letter('E') => buffer.push('E'),
                Key::Letter('F') => buffer.push('F'),
                Key::Letter('G') => buffer.push('G'),
                Key::Letter('H') => buffer.push('H'),
                Key::Letter('I') => buffer.push('I'),
                Key::Letter('J') => buffer.push('J'),
                Key::Letter('K') => buffer.push('K'),
                Key::Letter('L') => buffer.push('L'),
                Key::Letter('M') => buffer.push('M'),
                Key::Letter('N') => buffer.push('N'),
                Key::Letter('O') => buffer.push('O'),
                Key::Letter('P') => buffer.push('P'),
                Key::Letter('Q') => buffer.push('Q'),
                Key::Letter('R') => buffer.push('R'),
                Key::Letter('S') => buffer.push('S'),
                Key::Letter('T') => buffer.push('T'),
                Key::Letter('U') => buffer.push('U'),
                Key::Letter('V') => buffer.push('V'),
                Key::Letter('W') => buffer.push('W'),
                Key::Letter('X') => buffer.push('X'),
                Key::Letter('Y') => buffer.push('Y'),
                Key::Letter('Z') => buffer.push('Z'),
                Key::Letter('[') => buffer.push('['),
                Key::Letter('\\') => buffer.push('\\'),
                Key::Letter(']') => buffer.push(']'),
                Key::Letter('^') => buffer.push('^'),
                Key::Letter('_') => buffer.push('_'),
                Key::Letter('`') => buffer.push('`'),
                Key::Letter('a') => buffer.push('a'),
                Key::Letter('b') => buffer.push('b'),
                Key::Letter('c') => buffer.push('c'),
                Key::Letter('d') => buffer.push('d'),
                Key::Letter('e') => buffer.push('e'),
                Key::Letter('f') => buffer.push('f'),
                Key::Letter('g') => buffer.push('g'),
                Key::Letter('h') => buffer.push('h'),
                Key::Letter('i') => buffer.push('i'),
                Key::Letter('j') => buffer.push('j'),
                Key::Letter('k') => buffer.push('k'),
                Key::Letter('l') => buffer.push('l'),
                Key::Letter('m') => buffer.push('m'),
                Key::Letter('n') => buffer.push('n'),
                Key::Letter('o') => buffer.push('o'),
                Key::Letter('p') => buffer.push('p'),
                Key::Letter('q') => buffer.push('q'),
                Key::Letter('r') => buffer.push('r'),
                Key::Letter('s') => buffer.push('s'),
                Key::Letter('t') => buffer.push('t'),
                Key::Letter('u') => buffer.push('u'),
                Key::Letter('v') => buffer.push('v'),
                Key::Letter('w') => buffer.push('w'),
                Key::Letter('x') => buffer.push('x'),
                Key::Letter('y') => buffer.push('y'),
                Key::Letter('z') => buffer.push('z'),
                Key::Letter('{') => buffer.push('{'),
                Key::Letter('|') => buffer.push('|'),
                Key::Letter('}') => buffer.push('}'),
                Key::Letter('~') => buffer.push('~'),
                _ => buffer.pass(key),
            }
        });
    }
}

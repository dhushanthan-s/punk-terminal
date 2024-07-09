use keyboard::binder;
use enums::Key;

pub struct Buffer {
    buffer: String,
    pointer: u32,
    size: u32,
}

#[cfg(target_os = "linux")]
pub fn KEY_ACTIVITY_MAPPER(key: Key)
{
    unsafe
    {
        match key   {
            Key::arrow("left") => BUFFER.handle_left(),
            Key::arrow("right") => BUFFER.handle_right(),
            Key::Backspace => BUFFER.delete(),
            Key::Delete => BUFFER.backspace(),
            Key::letter(' ') => BUFFER.push(' '),
            Key::letter('!') => BUFFER.push('!'),
            Key::letter('"') => BUFFER.push('"'),
            Key::letter('#') => BUFFER.push('#'),
            Key::letter('$') => BUFFER.push('$'),
            Key::letter('%') => BUFFER.push('%'),
            Key::letter('&') => BUFFER.push('&'),
            Key::letter('\'') => BUFFER.push('\''),
            Key::letter('(') => BUFFER.push('('),
            Key::letter(')') => BUFFER.push(')'),
            Key::letter('*') => BUFFER.push('*'),
            Key::letter('+') => BUFFER.push('+'),
            Key::letter(',') => BUFFER.push(','),
            Key::letter('-') => BUFFER.push('-'),
            Key::letter('.') => BUFFER.push('.'),
            Key::letter('/') => BUFFER.push('/'),
            Key::letter('0') => BUFFER.push('0'),
            Key::letter('1') => BUFFER.push('1'),
            Key::letter('2') => BUFFER.push('2'),
            Key::letter('3') => BUFFER.push('3'),
            Key::letter('4') => BUFFER.push('4'),
            Key::letter('5') => BUFFER.push('5'),
            Key::letter('6') => BUFFER.push('6'),
            Key::letter('7') => BUFFER.push('7'),
            Key::letter('8') => BUFFER.push('8'),
            Key::letter('9') => BUFFER.push('9'),
            Key::letter(':') => BUFFER.push(':'),
            Key::letter(';') => BUFFER.push(';'),
            Key::letter('<') => BUFFER.push('<'),
            Key::letter('=') => BUFFER.push('='),
            Key::letter('>') => BUFFER.push('>'),
            Key::letter('?') => BUFFER.push('?'),
            Key::letter('@') => BUFFER.push('@'),
            Key::letter('A') => BUFFER.push('A'),
            Key::letter('B') => BUFFER.push('B'),
            Key::letter('C') => BUFFER.push('C'),
            Key::letter('D') => BUFFER.push('D'),
            Key::letter('E') => BUFFER.push('E'),
            Key::letter('F') => BUFFER.push('F'),
            Key::letter('G') => BUFFER.push('G'),
            Key::letter('H') => BUFFER.push('H'),
            Key::letter('I') => BUFFER.push('I'),
            Key::letter('J') => BUFFER.push('J'),
            Key::letter('K') => BUFFER.push('K'),
            Key::letter('L') => BUFFER.push('L'),
            Key::letter('M') => BUFFER.push('M'),
            Key::letter('N') => BUFFER.push('N'),
            Key::letter('O') => BUFFER.push('O'),
            Key::letter('P') => BUFFER.push('P'),
            Key::letter('Q') => BUFFER.push('Q'),
            Key::letter('R') => BUFFER.push('R'),
            Key::letter('S') => BUFFER.push('S'),
            Key::letter('T') => BUFFER.push('T'),
            Key::letter('U') => BUFFER.push('U'),
            Key::letter('V') => BUFFER.push('V'),
            Key::letter('W') => BUFFER.push('W'),
            Key::letter('X') => BUFFER.push('X'),
            Key::letter('Y') => BUFFER.push('Y'),
            Key::letter('Z') => BUFFER.push('Z'),
            Key::letter('[') => BUFFER.push('['),
            Key::letter('\\') => BUFFER.push('\\'),
            Key::letter(']') => BUFFER.push(']'),
            Key::letter('^') => BUFFER.push('^'),
            Key::letter('_') => BUFFER.push('_'),
            Key::letter('`') => BUFFER.push('`'),
            Key::letter('a') => BUFFER.push('a'),
            Key::letter('b') => BUFFER.push('b'),
            Key::letter('c') => BUFFER.push('c'),
            Key::letter('d') => BUFFER.push('d'),
            Key::letter('e') => BUFFER.push('e'),
            Key::letter('f') => BUFFER.push('f'),
            Key::letter('g') => BUFFER.push('g'),
            Key::letter('h') => BUFFER.push('h'),
            Key::letter('i') => BUFFER.push('i'),
            Key::letter('j') => BUFFER.push('j'),
            Key::letter('k') => BUFFER.push('k'),
            Key::letter('l') => BUFFER.push('l'),
            Key::letter('m') => BUFFER.push('m'),
            Key::letter('n') => BUFFER.push('n'),
            Key::letter('o') => BUFFER.push('o'),
            Key::letter('p') => BUFFER.push('p'),
            Key::letter('q') => BUFFER.push('q'),
            Key::letter('r') => BUFFER.push('r'),
            Key::letter('s') => BUFFER.push('s'),
            Key::letter('t') => BUFFER.push('t'),
            Key::letter('u') => BUFFER.push('u'),
            Key::letter('v') => BUFFER.push('v'),
            Key::letter('w') => BUFFER.push('w'),
            Key::letter('x') => BUFFER.push('x'),
            Key::letter('y') => BUFFER.push('y'),
            Key::letter('z') => BUFFER.push('z'),
            Key::letter('{') => BUFFER.push('{'),
            Key::letter('|') => BUFFER.push('|'),
            Key::letter('}') => BUFFER.push('}'),
            Key::letter('~') => BUFFER.push('~'),
            _ =>  BUFFER.pass(key)
        }
    }
}
#[cfg(target_os = "macos")]
pub fn KEY_ACTIVITY_MAPPER(key: Key)
{
    unsafe
    {
        match key   {
            Key::arrow("left") => BUFFER.handle_left(),
            Key::arrow("right") => BUFFER.handle_right(),
            Key::Backspace => BUFFER.delete(),
            Key::Delete => BUFFER.backspace(),
            Key::letter(' ') => BUFFER.push(' '),
            Key::letter('!') => BUFFER.push('!'),
            Key::letter('"') => BUFFER.push('"'),
            Key::letter('#') => BUFFER.push('#'),
            Key::letter('$') => BUFFER.push('$'),
            Key::letter('%') => BUFFER.push('%'),
            Key::letter('&') => BUFFER.push('&'),
            Key::letter('\'') => BUFFER.push('\''),
            Key::letter('(') => BUFFER.push('('),
            Key::letter(')') => BUFFER.push(')'),
            Key::letter('*') => BUFFER.push('*'),
            Key::letter('+') => BUFFER.push('+'),
            Key::letter(',') => BUFFER.push(','),
            Key::letter('-') => BUFFER.push('-'),
            Key::letter('.') => BUFFER.push('.'),
            Key::letter('/') => BUFFER.push('/'),
            Key::letter('0') => BUFFER.push('0'),
            Key::letter('1') => BUFFER.push('1'),
            Key::letter('2') => BUFFER.push('2'),
            Key::letter('3') => BUFFER.push('3'),
            Key::letter('4') => BUFFER.push('4'),
            Key::letter('5') => BUFFER.push('5'),
            Key::letter('6') => BUFFER.push('6'),
            Key::letter('7') => BUFFER.push('7'),
            Key::letter('8') => BUFFER.push('8'),
            Key::letter('9') => BUFFER.push('9'),
            Key::letter(':') => BUFFER.push(':'),
            Key::letter(';') => BUFFER.push(';'),
            Key::letter('<') => BUFFER.push('<'),
            Key::letter('=') => BUFFER.push('='),
            Key::letter('>') => BUFFER.push('>'),
            Key::letter('?') => BUFFER.push('?'),
            Key::letter('@') => BUFFER.push('@'),
            Key::letter('A') => BUFFER.push('A'),
            Key::letter('B') => BUFFER.push('B'),
            Key::letter('C') => BUFFER.push('C'),
            Key::letter('D') => BUFFER.push('D'),
            Key::letter('E') => BUFFER.push('E'),
            Key::letter('F') => BUFFER.push('F'),
            Key::letter('G') => BUFFER.push('G'),
            Key::letter('H') => BUFFER.push('H'),
            Key::letter('I') => BUFFER.push('I'),
            Key::letter('J') => BUFFER.push('J'),
            Key::letter('K') => BUFFER.push('K'),
            Key::letter('L') => BUFFER.push('L'),
            Key::letter('M') => BUFFER.push('M'),
            Key::letter('N') => BUFFER.push('N'),
            Key::letter('O') => BUFFER.push('O'),
            Key::letter('P') => BUFFER.push('P'),
            Key::letter('Q') => BUFFER.push('Q'),
            Key::letter('R') => BUFFER.push('R'),
            Key::letter('S') => BUFFER.push('S'),
            Key::letter('T') => BUFFER.push('T'),
            Key::letter('U') => BUFFER.push('U'),
            Key::letter('V') => BUFFER.push('V'),
            Key::letter('W') => BUFFER.push('W'),
            Key::letter('X') => BUFFER.push('X'),
            Key::letter('Y') => BUFFER.push('Y'),
            Key::letter('Z') => BUFFER.push('Z'),
            Key::letter('[') => BUFFER.push('['),
            Key::letter('\\') => BUFFER.push('\\'),
            Key::letter(']') => BUFFER.push(']'),
            Key::letter('^') => BUFFER.push('^'),
            Key::letter('_') => BUFFER.push('_'),
            Key::letter('`') => BUFFER.push('`'),
            Key::letter('a') => BUFFER.push('a'),
            Key::letter('b') => BUFFER.push('b'),
            Key::letter('c') => BUFFER.push('c'),
            Key::letter('d') => BUFFER.push('d'),
            Key::letter('e') => BUFFER.push('e'),
            Key::letter('f') => BUFFER.push('f'),
            Key::letter('g') => BUFFER.push('g'),
            Key::letter('h') => BUFFER.push('h'),
            Key::letter('i') => BUFFER.push('i'),
            Key::letter('j') => BUFFER.push('j'),
            Key::letter('k') => BUFFER.push('k'),
            Key::letter('l') => BUFFER.push('l'),
            Key::letter('m') => BUFFER.push('m'),
            Key::letter('n') => BUFFER.push('n'),
            Key::letter('o') => BUFFER.push('o'),
            Key::letter('p') => BUFFER.push('p'),
            Key::letter('q') => BUFFER.push('q'),
            Key::letter('r') => BUFFER.push('r'),
            Key::letter('s') => BUFFER.push('s'),
            Key::letter('t') => BUFFER.push('t'),
            Key::letter('u') => BUFFER.push('u'),
            Key::letter('v') => BUFFER.push('v'),
            Key::letter('w') => BUFFER.push('w'),
            Key::letter('x') => BUFFER.push('x'),
            Key::letter('y') => BUFFER.push('y'),
            Key::letter('z') => BUFFER.push('z'),
            Key::letter('{') => BUFFER.push('{'),
            Key::letter('|') => BUFFER.push('|'),
            Key::letter('}') => BUFFER.push('}'),
            Key::letter('~') => BUFFER.push('~'),
            _ =>  BUFFER.pass(key)
        }
    }
}
#[cfg(target_os = "windows")]
pub fn KEY_ACTIVITY_MAPPER(key: Key)
{
    unsafe
    {
        match key   {
            Key::arrow("left") => BUFFER.handle_left(),
            Key::arrow("right") => BUFFER.handle_right(),
            Key::Backspace => BUFFER.delete(),
            Key::Delete => BUFFER.backspace(),
            Key::letter(' ') => BUFFER.push(' '),
            Key::letter('!') => BUFFER.push('!'),
            Key::letter('"') => BUFFER.push('"'),
            Key::letter('#') => BUFFER.push('#'),
            Key::letter('$') => BUFFER.push('$'),
            Key::letter('%') => BUFFER.push('%'),
            Key::letter('&') => BUFFER.push('&'),
            Key::letter('\'') => BUFFER.push('\''),
            Key::letter('(') => BUFFER.push('('),
            Key::letter(')') => BUFFER.push(')'),
            Key::letter('*') => BUFFER.push('*'),
            Key::letter('+') => BUFFER.push('+'),
            Key::letter(',') => BUFFER.push(','),
            Key::letter('-') => BUFFER.push('-'),
            Key::letter('.') => BUFFER.push('.'),
            Key::letter('/') => BUFFER.push('/'),
            Key::letter('0') => BUFFER.push('0'),
            Key::letter('1') => BUFFER.push('1'),
            Key::letter('2') => BUFFER.push('2'),
            Key::letter('3') => BUFFER.push('3'),
            Key::letter('4') => BUFFER.push('4'),
            Key::letter('5') => BUFFER.push('5'),
            Key::letter('6') => BUFFER.push('6'),
            Key::letter('7') => BUFFER.push('7'),
            Key::letter('8') => BUFFER.push('8'),
            Key::letter('9') => BUFFER.push('9'),
            Key::letter(':') => BUFFER.push(':'),
            Key::letter(';') => BUFFER.push(';'),
            Key::letter('<') => BUFFER.push('<'),
            Key::letter('=') => BUFFER.push('='),
            Key::letter('>') => BUFFER.push('>'),
            Key::letter('?') => BUFFER.push('?'),
            Key::letter('@') => BUFFER.push('@'),
            Key::letter('A') => BUFFER.push('A'),
            Key::letter('B') => BUFFER.push('B'),
            Key::letter('C') => BUFFER.push('C'),
            Key::letter('D') => BUFFER.push('D'),
            Key::letter('E') => BUFFER.push('E'),
            Key::letter('F') => BUFFER.push('F'),
            Key::letter('G') => BUFFER.push('G'),
            Key::letter('H') => BUFFER.push('H'),
            Key::letter('I') => BUFFER.push('I'),
            Key::letter('J') => BUFFER.push('J'),
            Key::letter('K') => BUFFER.push('K'),
            Key::letter('L') => BUFFER.push('L'),
            Key::letter('M') => BUFFER.push('M'),
            Key::letter('N') => BUFFER.push('N'),
            Key::letter('O') => BUFFER.push('O'),
            Key::letter('P') => BUFFER.push('P'),
            Key::letter('Q') => BUFFER.push('Q'),
            Key::letter('R') => BUFFER.push('R'),
            Key::letter('S') => BUFFER.push('S'),
            Key::letter('T') => BUFFER.push('T'),
            Key::letter('U') => BUFFER.push('U'),
            Key::letter('V') => BUFFER.push('V'),
            Key::letter('W') => BUFFER.push('W'),
            Key::letter('X') => BUFFER.push('X'),
            Key::letter('Y') => BUFFER.push('Y'),
            Key::letter('Z') => BUFFER.push('Z'),
            Key::letter('[') => BUFFER.push('['),
            Key::letter('\\') => BUFFER.push('\\'),
            Key::letter(']') => BUFFER.push(']'),
            Key::letter('^') => BUFFER.push('^'),
            Key::letter('_') => BUFFER.push('_'),
            Key::letter('`') => BUFFER.push('`'),
            Key::letter('a') => BUFFER.push('a'),
            Key::letter('b') => BUFFER.push('b'),
            Key::letter('c') => BUFFER.push('c'),
            Key::letter('d') => BUFFER.push('d'),
            Key::letter('e') => BUFFER.push('e'),
            Key::letter('f') => BUFFER.push('f'),
            Key::letter('g') => BUFFER.push('g'),
            Key::letter('h') => BUFFER.push('h'),
            Key::letter('i') => BUFFER.push('i'),
            Key::letter('j') => BUFFER.push('j'),
            Key::letter('k') => BUFFER.push('k'),
            Key::letter('l') => BUFFER.push('l'),
            Key::letter('m') => BUFFER.push('m'),
            Key::letter('n') => BUFFER.push('n'),
            Key::letter('o') => BUFFER.push('o'),
            Key::letter('p') => BUFFER.push('p'),
            Key::letter('q') => BUFFER.push('q'),
            Key::letter('r') => BUFFER.push('r'),
            Key::letter('s') => BUFFER.push('s'),
            Key::letter('t') => BUFFER.push('t'),
            Key::letter('u') => BUFFER.push('u'),
            Key::letter('v') => BUFFER.push('v'),
            Key::letter('w') => BUFFER.push('w'),
            Key::letter('x') => BUFFER.push('x'),
            Key::letter('y') => BUFFER.push('y'),
            Key::letter('z') => BUFFER.push('z'),
            Key::letter('{') => BUFFER.push('{'),
            Key::letter('|') => BUFFER.push('|'),
            Key::letter('}') => BUFFER.push('}'),
            Key::letter('~') => BUFFER.push('~'),
            _ =>  BUFFER.pass(key)
        }
    }
}

impl Buffer {
    fn handle_right(&mut self) {
        if self.pointer < self.size {
            self.pointer += 1;
        }
    }

    fn handle_left(&mut self)   {
        if self.pointer > 0    {
            self.pointer -= 1;
        }
    }

    fn push(&mut self, ch: char)  {
        let mut left_str:String = self.buffer[..self.pointer as usize].to_string();
        let right_str:String = self.buffer[self.pointer as usize ..(self.size) as usize].to_string();

        left_str.push(ch);
        left_str.push_str(right_str.as_str());
        self.buffer =  left_str;
        self.pointer += 1;
        self.size += 1;
    }

    fn backspace(&mut self)
    {
        if self.pointer == 0
        {
            return ();
        }
        let mut left_str:String = self.buffer[..(self.pointer-1) as usize].to_string();
        let right_str:String = self.buffer[self.pointer as usize ..(self.size) as usize].to_string();

        left_str.push_str(right_str.as_str());
        self.buffer =  left_str;
        if self.pointer > 0
        {
            self.size -= 1;
            self.pointer -= 1;
        }
    }

    fn delete(&mut self)
    {
        if self.pointer >= self.size
        {
            return ();
        }
        let mut left_str:String = self.buffer[..self.pointer as usize].to_string();
        let right_str:String = self.buffer[(self.pointer+1) as usize ..(self.size) as usize].to_string();

        left_str.push_str(right_str.as_str());
        self.buffer =  left_str;  
        self.size -= 1;
    }

    pub const fn new() -> Buffer {
        Buffer
        {
            buffer : String::new(),
            pointer : 0,
            size : 0,
        }
    }

    fn get_buffer(&mut self) -> String
    {
        return self.buffer.clone();
    }

    fn pass(&mut self, key: Key)
    {
        binder::handle_and_call(key);
    }

}

static mut BUFFER: Buffer = Buffer::new();

pub fn map_activity(key: Key)
{
    unsafe
    {
        KEY_ACTIVITY_MAPPER(key);
    }
}

pub fn get_buffer() -> String
{
    unsafe
    {
        return BUFFER.get_buffer();
    }
}



pub fn windows_key_vs_activity_mapper(key: Key)
{
    unsafe
    {
        match key   {
            Key::arrow("left") => BUFFER.handle_left(),
            Key::arrow("right") => BUFFER.handle_right(),
            Key::Backspace => BUFFER.delete(),
            Key::Delete => BUFFER.backspace(),
            Key::letter(' ') => BUFFER.push(' '),
            Key::letter('!') => BUFFER.push('!'),
            Key::letter('"') => BUFFER.push('"'),
            Key::letter('#') => BUFFER.push('#'),
            Key::letter('$') => BUFFER.push('$'),
            Key::letter('%') => BUFFER.push('%'),
            Key::letter('&') => BUFFER.push('&'),
            Key::letter('\'') => BUFFER.push('\''),
            Key::letter('(') => BUFFER.push('('),
            Key::letter(')') => BUFFER.push(')'),
            Key::letter('*') => BUFFER.push('*'),
            Key::letter('+') => BUFFER.push('+'),
            Key::letter(',') => BUFFER.push(','),
            Key::letter('-') => BUFFER.push('-'),
            Key::letter('.') => BUFFER.push('.'),
            Key::letter('/') => BUFFER.push('/'),
            Key::letter('0') => BUFFER.push('0'),
            Key::letter('1') => BUFFER.push('1'),
            Key::letter('2') => BUFFER.push('2'),
            Key::letter('3') => BUFFER.push('3'),
            Key::letter('4') => BUFFER.push('4'),
            Key::letter('5') => BUFFER.push('5'),
            Key::letter('6') => BUFFER.push('6'),
            Key::letter('7') => BUFFER.push('7'),
            Key::letter('8') => BUFFER.push('8'),
            Key::letter('9') => BUFFER.push('9'),
            Key::letter(':') => BUFFER.push(':'),
            Key::letter(';') => BUFFER.push(';'),
            Key::letter('<') => BUFFER.push('<'),
            Key::letter('=') => BUFFER.push('='),
            Key::letter('>') => BUFFER.push('>'),
            Key::letter('?') => BUFFER.push('?'),
            Key::letter('@') => BUFFER.push('@'),
            Key::letter('A') => BUFFER.push('A'),
            Key::letter('B') => BUFFER.push('B'),
            Key::letter('C') => BUFFER.push('C'),
            Key::letter('D') => BUFFER.push('D'),
            Key::letter('E') => BUFFER.push('E'),
            Key::letter('F') => BUFFER.push('F'),
            Key::letter('G') => BUFFER.push('G'),
            Key::letter('H') => BUFFER.push('H'),
            Key::letter('I') => BUFFER.push('I'),
            Key::letter('J') => BUFFER.push('J'),
            Key::letter('K') => BUFFER.push('K'),
            Key::letter('L') => BUFFER.push('L'),
            Key::letter('M') => BUFFER.push('M'),
            Key::letter('N') => BUFFER.push('N'),
            Key::letter('O') => BUFFER.push('O'),
            Key::letter('P') => BUFFER.push('P'),
            Key::letter('Q') => BUFFER.push('Q'),
            Key::letter('R') => BUFFER.push('R'),
            Key::letter('S') => BUFFER.push('S'),
            Key::letter('T') => BUFFER.push('T'),
            Key::letter('U') => BUFFER.push('U'),
            Key::letter('V') => BUFFER.push('V'),
            Key::letter('W') => BUFFER.push('W'),
            Key::letter('X') => BUFFER.push('X'),
            Key::letter('Y') => BUFFER.push('Y'),
            Key::letter('Z') => BUFFER.push('Z'),
            Key::letter('[') => BUFFER.push('['),
            Key::letter('\\') => BUFFER.push('\\'),
            Key::letter(']') => BUFFER.push(']'),
            Key::letter('^') => BUFFER.push('^'),
            Key::letter('_') => BUFFER.push('_'),
            Key::letter('`') => BUFFER.push('`'),
            Key::letter('a') => BUFFER.push('a'),
            Key::letter('b') => BUFFER.push('b'),
            Key::letter('c') => BUFFER.push('c'),
            Key::letter('d') => BUFFER.push('d'),
            Key::letter('e') => BUFFER.push('e'),
            Key::letter('f') => BUFFER.push('f'),
            Key::letter('g') => BUFFER.push('g'),
            Key::letter('h') => BUFFER.push('h'),
            Key::letter('i') => BUFFER.push('i'),
            Key::letter('j') => BUFFER.push('j'),
            Key::letter('k') => BUFFER.push('k'),
            Key::letter('l') => BUFFER.push('l'),
            Key::letter('m') => BUFFER.push('m'),
            Key::letter('n') => BUFFER.push('n'),
            Key::letter('o') => BUFFER.push('o'),
            Key::letter('p') => BUFFER.push('p'),
            Key::letter('q') => BUFFER.push('q'),
            Key::letter('r') => BUFFER.push('r'),
            Key::letter('s') => BUFFER.push('s'),
            Key::letter('t') => BUFFER.push('t'),
            Key::letter('u') => BUFFER.push('u'),
            Key::letter('v') => BUFFER.push('v'),
            Key::letter('w') => BUFFER.push('w'),
            Key::letter('x') => BUFFER.push('x'),
            Key::letter('y') => BUFFER.push('y'),
            Key::letter('z') => BUFFER.push('z'),
            Key::letter('{') => BUFFER.push('{'),
            Key::letter('|') => BUFFER.push('|'),
            Key::letter('}') => BUFFER.push('}'),
            Key::letter('~') => BUFFER.push('~'),
            _ =>  BUFFER.pass(key)
        }
    }
}

pub fn macos_key_vs_activity_mapper(key: Key)
{
    unsafe
    {
        match key   {
            Key::arrow("left") => BUFFER.handle_left(),
            Key::arrow("right") => BUFFER.handle_right(),
            Key::Backspace => BUFFER.delete(),
            Key::Delete => BUFFER.backspace(),
            Key::letter(' ') => BUFFER.push(' '),
            Key::letter('!') => BUFFER.push('!'),
            Key::letter('"') => BUFFER.push('"'),
            Key::letter('#') => BUFFER.push('#'),
            Key::letter('$') => BUFFER.push('$'),
            Key::letter('%') => BUFFER.push('%'),
            Key::letter('&') => BUFFER.push('&'),
            Key::letter('\'') => BUFFER.push('\''),
            Key::letter('(') => BUFFER.push('('),
            Key::letter(')') => BUFFER.push(')'),
            Key::letter('*') => BUFFER.push('*'),
            Key::letter('+') => BUFFER.push('+'),
            Key::letter(',') => BUFFER.push(','),
            Key::letter('-') => BUFFER.push('-'),
            Key::letter('.') => BUFFER.push('.'),
            Key::letter('/') => BUFFER.push('/'),
            Key::letter('0') => BUFFER.push('0'),
            Key::letter('1') => BUFFER.push('1'),
            Key::letter('2') => BUFFER.push('2'),
            Key::letter('3') => BUFFER.push('3'),
            Key::letter('4') => BUFFER.push('4'),
            Key::letter('5') => BUFFER.push('5'),
            Key::letter('6') => BUFFER.push('6'),
            Key::letter('7') => BUFFER.push('7'),
            Key::letter('8') => BUFFER.push('8'),
            Key::letter('9') => BUFFER.push('9'),
            Key::letter(':') => BUFFER.push(':'),
            Key::letter(';') => BUFFER.push(';'),
            Key::letter('<') => BUFFER.push('<'),
            Key::letter('=') => BUFFER.push('='),
            Key::letter('>') => BUFFER.push('>'),
            Key::letter('?') => BUFFER.push('?'),
            Key::letter('@') => BUFFER.push('@'),
            Key::letter('A') => BUFFER.push('A'),
            Key::letter('B') => BUFFER.push('B'),
            Key::letter('C') => BUFFER.push('C'),
            Key::letter('D') => BUFFER.push('D'),
            Key::letter('E') => BUFFER.push('E'),
            Key::letter('F') => BUFFER.push('F'),
            Key::letter('G') => BUFFER.push('G'),
            Key::letter('H') => BUFFER.push('H'),
            Key::letter('I') => BUFFER.push('I'),
            Key::letter('J') => BUFFER.push('J'),
            Key::letter('K') => BUFFER.push('K'),
            Key::letter('L') => BUFFER.push('L'),
            Key::letter('M') => BUFFER.push('M'),
            Key::letter('N') => BUFFER.push('N'),
            Key::letter('O') => BUFFER.push('O'),
            Key::letter('P') => BUFFER.push('P'),
            Key::letter('Q') => BUFFER.push('Q'),
            Key::letter('R') => BUFFER.push('R'),
            Key::letter('S') => BUFFER.push('S'),
            Key::letter('T') => BUFFER.push('T'),
            Key::letter('U') => BUFFER.push('U'),
            Key::letter('V') => BUFFER.push('V'),
            Key::letter('W') => BUFFER.push('W'),
            Key::letter('X') => BUFFER.push('X'),
            Key::letter('Y') => BUFFER.push('Y'),
            Key::letter('Z') => BUFFER.push('Z'),
            Key::letter('[') => BUFFER.push('['),
            Key::letter('\\') => BUFFER.push('\\'),
            Key::letter(']') => BUFFER.push(']'),
            Key::letter('^') => BUFFER.push('^'),
            Key::letter('_') => BUFFER.push('_'),
            Key::letter('`') => BUFFER.push('`'),
            Key::letter('a') => BUFFER.push('a'),
            Key::letter('b') => BUFFER.push('b'),
            Key::letter('c') => BUFFER.push('c'),
            Key::letter('d') => BUFFER.push('d'),
            Key::letter('e') => BUFFER.push('e'),
            Key::letter('f') => BUFFER.push('f'),
            Key::letter('g') => BUFFER.push('g'),
            Key::letter('h') => BUFFER.push('h'),
            Key::letter('i') => BUFFER.push('i'),
            Key::letter('j') => BUFFER.push('j'),
            Key::letter('k') => BUFFER.push('k'),
            Key::letter('l') => BUFFER.push('l'),
            Key::letter('m') => BUFFER.push('m'),
            Key::letter('n') => BUFFER.push('n'),
            Key::letter('o') => BUFFER.push('o'),
            Key::letter('p') => BUFFER.push('p'),
            Key::letter('q') => BUFFER.push('q'),
            Key::letter('r') => BUFFER.push('r'),
            Key::letter('s') => BUFFER.push('s'),
            Key::letter('t') => BUFFER.push('t'),
            Key::letter('u') => BUFFER.push('u'),
            Key::letter('v') => BUFFER.push('v'),
            Key::letter('w') => BUFFER.push('w'),
            Key::letter('x') => BUFFER.push('x'),
            Key::letter('y') => BUFFER.push('y'),
            Key::letter('z') => BUFFER.push('z'),
            Key::letter('{') => BUFFER.push('{'),
            Key::letter('|') => BUFFER.push('|'),
            Key::letter('}') => BUFFER.push('}'),
            Key::letter('~') => BUFFER.push('~'),
            _ =>  BUFFER.pass(key)
        }
    }
}
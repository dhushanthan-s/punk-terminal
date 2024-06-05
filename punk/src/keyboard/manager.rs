use Keys;

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

    fn handle_left(&mut self)   {
        if(self.pointer > 0)    {
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
        if(self.pointer == 0)
        {
            return ();
        }
        let mut left_str:String = self.buffer[..(self.pointer-1) as usize].to_string();
        let right_str:String = self.buffer[self.pointer as usize ..(self.size) as usize].to_string();

        left_str.push_str(right_str.as_str());
        self.buffer =  left_str;
        if( self.pointer > 0)
        {
            self.size -= 1;
            self.pointer -= 1;
        }
    }

    fn delete(&mut self)
    {
        if(self.pointer >= self.size)
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

    fn pass(&mut self, key: Keys)
    {
        println!("Kindly handle {:?}", key);
        ()
    }

}

static mut BUFFER: Buffer = Buffer::new();

pub fn map_activity(key: Keys)
{
    unsafe
    {
        match key   {
            Keys::Left => BUFFER.handle_left(),
            Keys::Right => BUFFER.handle_right(),
            Keys::Backtab => BUFFER.backspace(),
            Keys::Delete => BUFFER.backspace(),
            Keys::Char(' ') => BUFFER.push(' '),
            Keys::Char('!') => BUFFER.push('!'),
            Keys::Char('"') => BUFFER.push('"'),
            Keys::Char('#') => BUFFER.push('#'),
            Keys::Char('$') => BUFFER.push('$'),
            Keys::Char('%') => BUFFER.push('%'),
            Keys::Char('&') => BUFFER.push('&'),
            Keys::Char('\'') => BUFFER.push('\''),
            Keys::Char('(') => BUFFER.push('('),
            Keys::Char(')') => BUFFER.push(')'),
            Keys::Char('*') => BUFFER.push('*'),
            Keys::Char('+') => BUFFER.push('+'),
            Keys::Char(',') => BUFFER.push(','),
            Keys::Char('-') => BUFFER.push('-'),
            Keys::Char('.') => BUFFER.push('.'),
            Keys::Char('/') => BUFFER.push('/'),
            Keys::Char('0') => BUFFER.push('0'),
            Keys::Char('1') => BUFFER.push('1'),
            Keys::Char('2') => BUFFER.push('2'),
            Keys::Char('3') => BUFFER.push('3'),
            Keys::Char('4') => BUFFER.push('4'),
            Keys::Char('5') => BUFFER.push('5'),
            Keys::Char('6') => BUFFER.push('6'),
            Keys::Char('7') => BUFFER.push('7'),
            Keys::Char('8') => BUFFER.push('8'),
            Keys::Char('9') => BUFFER.push('9'),
            Keys::Char(':') => BUFFER.push(':'),
            Keys::Char(';') => BUFFER.push(';'),
            Keys::Char('<') => BUFFER.push('<'),
            Keys::Char('=') => BUFFER.push('='),
            Keys::Char('>') => BUFFER.push('>'),
            Keys::Char('?') => BUFFER.push('?'),
            Keys::Char('@') => BUFFER.push('@'),
            Keys::Char('A') => BUFFER.push('A'),
            Keys::Char('B') => BUFFER.push('B'),
            Keys::Char('C') => BUFFER.push('C'),
            Keys::Char('D') => BUFFER.push('D'),
            Keys::Char('E') => BUFFER.push('E'),
            Keys::Char('F') => BUFFER.push('F'),
            Keys::Char('G') => BUFFER.push('G'),
            Keys::Char('H') => BUFFER.push('H'),
            Keys::Char('I') => BUFFER.push('I'),
            Keys::Char('J') => BUFFER.push('J'),
            Keys::Char('K') => BUFFER.push('K'),
            Keys::Char('L') => BUFFER.push('L'),
            Keys::Char('M') => BUFFER.push('M'),
            Keys::Char('N') => BUFFER.push('N'),
            Keys::Char('O') => BUFFER.push('O'),
            Keys::Char('P') => BUFFER.push('P'),
            Keys::Char('Q') => BUFFER.push('Q'),
            Keys::Char('R') => BUFFER.push('R'),
            Keys::Char('S') => BUFFER.push('S'),
            Keys::Char('T') => BUFFER.push('T'),
            Keys::Char('U') => BUFFER.push('U'),
            Keys::Char('V') => BUFFER.push('V'),
            Keys::Char('W') => BUFFER.push('W'),
            Keys::Char('X') => BUFFER.push('X'),
            Keys::Char('Y') => BUFFER.push('Y'),
            Keys::Char('Z') => BUFFER.push('Z'),
            Keys::Char('[') => BUFFER.push('['),
            Keys::Char('\\') => BUFFER.push('\\'),
            Keys::Char(']') => BUFFER.push(']'),
            Keys::Char('^') => BUFFER.push('^'),
            Keys::Char('_') => BUFFER.push('_'),
            Keys::Char('`') => BUFFER.push('`'),
            Keys::Char('a') => BUFFER.push('a'),
            Keys::Char('b') => BUFFER.push('b'),
            Keys::Char('c') => BUFFER.push('c'),
            Keys::Char('d') => BUFFER.push('d'),
            Keys::Char('e') => BUFFER.push('e'),
            Keys::Char('f') => BUFFER.push('f'),
            Keys::Char('g') => BUFFER.push('g'),
            Keys::Char('h') => BUFFER.push('h'),
            Keys::Char('i') => BUFFER.push('i'),
            Keys::Char('j') => BUFFER.push('j'),
            Keys::Char('k') => BUFFER.push('k'),
            Keys::Char('l') => BUFFER.push('l'),
            Keys::Char('m') => BUFFER.push('m'),
            Keys::Char('n') => BUFFER.push('n'),
            Keys::Char('o') => BUFFER.push('o'),
            Keys::Char('p') => BUFFER.push('p'),
            Keys::Char('q') => BUFFER.push('q'),
            Keys::Char('r') => BUFFER.push('r'),
            Keys::Char('s') => BUFFER.push('s'),
            Keys::Char('t') => BUFFER.push('t'),
            Keys::Char('u') => BUFFER.push('u'),
            Keys::Char('v') => BUFFER.push('v'),
            Keys::Char('w') => BUFFER.push('w'),
            Keys::Char('x') => BUFFER.push('x'),
            Keys::Char('y') => BUFFER.push('y'),
            Keys::Char('z') => BUFFER.push('z'),
            Keys::Char('{') => BUFFER.push('{'),
            Keys::Char('|') => BUFFER.push('|'),
            Keys::Char('}') => BUFFER.push('}'),
            Keys::Char('~') => BUFFER.push('~'),
            _ =>  BUFFER.pass(key)
        }
    }
}

pub fn get_buffer() -> String
{
    unsafe
    {
        return BUFFER.get_buffer();
    }
}
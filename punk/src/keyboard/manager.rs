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
        println!("Before - {:?}", self.buffer);
        let mut left_str:String = self.buffer[..self.pointer as usize].to_string();
        let right_str:String = self.buffer[self.pointer as usize ..(self.size) as usize].to_string();

        left_str.push(ch);
        left_str.push_str(right_str.as_str());
        self.buffer =  left_str;
        self.pointer += 1;
        println!("After - {:?}", self.buffer);
    }

    fn backspace(&mut self)
    {
        let mut left_str:String = self.buffer[..(self.pointer-1) as usize].to_string();
        let right_str:String = self.buffer[self.pointer as usize ..(self.size+1) as usize].to_string();

        left_str.push_str(right_str.as_str());
        self.buffer =  left_str;
    }

    fn delete(&mut self)
    {
        let mut left_str:String = self.buffer[..self.pointer as usize].to_string();
        let right_str:String = self.buffer[(self.pointer+1) as usize ..(self.size+1) as usize].to_string();

        left_str.push_str(right_str.as_str());
        self.buffer =  left_str;  }

    pub const fn new() -> Buffer {
        Buffer
        {
            buffer : String::new(),
            pointer : 0,
            size : 0,
        }
    }

    fn pass(&mut self)
    {
        ()
    }

}

pub fn map_activity(key: Keys, BUFFER: Buffer)
{
    match key   {
        Keys::Left => BUFFER.handle_left(),
        Keys::Right => BUFFER.handle_right(),
        Keys::Backtab => BUFFER.backspace(),
        Keys::Delete => BUFFER.delete(),
        Keys::Char('A') => BUFFER.push('A'),
        _ =>  BUFFER.pass()
    }
}
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref READER: Mutex<BufReader<File>> = {
        let file = File::open("/dev/tty").expect("Failed to open TTY device");
        Mutex::new(BufReader::new(file))
    };

    pub static ref WRITER: Mutex<File> = {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open("/dev/tty")
            .expect("Failed to open TTY device");
        Mutex::new(file)
    };
}

pub fn read_tty() -> String {
    let mut reader = READER.lock().unwrap();
    let mut input = String::new();
    match reader.read_line(&mut input)
    {
        Ok(_) => return input.trim().to_string(),
        Err(_) => return "Internal Error".to_string()
    }
}

pub fn write_tty(output: &[u8]) -> io::Result<()> {
    let mut writer = WRITER.lock().unwrap();
    writer.write_all(output)
}

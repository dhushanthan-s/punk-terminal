use std::fs::{self, File, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::Mutex;
use terminal::tty::fs::OpenOptions;

lazy_static::lazy_static! {
    pub static ref READER: Mutex<BufReader<File>> = {
        let file = File::open("/dev/tty").expect("Failed to open TTY device");
        Mutex::new(BufReader::new(file))
    };

    pub static ref WRITER: Mutex<std::fs::File> = {
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
    match reader.read_line(&mut input) {
        Ok(_) => {
            let trimmed_input = input.trim().to_string();
            return trimmed_input;
        }
        Err(err) => 
        {
            return "internal error".to_string();
        },
    }
}

pub fn write_tty(output: &[u8]) -> io::Result<()> {
    let mut writer = WRITER.lock().unwrap();
    writer.write_all(output)
}
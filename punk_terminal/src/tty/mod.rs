use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;
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
    match reader.read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(_) => "Internal Error".to_string(),
    }
}

pub fn write_tty(output: &[u8]) -> io::Result<()> {
    let mut writer = WRITER.lock().unwrap();
    writer.write_all(output)
}

pub fn execute(cmd: &str) {
    let raw_output = Command::new(cmd).output();

    let output = raw_output.unwrap();

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

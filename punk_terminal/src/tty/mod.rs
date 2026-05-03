use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Mutex;
use std::thread;

struct ShellProcess {
    stdin: ChildStdin,
}

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

    static ref SHELL: Mutex<ShellProcess> = {
        let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        let mut child = Command::new(&shell_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to spawn default shell");
        let stdin = child.stdin.take().expect("Failed to get shell stdin");
        spawn_exit_watcher(child);
        Mutex::new(ShellProcess { stdin })
    };
}

fn spawn_exit_watcher(mut child: Child) {
    thread::spawn(move || {
        let status = child.wait().expect("Failed to wait on shell process");
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    });
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

pub fn send_to_shell(cmd: &str) {
    let mut shell = SHELL.lock().unwrap();
    if writeln!(shell.stdin, "{}", cmd).is_err() {
        std::process::exit(1);
    }
    if shell.stdin.flush().is_err() {
        std::process::exit(1);
    }
}

pub fn execute(cmd: &str) {
    let raw_output = Command::new(cmd).output();

    let output = raw_output.unwrap();

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

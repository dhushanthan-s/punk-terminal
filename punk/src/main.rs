use std::io;
use std::io::Write;

mod terminal;
fn main() {
    loop
    {
        println! ("{:?}",terminal::tty::read_tty());
        if let Err(err) = terminal::tty::write_tty("hello".as_bytes()) {
            eprintln!("Error writing to TTY: {}", err);
        }
    }
}

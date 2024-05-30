use std::io;
use std::io::Write;

mod terminal;
fn main() {
    loop
    {
        print!("? >");
        io::stdout().flush().unwrap();
        let mut cmd_to_execute = String::new();
        io::stdin().read_line(&mut cmd_to_execute).expect("Error occured while fetching from input stream");
        terminal::executor::execute(cmd_to_execute.trim());
    }
}

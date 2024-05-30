use std::process::Command;

pub fn execute(cmd:&str)
{
    let raw_output = Command::new(cmd)
    .output();

    let output = raw_output.unwrap();

    if output.status.success()
    {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
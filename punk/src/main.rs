extern crate k_board;

use k_board::keys::Keys;

mod terminal;
mod keyboard;


fn main() {
    let clc : u8 = 21;
    loop
    {
        let key:Keys = keyboard::buffer::watch();
        if(key.eq(&Keys::Null))
        {
            continue;
        }
        if(key.eq(&Keys::Ctrl('c')))
        {
            println!("Bye ...");
            break;
        }
        keyboard::manager::map_activity(key);
        terminal::tty::write_tty(&[clc]);
        terminal::tty::write_tty(keyboard::manager::get_buffer().as_bytes());
    }
}

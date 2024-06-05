extern crate k_board;

use k_board::keys::Keys;

mod terminal;
mod keyboard;


fn main() {
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
        println!("{:?}",keyboard::manager::get_buffer())
    }
}

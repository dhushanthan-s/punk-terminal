extern crate k_board;

use k_board::keys::Keys;
use keyboard::manager::Buffer;

mod terminal;
mod keyboard;


fn main() {
    let mut c = 1;
    let mut BUFFER: Buffer = Buffer::new();
    loop
    {
        let key:Keys = keyboard::buffer::watch();
        keyboard::manager::map_activity(key, &BUFFER);
        c+=1;
        if(c==10)
        {
            break;
        }
    }
}

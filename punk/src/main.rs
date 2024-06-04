extern crate k_board;

use k_board::keys::Keys;

mod terminal;
mod keyboard;

fn main() {
    let mut c = 1;
    loop
    {
        let key:Keys = keyboard::buffer::watch();
        println!("{:?}", key);
        c+=1;
        if(c==10)
        {
            break;
        }
    }
}

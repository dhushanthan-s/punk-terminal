use Key;
use enums::key_mapper;
use std::io;
use std::io::Read;

static mut INPUT : [u8 ; 5] = [0; 5];
pub fn watch() -> Key<'static> {
    let stdin = io::stdin();
    unsafe
    {
        let input_len = stdin.lock().read(&mut INPUT).expect("Failed to read input");
        let key = &INPUT[0..input_len];
        return key_mapper::matcher(key);
    }
}
use crate::enums::key_mapper;
use std::io;
use std::io::Read;

use crate::enums::Key;

static mut INPUT: [u8; 5] = [0; 5];

pub fn watch() -> Key<'static> {
    let stdin = io::stdin();
    unsafe {
        let input_ptr: *mut [u8; 5] = core::ptr::addr_of_mut!(INPUT);
        let input = &mut *input_ptr;
        let input_len = stdin.lock().read(input).expect("Failed to read input");
        let key = &input[0..input_len];
        key_mapper::matcher(key)
    }
}

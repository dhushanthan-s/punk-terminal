use std::sync::Once;

static INIT: Once = Once::new();

pub mod handler;
pub mod helper;
pub mod os;

pub fn init() {
    INIT.call_once(|| {
        helper::initialize_resource();
    });
}

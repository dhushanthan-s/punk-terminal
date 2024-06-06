use k_board::{keyboard::Keyboard, keys::Keys};

pub fn watch() -> Keys {
    let key:Keys = Keys::Enter;
    let keyboard: Keyboard = Keyboard::new();
    for key in keyboard {
        return key;
    }
    return key;
}
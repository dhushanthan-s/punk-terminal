// use k_board::{keyboard::Keyboard, keys::Keys};

// pub fn watch() -> Option<Keys>
// {
//     for key in Keyboard::new() {
//         match key {
//             Keys::Enter => {
//                 break;
//             }
//             _ => {return Some(key)}
//         }
//     }

//     return Some(Keys::Enter);
// }

use k_board::{keyboard::Keyboard, keys::Keys};

pub fn watch() -> Keys {
    let key:Keys = Keys::Enter;
    let keyboard: Keyboard = Keyboard::new();
    for key in keyboard {
        return key;
    }
    return key;
}

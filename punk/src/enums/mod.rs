pub mod key_mapper;

#[derive(Debug, Clone)]
pub enum Key<'a> {
    letter(char),
    ctrl(char),
    arrow(&'a str),
    Backspace,
    Tab,
    Esc,
    Delete,
    unknown,
}

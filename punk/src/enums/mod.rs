pub mod key_mapper;

#[derive(Debug, Clone)]
pub enum Key<'a> {
    Letter(char),
    Ctrl(char),
    Arrow(&'a str),
    Backspace,
    Tab,
    Esc,
    Delete,
    Unknown,
}

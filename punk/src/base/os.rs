use crate::base::handler::*;

#[cfg(target_os = "linux")]
pub fn linux_specific_handler() {
    let _ = unix_handler::enable_raw_mode();
}

#[cfg(target_os = "macos")]
pub fn macos_specific_handler() {
    let _ = unix_handler::enable_raw_mode();
}

#[cfg(target_os = "windows")]
pub fn windows_specific_handler() {
    let handle = get_stdin_handle();
    let _ = windows_handler::enable_raw_mode(handle);
}

use crate::base::handler::*;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn linux_specific_handler() {
    let _ = unix_handler::enable_raw_mode();
}

#[cfg(target_os = "windows")]
pub fn windows_specific_handler() {
    let handle = get_stdin_handle();
    let _ = windows_handler::enable_raw_mode(handle);
}

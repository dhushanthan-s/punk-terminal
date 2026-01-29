use crate::base::os::*;

pub fn initialize_resource() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    linux_specific_handler();

    #[cfg(target_os = "windows")]
    windows_specific_handler();
}

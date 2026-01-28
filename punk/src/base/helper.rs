use base::os::*;

pub fn initialize_resource() {
    #[cfg(target_os = "linux")]
    linux_specific_handler();

    #[cfg(target_os = "macos")]
    macos_specific_handler();

    #[cfg(target_os = "windows")]
    windows_specific_handler();
}

use base::os::OperatingSystem;

pub static mut CURRENT_OS : OperatingSystem = OperatingSystem::Linux;

pub fn initialize_resource() {
    unsafe
    {
        CURRENT_OS = detect_os();
    }
}

fn detect_os() -> OperatingSystem {
    let os_name = std::env::consts::OS;
    match os_name.to_lowercase().as_str(){
        "linux" => OperatingSystem::Linux,
        "darwin" => OperatingSystem::MacOS,
        "windows" => OperatingSystem::Windows,
        _ => OperatingSystem::Unknown
    }
}

pub fn get_current_os() -> OperatingSystem
{
    unsafe
    {
        return CURRENT_OS.clone();
    }
}

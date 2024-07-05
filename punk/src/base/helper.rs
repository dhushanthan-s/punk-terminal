use base::os::OperatingSystem;
use keyboard::manager;
use Keys;

pub static mut CURRENT_OS : OperatingSystem = OperatingSystem::Linux;

pub fn initialize_resource() {
    unsafe
    {
        CURRENT_OS = detect_os();
        match CURRENT_OS
        {
            OperatingSystem::Linux => 
            {
                manager::KEY_ACTIVITY_MAPPER = manager::linux_key_vs_activity_mapper as fn(Keys);
            },
            OperatingSystem::Windows =>
            {
                manager::KEY_ACTIVITY_MAPPER = manager::windows_key_vs_activity_mapper as fn(Keys);
            },
            OperatingSystem::MacOS =>
            {
                manager::KEY_ACTIVITY_MAPPER = manager::macos_key_vs_activity_mapper as fn(Keys);
            },
            _ =>
            {
                println!("Unknown OS found, unable to load keybindings");
            }
        }
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
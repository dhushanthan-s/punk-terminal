#[cfg(unix)]
pub mod unix_handler {
    use libc::{ECHO, ICANON, TCSANOW, c_uint, tcgetattr, tcsetattr, termios};
    use std::io;
    use std::mem;
    use std::os::unix::io::AsRawFd;

    pub fn enable_raw_mode() -> io::Result<()> {
        let mut termios = unsafe { mem::zeroed::<termios>() };
        if unsafe { tcgetattr(io::stdin().as_raw_fd(), &mut termios) } < 0 {
            return Err(io::Error::last_os_error());
        }
        let original_termios = termios;
        termios.c_lflag &= !(ICANON | ECHO);
        termios.c_cc[libc::VMIN] = 1;
        termios.c_cc[libc::VTIME] = 0;
        if unsafe { tcsetattr(io::stdin().as_raw_fd(), TCSANOW, &termios) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(windows)]
pub mod windows_handler {
    use std::io;
    use std::os::windows::io::{AsRawHandle, RawHandle};
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::wincon::{
        ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_MOUSE_INPUT, ENABLE_PROCESSED_INPUT,
        ENABLE_QUICK_EDIT_MODE,
    };
    use winapi::um::winnt::HANDLE;

    pub fn enable_raw_mode(handle: HANDLE) -> io::Result<()> {
        let mut mode: u32 = 0;
        if unsafe { GetConsoleMode(handle, &mut mode as *mut u32) } == 0 {
            return Err(io::Error::last_os_error());
        }
        let new_mode = mode
            & !(ENABLE_ECHO_INPUT
                | ENABLE_LINE_INPUT
                | ENABLE_MOUSE_INPUT
                | ENABLE_PROCESSED_INPUT
                | ENABLE_QUICK_EDIT_MODE);
        if unsafe { SetConsoleMode(handle, new_mode) } == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub fn get_stdin_handle() -> HANDLE {
        unsafe { GetStdHandle(winapi::um::winbase::STD_INPUT_HANDLE) }
    }
}

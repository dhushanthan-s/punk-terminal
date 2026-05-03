use libc::{self, winsize};
use std::ffi::{CString, OsStr};
use std::fs::File;
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

/// Master side of a PTY with a child shell.
pub struct PtySession {
    master: File,
    child_pid: libc::pid_t,
}

impl PtySession {
    /// Spawns `$SHELL -i` (or `/bin/sh -i`) on a new PTY with the given size.
    pub fn spawn_shell(cols: u16, rows: u16) -> io::Result<Self> {
        let master_fd = unsafe {
            let fd = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
            if fd < 0 {
                return Err(io::Error::last_os_error());
            }
            if libc::grantpt(fd) != 0 {
                let e = io::Error::last_os_error();
                libc::close(fd);
                return Err(e);
            }
            if libc::unlockpt(fd) != 0 {
                let e = io::Error::last_os_error();
                libc::close(fd);
                return Err(e);
            }
            fd
        };

        let mut pts_buf = [0u8; 512];
        let ok = unsafe {
            libc::ptsname_r(
                master_fd,
                pts_buf.as_mut_ptr() as *mut libc::c_char,
                pts_buf.len(),
            )
        };
        if ok != 0 {
            unsafe {
                libc::close(master_fd);
            }
            return Err(io::Error::last_os_error());
        }
        let nul = pts_buf
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(pts_buf.len());
        let slave_path = OsStr::from_bytes(&pts_buf[..nul]);

        let ws = winsize {
            ws_row: rows,
            ws_col: cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        if unsafe { libc::ioctl(master_fd, libc::TIOCSWINSZ, &ws) } != 0 {
            let e = io::Error::last_os_error();
            unsafe {
                libc::close(master_fd);
            }
            return Err(e);
        }

        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        let shell_c = CString::new(shell.as_str())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let dash_i =
            CString::new("-i").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        let slave_c = CString::new(slave_path.as_bytes())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        let pid = unsafe { libc::fork() };
        if pid < 0 {
            let e = io::Error::last_os_error();
            unsafe {
                libc::close(master_fd);
            }
            return Err(e);
        }

        if pid == 0 {
            unsafe {
                libc::close(master_fd);
                libc::setsid();
                let slave = libc::open(slave_c.as_ptr(), libc::O_RDWR | libc::O_NOCTTY);
                if slave < 0 {
                    libc::_exit(127);
                }
                if libc::ioctl(slave, libc::TIOCSCTTY, std::ptr::null_mut::<libc::c_void>()) != 0 {
                    libc::close(slave);
                    libc::_exit(126);
                }
                let _ = libc::dup2(slave, libc::STDIN_FILENO);
                let _ = libc::dup2(slave, libc::STDOUT_FILENO);
                let _ = libc::dup2(slave, libc::STDERR_FILENO);
                if slave > libc::STDERR_FILENO {
                    libc::close(slave);
                }
                let term_k = CString::new("TERM").unwrap();
                let term_v = CString::new("xterm-256color").unwrap();
                libc::setenv(term_k.as_ptr(), term_v.as_ptr(), 1);
                let argv: [*const libc::c_char; 3] =
                    [shell_c.as_ptr(), dash_i.as_ptr(), std::ptr::null()];
                libc::execvp(shell_c.as_ptr(), argv.as_ptr());
                libc::_exit(125);
            }
        }

        unsafe {
            let flags = libc::fcntl(master_fd, libc::F_GETFL, 0);
            if flags < 0 {
                let e = io::Error::last_os_error();
                libc::close(master_fd);
                libc::kill(pid, libc::SIGKILL);
                return Err(e);
            }
            if libc::fcntl(master_fd, libc::F_SETFL, flags | libc::O_NONBLOCK) < 0 {
                let e = io::Error::last_os_error();
                libc::close(master_fd);
                libc::kill(pid, libc::SIGKILL);
                return Err(e);
            }
        }

        let master = unsafe { File::from_raw_fd(master_fd) };
        Ok(Self {
            master,
            child_pid: pid,
        })
    }

    /// Reads from the PTY master. Returns `Ok(0)` when no data is available (non-blocking).
    pub fn try_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = unsafe {
            libc::read(
                self.master.as_raw_fd(),
                buf.as_mut_ptr() as *mut _,
                buf.len(),
            )
        };
        if n < 0 {
            let err = io::Error::last_os_error();
            if err.raw_os_error() == Some(libc::EAGAIN)
                || err.raw_os_error() == Some(libc::EWOULDBLOCK)
            {
                return Ok(0);
            }
            return Err(err);
        }
        Ok(n as usize)
    }

    pub fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.master.write_all(buf)
    }

    pub fn resize(&mut self, cols: u16, rows: u16) -> io::Result<()> {
        let ws = winsize {
            ws_row: rows,
            ws_col: cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        let r = unsafe { libc::ioctl(self.master.as_raw_fd(), libc::TIOCSWINSZ, &ws) };
        if r != 0 {
            return Err(io::Error::last_os_error());
        }
        unsafe {
            libc::kill(self.child_pid, libc::SIGWINCH);
        }
        Ok(())
    }

    pub fn master_fd(&self) -> RawFd {
        self.master.as_raw_fd()
    }
}

impl Drop for PtySession {
    fn drop(&mut self) {
        unsafe {
            libc::kill(self.child_pid, libc::SIGHUP);
        }
    }
}

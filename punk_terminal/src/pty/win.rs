use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::ptr;

use windows_sys::Win32::Foundation::{BOOL, CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
use windows_sys::Win32::Storage::FileSystem::{ReadFile, WriteFile};
use windows_sys::Win32::System::Console::{
    COORD, ClosePseudoConsole, CreatePseudoConsole, HPCON, ResizePseudoConsole,
};
use windows_sys::Win32::System::Pipes::{CreatePipe, PeekNamedPipe};
use windows_sys::Win32::System::Threading::{
    CREATE_UNICODE_ENVIRONMENT, CreateProcessW, DeleteProcThreadAttributeList,
    EXTENDED_STARTUPINFO_PRESENT, InitializeProcThreadAttributeList,
    PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE, PROCESS_INFORMATION, STARTUPINFOEXW, STARTUPINFOW,
    TerminateProcess, UpdateProcThreadAttribute,
};

pub struct PtySession {
    in_write: File,
    out_read: File,
    hpcon: HPCON,
    proc: HANDLE,
}

impl PtySession {
    pub fn spawn_shell(cols: u16, rows: u16) -> io::Result<Self> {
        let sa = SECURITY_ATTRIBUTES {
            nLength: mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: ptr::null_mut(),
            bInheritHandle: 1,
        };

        let mut in_read = INVALID_HANDLE_VALUE;
        let mut in_write = INVALID_HANDLE_VALUE;
        let mut out_read = INVALID_HANDLE_VALUE;
        let mut out_write = INVALID_HANDLE_VALUE;

        unsafe {
            if CreatePipe(&mut in_read, &mut in_write, &sa, 0) == 0 {
                return Err(io::Error::last_os_error());
            }
            if CreatePipe(&mut out_read, &mut out_write, &sa, 0) == 0 {
                let e = io::Error::last_os_error();
                CloseHandle(in_read);
                CloseHandle(in_write);
                return Err(e);
            }

            let size = COORD {
                X: cols as i16,
                Y: rows as i16,
            };
            let mut hpcon: HPCON = 0;
            let hr = CreatePseudoConsole(size, in_read, out_write, 0, &mut hpcon);
            if hr != 0 {
                CloseHandle(in_read);
                CloseHandle(in_write);
                CloseHandle(out_read);
                CloseHandle(out_write);
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("CreatePseudoConsole failed: HRESULT {hr}"),
                ));
            }

            let mut attr_size: usize = 0;
            let _ = InitializeProcThreadAttributeList(ptr::null_mut(), 1, 0, &mut attr_size);
            let mut attr_buf = vec![0u8; attr_size];
            let attr_list = attr_buf.as_mut_ptr() as *mut std::ffi::c_void;
            if InitializeProcThreadAttributeList(attr_list, 1, 0, &mut attr_size) == 0 {
                let e = io::Error::last_os_error();
                ClosePseudoConsole(hpcon);
                CloseHandle(in_read);
                CloseHandle(in_write);
                CloseHandle(out_read);
                CloseHandle(out_write);
                return Err(e);
            }

            if UpdateProcThreadAttribute(
                attr_list,
                0,
                PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE as usize,
                ptr::addr_of_mut!(hpcon) as *mut _,
                mem::size_of::<HPCON>(),
                ptr::null_mut(),
                ptr::null_mut(),
            ) == 0
            {
                let e = io::Error::last_os_error();
                DeleteProcThreadAttributeList(attr_list);
                ClosePseudoConsole(hpcon);
                CloseHandle(in_read);
                CloseHandle(in_write);
                CloseHandle(out_read);
                CloseHandle(out_write);
                return Err(e);
            }

            let comspec: OsString = std::env::var_os("COMSPEC")
                .unwrap_or_else(|| OsString::from("C:\\Windows\\System32\\cmd.exe"));
            let mut cmd_wide: Vec<u16> = comspec.encode_wide().collect();
            cmd_wide.push(0);

            let mut si: STARTUPINFOEXW = mem::zeroed();
            si.StartupInfo.cb = mem::size_of::<STARTUPINFOEXW>() as u32;
            si.lpAttributeList = attr_list;

            let mut pi: PROCESS_INFORMATION = mem::zeroed();
            let flags = CREATE_UNICODE_ENVIRONMENT | EXTENDED_STARTUPINFO_PRESENT;
            let ok: BOOL = CreateProcessW(
                ptr::null(),
                cmd_wide.as_mut_ptr(),
                ptr::null(),
                ptr::null(),
                0,
                flags,
                ptr::null(),
                ptr::null(),
                ptr::addr_of_mut!(si.StartupInfo) as *const STARTUPINFOW,
                &mut pi,
            );
            DeleteProcThreadAttributeList(attr_list);

            if ok == 0 {
                let e = io::Error::last_os_error();
                ClosePseudoConsole(hpcon);
                CloseHandle(in_read);
                CloseHandle(in_write);
                CloseHandle(out_read);
                CloseHandle(out_write);
                return Err(e);
            }

            CloseHandle(pi.hThread);

            let in_write_f = File::from_raw_handle(in_write as _);
            let out_read_f = File::from_raw_handle(out_read as _);

            Ok(Self {
                in_write: in_write_f,
                out_read: out_read_f,
                hpcon,
                proc: pi.hProcess,
            })
        }
    }

    pub fn try_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut avail = 0u32;
        unsafe {
            if PeekNamedPipe(
                self.out_read.as_raw_handle() as HANDLE,
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                &mut avail,
                ptr::null_mut(),
            ) == 0
            {
                return Err(io::Error::last_os_error());
            }
            if avail == 0 {
                return Ok(0);
            }
            let to_read = (avail as usize).min(buf.len()) as u32;
            let mut read = 0u32;
            if ReadFile(
                self.out_read.as_raw_handle() as HANDLE,
                buf.as_mut_ptr() as *mut _,
                to_read,
                &mut read,
                ptr::null_mut(),
            ) == 0
            {
                return Err(io::Error::last_os_error());
            }
            Ok(read as usize)
        }
    }

    pub fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let mut written = 0u32;
        unsafe {
            if WriteFile(
                self.in_write.as_raw_handle() as HANDLE,
                buf.as_ptr() as *const _,
                buf.len() as u32,
                &mut written,
                ptr::null_mut(),
            ) == 0
            {
                return Err(io::Error::last_os_error());
            }
        }
        if written as usize != buf.len() {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "short write to pseudoconsole input",
            ));
        }
        Ok(())
    }

    pub fn resize(&mut self, cols: u16, rows: u16) -> io::Result<()> {
        let size = COORD {
            X: cols as i16,
            Y: rows as i16,
        };
        let hr = unsafe { ResizePseudoConsole(self.hpcon, size) };
        if hr != 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("ResizePseudoConsole failed: HRESULT {hr}"),
            ));
        }
        Ok(())
    }
}

impl Drop for PtySession {
    fn drop(&mut self) {
        unsafe {
            let _ = TerminateProcess(self.proc, 1);
            let _ = CloseHandle(self.proc);
            ClosePseudoConsole(self.hpcon);
        }
    }
}

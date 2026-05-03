//! Pseudoterminal session: in-repo Unix (`libc`) and Windows (ConPTY + `windows-sys`) glue.

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod win;

#[cfg(unix)]
pub use unix::PtySession;
#[cfg(windows)]
pub use win::PtySession;

//! Bash [exit codes](https://tldp.org/LDP/abs/html/exitcodes.html)

/// Convert [`std::io::ErrorKind`] to a [`Code`][crate::Code]
#[inline]
pub fn io_to_signal(kind: std::io::ErrorKind) -> Option<crate::Code> {
    use std::io::ErrorKind::*;
    match kind {
        BrokenPipe => Some(SIGPIPE),
        TimedOut => Some(SIGALRM),
        Interrupted => Some(SIGINT),
        _ => None,
    }
}

/// Command line usage error
///
/// While bash generally documents this as "Misuse of shell builtins (according to Bash
/// documentation)", it is more broadly interpreted as a general usage error.
pub const USAGE: crate::Code = crate::Code::new(2);

/// Command was found but is not executable by the shell.
pub const NOT_EXECUTABLE: crate::Code = crate::Code::new(126);

/// Usually indicates that the command was not found by the shell, or that
/// the command is found but that a library it requires is not found.
pub const NOT_FOUND: crate::Code = crate::Code::new(127);

/// Usually indicates that the command was not found by the shell, or that
/// the command is found but that a library it requires is not found.
pub const INVALID_EXIT: crate::Code = crate::Code::new(128);

/// Exit status out of range
///
/// `exit` takes only integer args in the range 0 - 255
pub const STATUS_OUT_OF_RANGE: crate::Code = crate::Code::new(255);

const SIGBASE: i32 = 128;

/// The `SIGHUP` signal is sent to a process when its controlling terminal
/// is closed.
pub const SIGHUP: crate::Code = crate::Code::new(SIGBASE + 1);

/// The `SIGINT` signal is sent to a process by its controlling terminal
/// when a user wishes to interrupt the process.
pub const SIGINT: crate::Code = crate::Code::new(SIGBASE + 2);

/// The `SIGQUIT` signal is sent to a process by its controlling terminal
/// when a user quit from keyboard (Ctrl-\. or, Ctrl-4 or, on the virtual console, the SysRq key)
pub const SIGQUIT: crate::Code = crate::Code::new(SIGBASE + 3);

/// The `SIGILL` signal is sent to a process by its controlling terminal
/// when an illegal instruction is encountered
pub const SIGILL: crate::Code = crate::Code::new(SIGBASE + 4);

/// The `SIGTRAP` signal is sent to a process by its controlling terminal
/// when there is a trace/breakpoint trap
pub const SIGTRAP: crate::Code = crate::Code::new(SIGBASE + 5);

/// The `SIGABRT` signal is sent to a process by its controlling terminal
/// when process abort signal
pub const SIGABRT: crate::Code = crate::Code::new(SIGBASE + 6);

/// The `SIGFPE` signal is sent to a process by its controlling terminal
/// when there is an erroneous arithmetic operation
pub const SIGFPE: crate::Code = crate::Code::new(SIGBASE + 8);

/// The `SIGKILL` signal is sent to a process to cause it to terminate
/// immediately.  In contrast to `SIGTERM` and `SIGINT`, this signal cannot
/// be caught or ignored, and the receiving process cannot perform any
/// clean-up upon receiving this signal.
pub const SIGKILL: crate::Code = crate::Code::new(SIGBASE + 9);

/// The `SIGSEGV` signal is sent to a process on invalid memory reference
pub const SIGSEGV: crate::Code = crate::Code::new(SIGBASE + 11);

/// The `SIGPIPE` signal is sent to a process when it attempts to write to
/// a pipe without a process connected to the other end.
pub const SIGPIPE: crate::Code = crate::Code::new(SIGBASE + 13);

/// The `SIGALRM` signal is sent to a process when the time limit specified
/// in a call to a preceding alarm setting function (such as `setitimer`)
/// elapses.
pub const SIGALRM: crate::Code = crate::Code::new(SIGBASE + 14);

/// The `SIGTERM` signal is sent to a process to request its termination.
/// Unlike the `SIGKILL` signal, it can be caught and interpreted or
/// ignored by the process.
pub const SIGTERM: crate::Code = crate::Code::new(SIGBASE + 15);

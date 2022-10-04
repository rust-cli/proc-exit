/// Process exit code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Code(i32);

impl Code {
    /// The process exited successfully.
    pub const SUCCESS: Code = Code(0);

    /// Generic failure.
    pub const FAILURE: Code = Code(1);

    /// Catch-all exit code when the process exits for an unknown reason.
    pub const UNKNOWN: Code = Code(2);

    /// Command was found but is not executable by the shell.
    pub const NOT_EXECUTABLE: Code = Code(126);

    /// Usually indicates that the command was not found by the shell, or that
    /// the command is found but that a library it requires is not found.
    pub const NOT_FOUND: Code = Code(127);

    /// Usually indicates that the command was not found by the shell, or that
    /// the command is found but that a library it requires is not found.
    pub const INVALID_EXIT: Code = Code(128);

    const SIGBASE: i32 = 128;

    /// The `SIGHUP` signal is sent to a process when its controlling terminal
    /// is closed.
    pub const SIGHUP: Code = Code(Code::SIGBASE + 1);

    /// The `SIGINT` signal is sent to a process by its controlling terminal
    /// when a user wishes to interrupt the process.
    pub const SIGINT: Code = Code(Code::SIGBASE + 2);

    /// The `SIGQUIT` signal is sent to a process by its controlling terminal
    /// when a user quit from keyboard (Ctrl-\. or, Ctrl-4 or, on the virtual console, the SysRq key)
    pub const SIGQUIT: Code = Code(Code::SIGBASE + 3);

    /// The `SIGILL` signal is sent to a process by its controlling terminal
    /// when an illegal instruction is encountered
    pub const SIGILL: Code = Code(Code::SIGBASE + 4);

    /// The `SIGTRAP` signal is sent to a process by its controlling terminal
    /// when there is a trace/breakpoint trap
    pub const SIGTRAP: Code = Code(Code::SIGBASE + 5);

    /// The `SIGABRT` signal is sent to a process by its controlling terminal
    /// when process abort signal
    pub const SIGABRT: Code = Code(Code::SIGBASE + 6);

    /// The `SIGFPE` signal is sent to a process by its controlling terminal
    /// when there is an erroneous arithmetic operation
    pub const SIGFPE: Code = Code(Code::SIGBASE + 8);

    /// The `SIGKILL` signal is sent to a process to cause it to terminate
    /// immediately.  In contrast to `SIGTERM` and `SIGINT`, this signal cannot
    /// be caught or ignored, and the receiving process cannot perform any
    /// clean-up upon receiving this signal.
    pub const SIGKILL: Code = Code(Code::SIGBASE + 9);

    /// The `SIGSEGV` signal is sent to a process on invalid memory reference
    pub const SIGSEGV: Code = Code(Code::SIGBASE + 11);

    /// The `SIGPIPE` signal is sent to a process when it attempts to write to
    /// a pipe without a process connected to the other end.
    pub const SIGPIPE: Code = Code(Code::SIGBASE + 13);

    /// The `SIGALRM` signal is sent to a process when the time limit specified
    /// in a call to a preceding alarm setting function (such as `setitimer`)
    /// elapses.
    pub const SIGALRM: Code = Code(Code::SIGBASE + 14);

    /// The `SIGTERM` signal is sent to a process to request its termination.
    /// Unlike the `SIGKILL` signal, it can be caught and interpreted or
    /// ignored by the process.
    pub const SIGTERM: Code = Code(Code::SIGBASE + 15);

    pub const fn new(code: i32) -> Self {
        Self(code)
    }

    /// Converts [`std::process::ExitStatus`] to [`Code`].
    ///
    /// On Unix, if the process was terminated by a fatal signal, the corresponding
    /// signal exit code is returned.
    pub fn from_status(status: std::process::ExitStatus) -> Self {
        Self::from(status)
    }

    /// Coerce the code to a portable value
    #[cfg(feature = "portable")]
    pub const fn coerce(self) -> Option<Self> {
        if self.is_portable() {
            Some(self)
        } else {
            None
        }
    }

    #[cfg(not(feature = "portable"))]
    const fn coerce(self) -> Option<Self> {
        if self.is_portable() {
            Some(self)
        } else {
            None
        }
    }

    /// Test if provided exit code is portable across platforms.
    ///
    /// While Windows has wider types for return codes, Unix OS's tend to only support 8-bits,
    /// stripping off the higher order bits.
    #[cfg(feature = "portable")]
    pub const fn is_portable(self) -> bool {
        0 <= self.0 && self.0 <= 255
    }

    #[cfg(not(feature = "portable"))]
    const fn is_portable(self) -> bool {
        true
    }

    pub fn process_exit(self) -> ! {
        std::process::exit(self.coerce().unwrap_or_default().raw())
    }

    pub fn ok(self) -> crate::ExitResult {
        if self.0 == Self::SUCCESS.0 {
            Ok(())
        } else {
            Err(crate::Exit::new(self))
        }
    }

    pub fn into_exit(self) -> crate::Exit {
        assert_ne!(self, Self::SUCCESS);
        crate::Exit::new(self)
    }

    pub fn with_message<D: std::fmt::Display + 'static>(self, msg: D) -> crate::Exit {
        self.into_exit().with_message(msg)
    }

    /// Determines if the provided [`std::process::ExitStatus`] was successful.
    ///
    /// Example:
    ///
    /// ```
    /// use std::process;
    ///
    /// let exit_status = process::Command::new("true")
    ///     .status()
    ///     .expect("failed to run true(1)");
    /// assert!(proc_exit::Code::from_status(exit_status).is_ok());
    /// ```
    ///
    pub const fn is_ok(self) -> bool {
        self.0 == Self::SUCCESS.0
    }

    /// Determines if the provided [`std::process::ExitStatus`] was unsuccessful.
    ///
    /// Example:
    ///
    /// ```
    /// use std::process;
    ///
    /// let exit_status = process::Command::new("false")
    ///     .status()
    ///     .expect("failed to run false(1)");
    /// assert!(proc_exit::Code::from_status(exit_status).is_err());
    /// ```
    ///
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }

    pub const fn raw(self) -> i32 {
        self.0
    }
}

impl Default for Code {
    fn default() -> Self {
        // Chosen to allow `coerce().unwrap_or_default`
        Self::UNKNOWN
    }
}

/// Converts an `i32` primitive integer to an exit code.
impl From<i32> for Code {
    fn from(n: i32) -> Self {
        Self(n)
    }
}

/// Converts [`std::process::ExitStatus`] to an exit code by looking at its
/// [`ExitStatus::code()`] value.
///
/// On Unix, if the process was terminated by a fatal signal, the corresponding
/// signal exit code is returned.
///
/// [`std::process::ExitStatus`]:
/// https://doc.rust-lang.org/std/process/struct.ExitStatus.html
/// [`ExitStatus::code()`]:
/// https://doc.rust-lang.org/std/process/struct.ExitStatus.html#method.code
impl From<std::process::ExitStatus> for Code {
    fn from(status: std::process::ExitStatus) -> Self {
        let n = platform_exit_code(status).unwrap_or(Code::UNKNOWN.0);
        From::from(n)
    }
}

pub fn io_to_signal(kind: std::io::ErrorKind) -> Option<Code> {
    use std::io::ErrorKind::*;
    match kind {
        BrokenPipe => Some(Code::SIGPIPE),
        TimedOut => Some(Code::SIGALRM),
        Interrupted => Some(Code::SIGINT),
        _ => None,
    }
}

#[cfg(target_family = "unix")]
fn platform_exit_code(status: std::process::ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    status.code().or_else(|| status.signal())
}

#[cfg(not(target_family = "unix"))]
fn platform_exit_code(status: std::process::ExitStatus) -> Option<i32> {
    status.code()
}

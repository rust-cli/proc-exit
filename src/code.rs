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

    /// The command was used incorrectly, e.g. with the wrong number of
    /// arguments, a bad flag, bad syntax in a parameter, or whatever.
    pub const USAGE_ERR: Code = Code(64);

    /// The input data was incorrect in some way.  This should only be used for
    /// user’s data and not system files.
    pub const DATA_ERR: Code = Code(65);

    /// An input file (not a system file) did not exist or was not readable.
    /// This could also include errors like “No message” to a mailer (if it
    /// cared to catch it).
    pub const NO_INPUT: Code = Code(66);

    /// The user specified did not exist.  This might be used for mail addresses
    /// or remote logins.
    pub const NO_USER: Code = Code(67);

    /// The host specified did not exist.  This is used in mail addresses or
    /// network requests.
    pub const NO_HOST: Code = Code(68);

    /// A service is unavailable.  This can occur if a support program or file
    /// does not exist.  This can also be used as a catch-all message when
    /// something you wanted to do doesn’t work, but you don’t know why.
    pub const SERVICE_UNAVAILABLE: Code = Code(69);

    /// An internal software error has been detected.  This should be limited
    /// to non-operating system related errors if possible.
    pub const SOFTWARE_ERR: Code = Code(70);

    /// An operating system error has been detected.  This is intended to be
    /// used for such things as “cannot fork”, or “cannot create pipe”.  It
    /// includes things like [getuid(2)] returning a user that does not exist
    /// in the passwd file.
    ///
    /// [getuid(2)]: https://man.openbsd.org/getuid.2
    pub const OS_ERR: Code = Code(71);

    /// Some system file (e.g. _/etc/passwd_, _/var/run/utmp_) does not exist,
    /// cannot be opened, or has some sort of error (e.g. syntax error).
    pub const OS_FILE_ERR: Code = Code(72);

    /// A (user specified) output file cannot be created.
    pub const CANT_CREAT: Code = Code(73);

    /// An error occurred while doing I/O on some file.
    pub const IO_ERR: Code = Code(74);

    /// Temporary failure, indicating something that is not really an error.
    /// For example that a mailer could not create a connection, and the
    /// request should be reattempted later.
    pub const TEMP_FAIL: Code = Code(75);

    /// The remote system returned something that was “not possible” during a
    /// protocol exchange.
    pub const PROTOCOL_ERR: Code = Code(76);

    /// You did not have sufficient permission to perform the operation.  This
    /// is not intended for file system problems, which should use `NoInput` or
    /// `CantCreat`, but rather for high level permissions.
    pub const NO_PERM: Code = Code(77);

    /// Something was found in an unconfigured or misconfigured state.
    pub const CONFIG_ERR: Code = Code(78);

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

    /// Tests if the provided exit code is reserved, and has a special meaning in
    /// shells.
    #[allow(clippy::needless_bool)]
    #[allow(clippy::if_same_then_else)]
    pub const fn is_reserved(self) -> bool {
        let code = self.0;
        if Self::SUCCESS.0 <= code && code <= Self::UNKNOWN.0 {
            true
        } else if Self::USAGE_ERR.0 <= code && code <= Self::CONFIG_ERR.0 {
            true
        } else if Self::NOT_EXECUTABLE.0 <= code && code <= Self::INVALID_EXIT.0 {
            true
        } else if Self::SIGHUP.0 <= code && code <= Self::SIGTERM.0 {
            true
        } else {
            false
        }
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

impl From<std::io::ErrorKind> for Code {
    fn from(kind: std::io::ErrorKind) -> Self {
        use std::io::ErrorKind::*;
        match kind {
            NotFound => Code::OS_FILE_ERR,
            PermissionDenied => Code::NO_PERM,
            ConnectionRefused | ConnectionReset | ConnectionAborted | NotConnected => {
                Code::PROTOCOL_ERR
            }
            AddrInUse | AddrNotAvailable => Code::SERVICE_UNAVAILABLE,
            BrokenPipe => Code::SIGPIPE,
            AlreadyExists => Code::CANT_CREAT,
            InvalidInput | InvalidData | UnexpectedEof => Code::DATA_ERR,
            TimedOut => Code::SIGALRM,
            WriteZero => Code::NO_INPUT,
            Interrupted => Code::SIGINT,
            Other => Code::FAILURE,
            _ => Code::IO_ERR,
        }
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

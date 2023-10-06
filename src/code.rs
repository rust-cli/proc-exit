/// Process exit code.
///
/// Common exit codes:
/// - [`Code::SUCCESS`]
/// - [`Code::FAILURE`]
/// - [`bash::USAGE`][crate::bash::USAGE]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Code(i32);

/// # Create a [`Code`]
impl Code {
    /// The process exited successfully.
    pub const SUCCESS: Code = Code(0);

    /// Generic failure.
    pub const FAILURE: Code = Code(1);

    /// Create a custom error code
    pub const fn new(code: i32) -> Self {
        Self(code)
    }

    /// Converts [`std::process::ExitStatus`] to [`Code`].
    ///
    /// On Unix, if the process was terminated by a fatal signal, the corresponding
    /// signal exit code is returned.
    #[inline]
    pub fn from_status(status: std::process::ExitStatus) -> Self {
        Self::from(status)
    }
}

/// # Bubble up the exit [`Code`]
impl Code {
    /// [`exit`][std::process::exit] now!
    #[inline]
    pub fn process_exit(self) -> ! {
        std::process::exit(self.as_raw())
    }

    /// Convert to [`Result`]
    #[inline]
    pub fn ok(self) -> crate::ExitResult {
        if self.as_raw() == Self::SUCCESS.as_raw() {
            Ok(())
        } else {
            Err(crate::Exit::new(self))
        }
    }

    /// Convert to [`Exit`][crate::Exit] error type
    #[inline]
    pub fn as_exit(self) -> crate::Exit {
        crate::Exit::new(self)
    }

    /// Add user-visible message (like an [`Error`][std::error::Error])
    #[inline]
    pub fn with_message<D: std::fmt::Display + 'static>(self, msg: D) -> crate::Exit {
        self.as_exit().with_message(msg)
    }
}

/// # Introspection and Integration
impl Code {
    /// Convert to [`ExitCode][std::process::ExitCode]
    #[inline]
    pub fn as_exit_code(self) -> Option<std::process::ExitCode> {
        self.as_portable().map(|c| c.into())
    }

    /// Convert to raw value
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }

    /// Convert to portable, raw value
    #[inline]
    pub const fn as_portable(self) -> Option<u8> {
        if self.is_portable() {
            Some(self.as_raw() as u8)
        } else {
            None
        }
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
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.as_raw() == Self::SUCCESS.as_raw()
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
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }

    /// Test if provided exit code is portable across platforms.
    ///
    /// While Windows has wider types for return codes, Unix OS's tend to only support 8-bits,
    /// stripping off the higher order bits.
    #[inline]
    pub const fn is_portable(self) -> bool {
        0 <= self.as_raw() && self.as_raw() <= 255
    }
}

impl Default for Code {
    #[inline]
    fn default() -> Self {
        // Chosen to allow `coerce().unwrap_or_default`
        Self::FAILURE
    }
}

/// Converts an `i32` primitive integer to an exit code.
impl From<i32> for Code {
    #[inline]
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
    #[inline]
    fn from(status: std::process::ExitStatus) -> Self {
        let n = platform_exit_code(status).unwrap_or(Code::default().0);
        From::from(n)
    }
}

#[cfg(target_family = "unix")]
#[inline]
fn platform_exit_code(status: std::process::ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    status.code().or_else(|| status.signal())
}

#[cfg(not(target_family = "unix"))]
#[inline]
fn platform_exit_code(status: std::process::ExitStatus) -> Option<i32> {
    status.code()
}

impl std::process::Termination for Code {
    #[inline]
    fn report(self) -> std::process::ExitCode {
        self.as_exit_code()
            .unwrap_or(std::process::ExitCode::FAILURE)
    }
}

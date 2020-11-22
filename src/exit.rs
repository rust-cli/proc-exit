use std::io::Write;

/// Error type for exiting programs.
pub struct Exit {
    pub code: crate::Code,
    pub msg: Option<Box<dyn std::fmt::Display>>,
}

impl Exit {
    pub fn new(code: crate::Code) -> Self {
        Self {
            code: code,
            msg: None,
        }
    }

    pub fn with_message<D: std::fmt::Display + 'static>(mut self, msg: D) -> Self {
        self.msg = Some(Box::new(msg));
        self
    }
}

impl std::fmt::Display for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(msg) = self.msg.as_ref() {
            msg.fmt(f)
        } else {
            Ok(())
        }
    }
}

impl std::fmt::Debug for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // For compatibility with `std::process::Termination`
        std::fmt::Display::fmt(self, f)
    }
}

impl From<std::io::Error> for Exit {
    fn from(err: std::io::Error) -> Self {
        Self::new(err.kind().into()).with_message(err)
    }
}

/// Extension for converting errors to `Exit`.
pub trait WithCodeResultExt<T> {
    /// Convert an Error into an `Exit`
    fn with_code(self, code: crate::Code) -> Result<T, Exit>;
}

impl<T, E: std::fmt::Display + 'static> WithCodeResultExt<T> for Result<T, E> {
    fn with_code(self, code: crate::Code) -> Result<T, Exit> {
        self.map_err(|e| Exit::new(code).with_message(e))
    }
}

/// Extension for `main()` functions`.
pub trait ProcessExitResultExt {
    /// Report any error message and exit.
    fn process_exit(self) -> !;

    /// Report, delegating exiting to the caller.
    fn report(self) -> crate::Code;
}

impl ProcessExitResultExt for Result<(), Exit> {
    fn process_exit(self) -> ! {
        let code = self.report();
        code.process_exit()
    }

    fn report(self) -> crate::Code {
        match self {
            Ok(()) => crate::Code::SUCCESS,
            Err(err) => {
                if let Some(msg) = err.msg {
                    // At this point, we might be exiting due to a broken pipe, just do our best and
                    // move on.
                    let _ = writeln!(std::io::stderr(), "{}", msg);
                }
                err.code
            }
        }
    }
}

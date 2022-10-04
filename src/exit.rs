use std::io::Write;

pub type ExitResult = Result<(), Exit>;

/// Error type for exiting programs.
pub struct Exit {
    code: crate::Code,
    msg: Option<Box<dyn std::fmt::Display>>,
}

impl Exit {
    pub fn new(code: crate::Code) -> Self {
        Self { code, msg: None }
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

impl std::process::Termination for Exit {
    #[inline]
    fn report(self) -> std::process::ExitCode {
        self.code
            .as_exit_code()
            .unwrap_or(std::process::ExitCode::FAILURE)
    }
}

impl std::fmt::Debug for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // For compatibility with `std::process::Termination`
        std::fmt::Display::fmt(self, f)
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

/// Report any error message and exit.
pub fn exit(result: ExitResult) -> ! {
    let code = report(result);
    code.process_exit()
}

/// Report, delegating exiting to the caller.
pub fn report(result: ExitResult) -> crate::Code {
    match result {
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

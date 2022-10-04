//! Support for [sysexits](https://www.freebsd.org/cgi/man.cgi?query=sysexits) codes
//!
//! Note: [FreeBSD no longer encourages these](https://reviews.freebsd.org/D27176)

/// Extension for converting errors to `Exit`.
pub trait ToSysexitsResultExt<T> {
    /// Convert an Error into an `Exit`
    fn to_sysexits(self) -> Result<T, crate::Exit>;
}

impl<T> ToSysexitsResultExt<T> for Result<T, std::io::Error> {
    fn to_sysexits(self) -> Result<T, crate::Exit> {
        self.map_err(|e| {
            let kind = e.kind();
            let code = io_to_sysexists(kind)
                .or_else(|| crate::bash::io_to_signal(kind))
                .unwrap_or(IO_ERR);
            crate::Exit::new(code).with_message(e)
        })
    }
}

/// Convert [`std::io::ErrorKind`] to a [`Code`]
pub fn io_to_sysexists(kind: std::io::ErrorKind) -> Option<crate::Code> {
    use std::io::ErrorKind::*;
    match kind {
        NotFound => Some(OS_FILE_ERR),
        PermissionDenied => Some(NO_PERM),
        ConnectionRefused | ConnectionReset | ConnectionAborted | NotConnected => {
            Some(PROTOCOL_ERR)
        }
        AddrInUse | AddrNotAvailable => Some(SERVICE_UNAVAILABLE),
        AlreadyExists => Some(CANT_CREAT),
        InvalidInput | InvalidData | UnexpectedEof => Some(DATA_ERR),
        WriteZero => Some(NO_INPUT),
        _ => None,
    }
}

/// The process exited successfully.
pub const OK: crate::Code = crate::Code::new(0);

/// The command was used incorrectly, e.g. with the wrong number of
/// arguments, a bad flag, bad syntax in a parameter, or whatever.
pub const USAGE_ERR: crate::Code = crate::Code::new(64);

/// The input data was incorrect in some way.  This should only be used for
/// user’s data and not system files.
pub const DATA_ERR: crate::Code = crate::Code::new(65);

/// An input file (not a system file) did not exist or was not readable.
/// This could also include errors like “No message” to a mailer (if it
/// cared to catch it).
pub const NO_INPUT: crate::Code = crate::Code::new(66);

/// The user specified did not exist.  This might be used for mail addresses
/// or remote logins.
pub const NO_USER: crate::Code = crate::Code::new(67);

/// The host specified did not exist.  This is used in mail addresses or
/// network requests.
pub const NO_HOST: crate::Code = crate::Code::new(68);

/// A service is unavailable.  This can occur if a support program or file
/// does not exist.  This can also be used as a catch-all message when
/// something you wanted to do doesn’t work, but you don’t know why.
pub const SERVICE_UNAVAILABLE: crate::Code = crate::Code::new(69);

/// An internal software error has been detected.  This should be limited
/// to non-operating system related errors if possible.
pub const SOFTWARE_ERR: crate::Code = crate::Code::new(70);

/// An operating system error has been detected.  This is intended to be
/// used for such things as “cannot fork”, or “cannot create pipe”.  It
/// includes things like [getuid(2)] returning a user that does not exist
/// in the passwd file.
///
/// [getuid(2)]: https://man.openbsd.org/getuid.2
pub const OS_ERR: crate::Code = crate::Code::new(71);

/// Some system file (e.g. _/etc/passwd_, _/var/run/utmp_) does not exist,
/// cannot be opened, or has some sort of error (e.g. syntax error).
pub const OS_FILE_ERR: crate::Code = crate::Code::new(72);

/// A (user specified) output file cannot be created.
pub const CANT_CREAT: crate::Code = crate::Code::new(73);

/// An error occurred while doing I/O on some file.
pub const IO_ERR: crate::Code = crate::Code::new(74);

/// Temporary failure, indicating something that is not really an error.
/// For example that a mailer could not create a connection, and the
/// request should be reattempted later.
pub const TEMP_FAIL: crate::Code = crate::Code::new(75);

/// The remote system returned something that was “not possible” during a
/// protocol exchange.
pub const PROTOCOL_ERR: crate::Code = crate::Code::new(76);

/// You did not have sufficient permission to perform the operation.  This
/// is not intended for file system problems, which should use `NoInput` or
/// `CantCreat`, but rather for high level permissions.
pub const NO_PERM: crate::Code = crate::Code::new(77);

/// Something was found in an unconfigured or misconfigured state.
pub const CONFIG_ERR: crate::Code = crate::Code::new(78);

//! ## Features
//!
//! - `i32` newtype for exit codes
//!   - Can represent any valid exit code
//!   - Type safe, operations are restricted to what is valid for exit codes
//! - Includes standard exit codes and signal exit codes
//! - Integrate with `main`, `std::process`, and `std::io::Error`
//! - Supports exiting silently (error message reported through another means)
//!
//! ## Install
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! proc-exit = "1.0.1"
//! ```
//!
//! Feature flags:
//! - `portable`:  Coerce exit codes to `u8` for consistent, cross-platform, behavior
//!
//! # Example
//!
//! ```
//! fn main() {
//!     // Simple but Macro-less `main`
//!     // - Fast compiles
//!     // - Composable with other features
//!     let result = run();
//!     proc_exit::exit(result);
//! }
//!
//! fn run() -> proc_exit::ExitResult {
//!     // Integrates directly with `std::io::Error`, returning the right exit code.
//!     let exit_status = std::process::Command::new("true")
//!          .status()?;
//!     // Can pass `Command` exit codes right up, when appropriate
//!     proc_exit::Code::from_status(exit_status).ok()?;
//!
//!     proc_exit::Code::SUCCESS.ok()
//! }
//! ```
//!
//! ## Relevant crates
//!
//! Other crates that might be useful in testing command line programs.
//! - [duct][duct] for orchestrating multiple processes.
//!   - or [commandspec][commandspec] for easier writing of commands
//! - [rexpect][rexpect] for controlling interactive programs.
//! - [`assert_cmd`][assert_cmd] can be reused to simplify controlling CLIs
//!
//! [duct]: https://crates.io/crates/duct
//! [rexpect]: https://crates.io/crates/rexpect
//! [assert_cmd]: https://crates.io/crates/assert_cmd
//! [commandspec]: https://crates.io/crates/commandspec
//!
//! ## Related crates
//!
//! Some crates that fill a similar role include:
//! - [sysexit][sysexit]
//!   - Uses an enum, making certain states unpresentable
//!   - Includes signals
//!   - Integrates with `std::process` and `std::io::Error`
//!   - Doesn't integrate with `main`
//! - [exit-code][exit-code]
//!   - `i32` constants and helper methods
//!   - Doesn't include signals
//!   - Doesn't integrate with `main`, `std::process`, or `std::io::Error`
//! - [exitcode][exitcode]
//!   - `i32` constants and helper methods
//!   - Doesn't include signals
//!   - Doesn't integrate with `main`, `std::process`, or `std::io::Error`
//! - [exitfailure][exitfailure]
//!   - Allows `Display`able errors to be used with [`?` in `main()`](https://github.com/rust-lang/rust/issues/43301)
//!
//! [sysexit]: https://crates.io/crates/sysexit
//! [exit-code]: https://crates.io/crates/exit-code
//! [exitcode]: https://crates.io/crates/exitcode
//! [exitfailure]: https://crates.io/crates/exitfailure
//!
//! ## References
//!
//! As a basis it encodes the exit codes of [sysexits(3)][sysexits] from OpenBSD (64-78), exit statuses used by [bash][appendix-e],
//! supplemented by codes created by shells when the command is terminated
//! by a fatal signal.  When the fatal signal is a number _N_, the latter
//! follows bash's strategy of using the value 128 + _N_ as the exit status.
//! This means that the `SIGHUP` (1) signal will be recognised as the exit code
//! for the number 129.  Signal codes were taken from [wikipedia](https://en.wikipedia.org/wiki/Signal_(IPC)#SIGALRM)
//!
//! [sysexits]: http://tldp.org/LDP/abs/html/exitcodes.html
//! [appendix-e]: http://tldp.org/LDP/abs/html/exitcodes.html

mod code;
mod exit;

pub use code::Code;
pub use exit::WithCodeResultExt;
pub use exit::{exit, report};
pub use exit::{Exit, ExitResult};

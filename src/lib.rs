//! ## Features
//!
//! - `i32` newtype for exit codes
//!   - Can represent any valid exit code
//!   - Type safe, operations are restricted to what is valid for exit codes
//! - Includes common exit codes and signal exit codes
//! - Integrate with `main`, `std::process`, and `std::io::Error`
//! - Supports exiting silently (error message reported through another means)
//!
//! ## Install
//!
//! Add to your `Cargo.toml`:
//!
//! ```console
//! $ cargo add proc-exit
//! ```
//!
//! # Example
//!
//! ```
//! use proc_exit::prelude::*;
//!
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
//!          .status().with_code(proc_exit::Code::FAILURE)?;
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
//!   - Uses an enum, making certain states unrepresentable
//!   - Includes signals
//!   - Integrates with `std::process` and `std::io::Error`
//!   - Doesn't integrate with `main`
//! - [sysexits][sysexits]
//!   - Only includes exit codes
//!   - Uses an exit code tagged enum which can be converted to [`std::process:ExitCode`]
//! - [exitfailure][exitfailure]
//!   - Allows `Display`able errors to be used with [`?` in `main()`](https://github.com/rust-lang/rust/issues/43301)
//!
//! [sysexit]: https://crates.io/crates/sysexit
//! [sysexits]: https://crates.io/crates/sysexits
//! [exitfailure]: https://crates.io/crates/exitfailure

mod code;
mod exit;

/// Easy access to traits
pub mod prelude {
    pub use super::WithCodeResultExt as _;
    pub use crate::sysexits::ToSysexitsResultExt as _;
}

pub mod bash;
pub mod sysexits;

pub use code::Code;
pub use exit::WithCodeResultExt;
pub use exit::{exit, report};
pub use exit::{Exit, ExitResult};

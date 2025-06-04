# proc-exit

> Exit codes for process termination

[![codecov](https://codecov.io/gh/rust-cli/proc-exit/branch/master/graph/badge.svg)](https://codecov.io/gh/rust-cli/proc-exit)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/proc-exit.svg)
[![Crates Status](https://img.shields.io/crates/v/proc-exit.svg)][Crates.io]

## Features

- `i32` newtype for exit codes
  - Can represent any valid exit code
  - Type safe, operations are restricted to what is valid for exit codes
- Includes standard exit codes and signal exit codes
- Integrate with `main`, `std::process`, and `std::io::Error`
- Supports exiting silently (error message reported through another means)

## Install

Add to your `Cargo.toml`:

```console
$ cargo add proc-exit
```

## Relevant crates

Other crates that might be useful in testing command line programs.
- [duct][duct] for orchestrating multiple processes.
  - or [commandspec][commandspec] for easier writing of commands
- [rexpect][rexpect] for controlling interactive programs.
- [`assert_cmd`][assert_cmd] can be reused to simplify controlling CLIs

[duct]: https://crates.io/crates/duct
[rexpect]: https://crates.io/crates/rexpect
[assert_cmd]: https://crates.io/crates/assert_cmd
[commandspec]: https://crates.io/crates/commandspec

## Related crates

Some crates that fill a similar role include:
- [sysexit][sysexit]
  - Uses an enum, making certain states unpresentable
  - Includes signals
  - Integrates with `std::process` and `std::io::Error`
  - Doesn't integrate with `main`
- [exit-code][exit-code]
  - `i32` constants and helper methods
  - Doesn't include signals
  - Doesn't integrate with `main`, `std::process`, or `std::io::Error`
- [exitcode][exitcode]
  - `i32` constants and helper methods
  - Doesn't include signals
  - Doesn't integrate with `main`, `std::process`, or `std::io::Error`
- [exitfailure][exitfailure]
  - Allows `Display`able errors to be used with [`?` in `main()`](https://github.com/rust-lang/rust/issues/43301)

[sysexit]: https://crates.io/crates/sysexit
[exit-code]: https://crates.io/crates/exit-code
[exitcode]: https://crates.io/crates/exitcode
[exitfailure]: https://crates.io/crates/exitfailure

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.

[Crates.io]: https://crates.io/crates/proc-exit
[Documentation]: https://docs.rs/proc-exit

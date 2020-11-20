# term-code

> Exit codes for process termination

[![Build Status](https://dev.azure.com/crate-ci/crate-ci/_apis/build/status/term-code?branchName=master)](https://dev.azure.com/crate-ci/crate-ci/_build/latest?definitionId=6&branchName=master)
[![codecov](https://codecov.io/gh/assert-rs/term-code/branch/master/graph/badge.svg)](https://codecov.io/gh/assert-rs/term-code)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/term-code.svg)
[![Crates Status](https://img.shields.io/crates/v/term-code.svg)](https://crates.io/crates/term-code)

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
term-code = "0.1"
```

## Relevant crates

Other crates that might be useful in testing command line programs.
* [escargot][escargot] for more control over configuring the crate's binary.
* [duct][duct] for orchestrating multiple processes.
  * or [commandspec] for easier writing of commands
* [rexpect][rexpect] for testing interactive programs.
* [`assert_cmd`][assert_cmd] for CLI fixtures and assertions.
  * or [tempfile][tempfile] for scratchpad directories.
* [dir-diff][dir-diff] for testing file side-effects.

[escargot]: http://docs.rs/escargot
[rexpect]: https://crates.io/crates/rexpect
[dir-diff]: https://crates.io/crates/dir-diff
[tempfile]: https://crates.io/crates/tempfile
[duct]: https://crates.io/crates/duct
[assert_cmd]: https://crates.io/crates/assert_cmd
[commandspec]: https://crates.io/crates/commandspec

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[Crates.io]: https://crates.io/crates/term-code
[Documentation]: https://docs.rs/term-code

# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [2.0.2] - 2024-07-25

### Compatibility

- Updated MSRV to 1.74

## [2.0.1] - 2023-03-14

### Performance

- Reduce code bloat

## [2.0.0] - 2022-10-04

### Breaking Changes

- sysexits codes moved from `Code::*` to `sysexits::*`
- bash codes moved from `Code::*` to `bash::*`
- `ErrorKind` conversion requires `ToSysexitsResultExt::to_sysexits`
- Removed `Display` for `Code`
- Removed `is_reserved`
- Changed default code to `FAILURE`
- Renamed `Code::raw` to `Code::as_raw`
- Removed `portable` feature forcing portable values when calling `std::process::exit`

### Compatibility

MSRV is now 1.61.0 to integrate with `Termination`

### Features

- Conversion from `Code` to `std::process::ExitCode`
- `Termination` support

## [1.0.3] - 2021-12-28

### Fixes

- Moved github orgs

## [1.0.2] - 2021-01-29

## [1.0.1] - 2020-11-23

## [1.0.0] - 2020-11-23

## [0.3.0] - 2020-11-23

### Fixes

- Skip the extension trait, make `proc_exit::exit` the tool for `main`.

## [0.2.0] - 2020-11-21

### Fixes

- Typos in documentation
- `Exit`s members should not be public

<!-- next-url -->
[Unreleased]: https://github.com/rust-cli/proc-exit/compare/v2.0.2...HEAD
[2.0.2]: https://github.com/rust-cli/proc-exit/compare/v2.0.1...v2.0.2
[2.0.1]: https://github.com/rust-cli/proc-exit/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/rust-cli/proc-exit/compare/v1.0.3...v2.0.0
[1.0.3]: https://github.com/rust-cli/proc-exit/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/rust-cli/proc-exit/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/rust-cli/proc-exit/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/rust-cli/proc-exit/compare/v0.3.0...v1.0.0
[0.3.0]: https://github.com/rust-cli/proc-exit/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/rust-cli/proc-exit/compare/v0.1.0...v0.2.0

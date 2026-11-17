# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.2.0] - 2026-11-17

### Changed

* **Breaking:** Replace [`chrono`] with [`jiff`] for the `date` feature because
  `chrono` has been [soft-deprecated][chrono-soft-deprecated] and marked as
  unmaintained in [RUSTSEC-2026-0543].

  `OsReleaseEntry::value_as_date`, `OsRelease::get_value_as_date`, and
  `OsRelease::support_end` now return `jiff::civil::Date` instead of
  `chrono::NaiveDate`, and `jiff::Error` instead of `chrono::ParseError`.
* Bump MSRV from 1.88.0 to 1.94.0
* Adopt [Trusted Publishing] for releases on crates.io.

[`chrono`]: https://crates.io/crates/chrono
[`jiff`]: https://crates.io/crates/jiff
[chrono-soft-deprecated]: https://github.com/chronotope/chrono/issues/1768
[RUSTSEC-2026-0543]: https://rustsec.org/advisories/RUSTSEC-2026-0543.html
[Trusted Publishing]: https://crates.io/docs/trusted-publishing

## [0.1.1] - 2024-10-04

### Fixed

* Fix broken links in crate documentation

## [0.1.0] - 2023-10-04

* First release

<!-- next-url -->
[Unreleased]: https://github.com/gifnksm/etc-os-release/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/gifnksm/etc-os-release/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/gifnksm/etc-os-release/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/gifnksm/etc-os-release/commits/v0.1.0

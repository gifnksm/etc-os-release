# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Changed

* **Breaking:** Replace [`chrono`] with [`jiff`] for the `date` feature because `chrono` has been [soft-deprecated][chrono-soft-deprecated] and marked as unmaintained in [RUSTSEC-2026-0543].

  `OsReleaseEntry::value_as_date`, `OsRelease::get_value_as_date`, and `OsRelease::support_end` now return `jiff::civil::Date` instead of `chrono::NaiveDate`, and `jiff::Error` instead of `chrono::ParseError`.

[`chrono`]: https://crates.io/crates/chrono
[`jiff`]: https://crates.io/crates/jiff
[chrono-soft-deprecated]: https://github.com/chronotope/chrono/issues/1768
[RUSTSEC-2026-0543]: https://rustsec.org/advisories/RUSTSEC-2026-0543.html

## [0.1.1] - 2024-10-04

### Fixed

* Fix broken links in crate documentation

## [0.1.0] - 2023-10-04

* First release

<!-- next-url -->
[Unreleased]: https://github.com/gifnksm/etc-os-release/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/gifnksm/etc-os-release/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/gifnksm/etc-os-release/commits/v0.1.0

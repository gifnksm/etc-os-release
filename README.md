<!-- cargo-sync-rdme title [[ -->
# etc-os-release
<!-- cargo-sync-rdme ]] -->
<!-- cargo-sync-rdme badge [[ -->
[![Maintenance: passively-maintained](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg?style=flat-square)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/etc-os-release.svg?style=flat-square)](#license)
[![crates.io](https://img.shields.io/crates/v/etc-os-release.svg?logo=rust&style=flat-square)](https://crates.io/crates/etc-os-release)
[![docs.rs](https://img.shields.io/docsrs/etc-os-release.svg?logo=docs.rs&style=flat-square)](https://docs.rs/etc-os-release)
[![Rust: ^1.82.0](https://img.shields.io/badge/rust-^1.82.0-93450a.svg?logo=rust&style=flat-square)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
[![GitHub Actions: CI](https://img.shields.io/github/actions/workflow/status/gifnksm/etc-os-release/ci.yml.svg?label=CI&logo=github&style=flat-square)](https://github.com/gifnksm/etc-os-release/actions/workflows/ci.yml)
[![Codecov](https://img.shields.io/codecov/c/github/gifnksm/etc-os-release.svg?label=codecov&logo=codecov&style=flat-square)](https://codecov.io/gh/gifnksm/etc-os-release)
<!-- cargo-sync-rdme ]] -->

<!-- cargo-sync-rdme rustdoc [[ -->
A parser and data structures for the `/etc/os-release` file.

os-release file is used by systemd and other tools to store information about the
operating system distribution.

The file is formatted as a list of environment-like shell-compatible
variable assignments.

For more information, see [`os-release(5)`]

## Usage

Add this to your `Cargo.toml`:

````toml
[dependencies]
etc-os-release = "0.1.1"
````

## Examples

Open the os-release file and print the OS name and version:

````rust,no_run
use etc_os_release::OsRelease;

let os_release = OsRelease::open()?;
println!("{}-{}", os_release.id(), os_release.version_id().unwrap_or_default());
````

Parse a string containing the contents of the os-release file:

````rust
use std::str::FromStr;

use etc_os_release::OsRelease;

let os_release = OsRelease::from_str(r#"
NAME=Fedora
VERSION="32 (Workstation Edition)"
ID=fedora
VERSION_ID=32
PRETTY_NAME="Fedora 32 (Workstation Edition)"
ANSI_COLOR="0;38;2;60;110;180"
LOGO=fedora-logo-icon
CPE_NAME="cpe:/o:fedoraproject:fedora:32"
HOME_URL="https://fedoraproject.org/"
DOCUMENTATION_URL="https://docs.fedoraproject.org/en-US/fedora/f32/system-administrators-guide/"
SUPPORT_URL="https://fedoraproject.org/wiki/Communicating_and_getting_help"
BUG_REPORT_URL="https://bugzilla.redhat.com/"
REDHAT_BUGZILLA_PRODUCT="Fedora"
REDHAT_BUGZILLA_PRODUCT_VERSION=32
REDHAT_SUPPORT_PRODUCT="Fedora"
REDHAT_SUPPORT_PRODUCT_VERSION=32
PRIVACY_POLICY_URL="https://fedoraproject.org/wiki/Legal:PrivacyPolicy"
VARIANT="Workstation Edition"
VARIANT_ID=workstation
"#).unwrap();

assert_eq!(os_release.id(), "fedora");
assert_eq!(os_release.version_id(), Some("32"));
````

[`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
<!-- cargo-sync-rdme ]] -->

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust 1.82.0**.
At least the last 3 versions of stable Rust are supported at any given time.

While a crate is a pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
Once a crate has reached 1.x, any MSRV bump will be accompanied by a new minor version.

## License

This project is licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

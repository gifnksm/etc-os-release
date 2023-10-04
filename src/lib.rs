//! A parser and data structures for the `/etc/os-release` file.
//!
//! os-release file is used by systemd and other tools to store information about the
//! operating system distribution.
//!
//! The file is formatted as a list of environment-like shell-compatible
//! variable assignments.
//!
//! For more information, see [`os-release(5)`]
//!
//! [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! etc-os-release = "0.0.0"
//! ```
//!
//! # Examples
//!
//! Open the os-release file and print the OS name and version:
//!
//! ```rust,no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use etc_os_release::OsRelease;
//!
//! let os_release = OsRelease::open()?;
//! println!("{}-{}", os_release.id(), os_release.version_id().unwrap_or_default());
//! # Ok(())
//! # }
//! ```
//!
//! Parse a string containing the contents of the os-release file:
//!
//! ```rust
//! use std::str::FromStr;
//!
//! use etc_os_release::OsRelease;
//!
//! let os_release = OsRelease::from_str(r#"
//! NAME=Fedora
//! VERSION="32 (Workstation Edition)"
//! ID=fedora
//! VERSION_ID=32
//! PRETTY_NAME="Fedora 32 (Workstation Edition)"
//! ANSI_COLOR="0;38;2;60;110;180"
//! LOGO=fedora-logo-icon
//! CPE_NAME="cpe:/o:fedoraproject:fedora:32"
//! HOME_URL="https://fedoraproject.org/"
//! DOCUMENTATION_URL="https://docs.fedoraproject.org/en-US/fedora/f32/system-administrators-guide/"
//! SUPPORT_URL="https://fedoraproject.org/wiki/Communicating_and_getting_help"
//! BUG_REPORT_URL="https://bugzilla.redhat.com/"
//! REDHAT_BUGZILLA_PRODUCT="Fedora"
//! REDHAT_BUGZILLA_PRODUCT_VERSION=32
//! REDHAT_SUPPORT_PRODUCT="Fedora"
//! REDHAT_SUPPORT_PRODUCT_VERSION=32
//! PRIVACY_POLICY_URL="https://fedoraproject.org/wiki/Legal:PrivacyPolicy"
//! VARIANT="Workstation Edition"
//! VARIANT_ID=workstation
//! "#).unwrap();
//!
//! assert_eq!(os_release.id(), "fedora");
//! assert_eq!(os_release.version_id(), Some("32"));
//! ```

#![doc(html_root_url = "https://docs.rs/etc-os-release/0.0.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs, unreachable_pub)]

use indexmap::IndexMap;

pub use crate::{
    construct::Error,
    entry::{OsReleaseEntry, OsReleaseLine},
};

mod construct;
mod entry;
mod fields;

/// The parsed contents of the os-release file.
///
/// This structure is a map of the fields in the os-release file.
///
/// For more information, see [`os-release(5)`].
///
/// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
///
/// # Notes
///
/// If you are using this crate to determine the OS or a specific version of it, use the [`Self::id()`]
/// and [`Self::version_id()`] methods, possibly with [`Self::id_like()`] as fallback for [`Self::id()`].
/// When looking for an OS identification string for presentation to the user, use the [`Self::pretty_name()`] method.
///
/// Note that operating system vendors may choose not to provide version information, for example to accommodate for rolling releases.
/// In this case, [`Self::version()`] and [`Self::version_id()`] may be `None`.
/// Application should not rely on these fields to be set.
///
/// # Examples
///
/// Open the os-release file and print the OS name and version:
///
/// ```rust,no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use etc_os_release::OsRelease;
///
/// let os_release = OsRelease::open()?;
/// println!("{}-{}", os_release.id(), os_release.version_id().unwrap_or_default());
/// # Ok(())
/// # }
/// ```
///
/// Parse a string containing the contents of the os-release file:
///
/// ```rust
/// use std::str::FromStr;
///
/// use etc_os_release::OsRelease;
///
/// let os_release = OsRelease::from_str(r#"
/// NAME=Fedora
/// VERSION="32 (Workstation Edition)"
/// ID=fedora
/// VERSION_ID=32
/// PRETTY_NAME="Fedora 32 (Workstation Edition)"
/// ANSI_COLOR="0;38;2;60;110;180"
/// LOGO=fedora-logo-icon
/// CPE_NAME="cpe:/o:fedoraproject:fedora:32"
/// HOME_URL="https://fedoraproject.org/"
/// DOCUMENTATION_URL="https://docs.fedoraproject.org/en-US/fedora/f32/system-administrators-guide/"
/// SUPPORT_URL="https://fedoraproject.org/wiki/Communicating_and_getting_help"
/// BUG_REPORT_URL="https://bugzilla.redhat.com/"
/// REDHAT_BUGZILLA_PRODUCT="Fedora"
/// REDHAT_BUGZILLA_PRODUCT_VERSION=32
/// REDHAT_SUPPORT_PRODUCT="Fedora"
/// REDHAT_SUPPORT_PRODUCT_VERSION=32
/// PRIVACY_POLICY_URL="https://fedoraproject.org/wiki/Legal:PrivacyPolicy"
/// VARIANT="Workstation Edition"
/// VARIANT_ID=workstation
/// "#).unwrap();
///
/// assert_eq!(os_release.id(), "fedora");
/// assert_eq!(os_release.version_id(), Some("32"));
/// ```
#[derive(Debug, Clone)]
pub struct OsRelease {
    // Use `IndexMap` for reserving insertion order.
    fields: IndexMap<String, String>,
}

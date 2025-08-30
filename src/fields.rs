#[cfg(feature = "date")]
use chrono::NaiveDate;
#[cfg(feature = "url")]
use url::Url;

use crate::{OsRelease, OsReleaseEntry};

/// Methods to get any field in the os-release file.
impl OsRelease {
    /// Returns the iterator over the fields in the os-release file.
    pub fn entries(&'_ self) -> impl Iterator<Item = OsReleaseEntry<'_>> {
        self.fields.iter().map(|(k, v)| OsReleaseEntry::new(k, v))
    }

    /// Returns the value of a field in the os-release file.
    pub fn get_value(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(String::as_str)
    }

    /// Returns the value of a field in the os-release as a list of strings.
    pub fn get_value_as_list(&self, key: &str) -> Option<impl Iterator<Item = &str>> {
        self.get_value(key).map(|value| value.split_whitespace())
    }

    /// Returns the value of a field in the os-release as a URL.
    #[cfg(feature = "url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "url")))]
    pub fn get_value_as_url(&self, key: &str) -> Result<Option<Url>, url::ParseError> {
        self.get_value(key).map(Url::parse).transpose()
    }

    /// Returns the value of a field in the os-release as a date.
    #[cfg(feature = "date")]
    #[cfg_attr(docsrs, doc(cfg(feature = "date")))]
    pub fn get_value_as_date(&self, key: &str) -> Result<Option<NaiveDate>, chrono::ParseError> {
        self.get_value(key)
            .map(|date| NaiveDate::parse_from_str(date, "%Y-%m-%d"))
            .transpose()
    }
}

/// Methods to get general information identifying the operating system.
///
/// For more information, see the [General information identifying the operating system][detail] section of [`os-release(5)`].
///
/// [detail]: https://www.freedesktop.org/software/systemd/man/os-release.html#General%20information%20identifying%20the%20operating%20system
/// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
impl OsRelease {
    /// Returns the string identifying the operating system, without a version component.
    ///
    /// This field is suitable for presentation to the user.
    ///
    /// If not set in the os-release file, defaults to `linux`.
    ///
    /// For more information, see the [`NAME=`] section of [`os-release(5)`]
    ///
    /// [`NAME=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#NAME=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn name(&self) -> &str {
        self.get_value("NAME").unwrap_or("linux")
    }

    /// Returns the lower-case string identifying the operating system, excluding any version information.
    ///
    /// This field is suitable for processing by scripts or usage in generated filenames.
    ///
    /// If not set in the os-release file, defaults to `linux`.
    ///
    /// For more information, see the [`ID=`] section of [`os-release(5)`]
    ///
    /// [`ID=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#ID=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn id(&self) -> &str {
        self.get_value("ID").unwrap_or("linux")
    }

    /// Returns the list of operating system identifiers.
    ///
    /// The list contains operating systems that are closely related to the local operating system in regards to packaging and programming interfaces.
    /// For example, the operating system that the local operating system is a derivative from.
    ///
    /// For more information, see the [`ID_LIKE=`] section of [`os-release(5)`]
    ///
    /// [`ID_LIKE=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#ID_LIKE=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn id_like(&self) -> Option<impl Iterator<Item = &str>> {
        self.get_value_as_list("ID_LIKE")
    }

    /// Returns the pretty operating system name in a format suitable for presentation to the user.
    ///
    /// If not set in the os-release file, defaults to `Linux`.
    ///
    /// For more information, see the [`PRETTY_NAME=`] section of [`os-release(5)`]
    ///
    /// [`PRETTY_NAME=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#PRETTY_NAME=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn pretty_name(&self) -> &str {
        self.get_value("PRETTY_NAME").unwrap_or("Linux")
    }

    /// Returns the CPE name for the operating system in URI binding syntax.
    ///
    /// The name follows the [Common Platform Enumeration Specification] as proposed by the NIST.
    ///
    /// For more information, see the [`CPE_NAME=`] section of [`os-release(5)`]
    ///
    /// [Common Platform Enumeration Specification]: http://scap.nist.gov/specifications/cpe/
    /// [`CPE_NAME=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#CPE_NAME=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn cpe_name(&self) -> Option<&str> {
        self.get_value("CPE_NAME")
    }

    /// Returns the string identifying a specific variant or edition of the operating system.
    ///
    /// This field is suitable for presentation to the user.
    ///
    /// For more information, see the [`VARIANT=`] section of [`os-release(5)`]
    ///
    /// # Note
    ///
    /// This field is for display purposes only. The [`Self::variant_id()`] field should be used for making programmatic decisions.
    ///
    /// [`VARIANT=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#VARIANT=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn variant(&self) -> Option<&str> {
        self.get_value("VARIANT")
    }

    /// Returns the lower-case string identifying a specific variant or edition of the operating system.
    ///
    /// For more information, see the [`VARIANT_ID=`] section of [`os-release(5)`]
    ///
    /// [`VARIANT_ID=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#VARIANT_ID=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn variant_id(&self) -> Option<&str> {
        self.get_value("VARIANT_ID")
    }
}

/// Methods to get information about the version of the operating system.
///
/// For more information, see the [Information about the version of the operating system][detail] section of [`os-release(5)`].
///
/// [detail]: https://www.freedesktop.org/software/systemd/man/os-release.html#Information%20about%20the%20version%20of%20the%20operating%20system
/// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
impl OsRelease {
    /// Returns the string identifying the operating system version, excluding any OS name information.
    ///
    /// This field possibly includes a release code name.
    ///
    /// This field is suitable for presentation to the user.
    ///
    /// For more information, see the [`VERSION=`] section of [`os-release(5)`]
    ///
    /// [`VERSION=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#VERSION=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn version(&self) -> Option<&str> {
        self.get_value("VERSION")
    }

    /// Returns the lower-case string identifying the operating system version, excluding any OS name information or release code name.
    ///
    /// This field is suitable for processing by scripts or usage in generated filenames.
    ///
    /// For more information, see the [`VERSION_ID=`] section of [`os-release(5)`]
    ///
    /// [`VERSION_ID=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#VERSION_ID=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn version_id(&self) -> Option<&str> {
        self.get_value("VERSION_ID")
    }

    /// Returns the string identifying the operating system release code name, excluding any OS name information or release code name.
    ///
    /// This field is suitable for processing by scripts or usage in generated filenames.
    ///
    /// For more information, see the [`VERSION_CODENAME=`] section of [`os-release(5)`]
    ///
    /// [`VERSION_CODENAME=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#VERSION_CODENAME=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn version_codename(&self) -> Option<&str> {
        self.get_value("VERSION_CODENAME")
    }

    /// Returns the string uniquely identifying the system image originally used as the installation base.
    ///
    /// For more information, see the [`BUILD_ID=`] section of [`os-release(5)`]
    ///
    /// [`BUILD_ID=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#BUILD_ID=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn build_id(&self) -> Option<&str> {
        self.get_value("BUILD_ID")
    }

    /// Returns the lower-case string identifying a specific image of the operating system.
    ///
    /// For more information, see the [`IMAGE_ID=`] section of [`os-release(5)`]
    ///
    /// [`IMAGE_ID=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#IMAGE_ID=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn image_id(&self) -> Option<&str> {
        self.get_value("IMAGE_ID")
    }

    /// Return the lower-case string identifying the OS image version.
    ///
    /// For more information, see the [`IMAGE_VERSION=`] section of [`os-release(5)`]
    ///
    /// [`IMAGE_VERSION=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#IMAGE_VERSION=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn image_version(&self) -> Option<&str> {
        self.get_value("IMAGE_VERSION")
    }
}

/// Methods to get presentation information and links.
///
/// For more information, see the [Presentation information and links][detail] section of [`os-release(5)`].
///
/// [detail]: https://www.freedesktop.org/software/systemd/man/os-release.html#Presentation%20information%20and%20links
/// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
impl OsRelease {
    /// Returns the URL of the homepage of the operating system, or alternatively some homepage of the specific version of the operating system.
    ///
    /// For more information, see the [`HOME_URL=`] section of [`os-release(5)`]
    ///
    /// [`HOME_URL=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#HOME_URL=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    #[cfg(feature = "url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "url")))]
    pub fn home_url(&self) -> Result<Option<Url>, url::ParseError> {
        self.get_value_as_url("HOME_URL")
    }

    /// Returns the URL of the main documentation page of the operating system.
    ///
    /// For more information, see the [`HOME_URL=`] section of [`os-release(5)`]
    ///
    /// [`HOME_URL=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#HOME_URL=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    #[cfg(feature = "url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "url")))]
    pub fn documentation_url(&self) -> Result<Option<Url>, url::ParseError> {
        self.get_value_as_url("DOCUMENTATION_URL")
    }

    /// Returns the URL of the main support page for the operating system.
    ///
    /// For more information, see the [`HOME_URL=`] section of [`os-release(5)`]
    ///
    /// [`HOME_URL=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#HOME_URL=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    #[cfg(feature = "url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "url")))]
    pub fn support_url(&self) -> Result<Option<Url>, url::ParseError> {
        self.get_value_as_url("SUPPORT_URL")
    }

    /// Returns the main bug reporting page for the operating system.
    ///
    /// For more information, see the [`HOME_URL=`] section of [`os-release(5)`]
    ///
    /// [`HOME_URL=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#HOME_URL=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    #[cfg(feature = "url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "url")))]
    pub fn bug_report_url(&self) -> Result<Option<Url>, url::ParseError> {
        self.get_value_as_url("BUG_REPORT_URL")
    }

    /// Returns the main privacy policy page for the operating system.
    ///
    /// For more information, see the [`HOME_URL=`] section of [`os-release(5)`]
    ///
    /// [`HOME_URL=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#HOME_URL=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    #[cfg(feature = "url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "url")))]
    pub fn privacy_policy_url(&self) -> Result<Option<Url>, url::ParseError> {
        self.get_value_as_url("PRIVACY_POLICY_URL")
    }

    /// Returns the date at which support for this version of the OS ends.
    ///
    /// For more information, see the [`SUPPORT_END=`] section of [`os-release(5)`]
    ///
    /// [`SUPPORT_END=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#SUPPORT_END=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    #[cfg(feature = "date")]
    #[cfg_attr(docsrs, doc(cfg(feature = "date")))]
    pub fn support_end(&self) -> Result<Option<NaiveDate>, chrono::ParseError> {
        self.get_value_as_date("SUPPORT_END")
    }

    /// Returns the logo string, specifying the name of an icon as defined by [freedesktop.org Icon Theme Specification][spec].
    ///
    /// For more information, see the [`LOGO=`] section of [`os-release(5)`]
    ///
    /// [spec]: https://standards.freedesktop.org/icon-theme-spec/latest
    /// [`LOGO=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#LOGO=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn logo(&self) -> Option<&str> {
        self.get_value("LOGO")
    }

    /// Returns the suggested presentation color when showing the OS name on the console.
    ///
    /// For more information, see the [`ANSI_COLOR=`] section of [`os-release(5)`]
    ///
    /// [`ANSI_COLOR=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#ANSI_COLOR=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn ansi_color(&self) -> Option<&str> {
        self.get_value("ANSI_COLOR")
    }

    /// Returns the name of the OS vendor.
    ///
    /// For more information, see the [`VENDOR_NAME=`] section of [`os-release(5)`]
    ///
    /// [`VENDOR_NAME=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#VENDOR_NAME=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn vendor_name(&self) -> Option<&str> {
        self.get_value("VENDOR_NAME")
    }

    /// Returns the homepage of the OS vendor.
    ///
    /// For more information, see the [`VENDOR_URL=`] section of [`os-release(5)`]
    ///
    /// [`VENDOR_URL=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#VENDOR_URL=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    #[cfg(feature = "url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "url")))]
    pub fn vendor_url(&self) -> Result<Option<Url>, url::ParseError> {
        self.get_value_as_url("VENDOR_URL")
    }
}

/// Methods to get distribution-level defaults and metadata.
///
/// For more information, see the [Distribution-level defaults and metadata][detail] section of [`os-release(5)`].
///
/// [detail]: https://www.freedesktop.org/software/systemd/man/os-release.html#Distribution-level%20defaults%20and%20metadata
/// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
impl OsRelease {
    /// Returns the string specifying the hostname if [`hostname(5)`] is not present and no other configuration source specifies the hostname.
    ///
    /// For more information, see the [`DEFAULT_HOSTNAME=`] section of [`os-release(5)`]
    ///
    /// [`hostname(5)`]: https://www.freedesktop.org/software/systemd/man/hostname.html
    /// [`DEFAULT_HOSTNAME=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#DEFAULT_HOSTNAME=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn default_hostname(&self) -> Option<&str> {
        self.get_value("DEFAULT_HOSTNAME")
    }

    /// Returns the string that specifies which CPU architecture the userspace binaries require.
    ///
    /// For more information, see the [`ARCHITECTURE=`] section of [`os-release(5)`]
    ///
    /// [`ARCHITECTURE=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#ARCHITECTURE=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn architecture(&self) -> Option<&str> {
        self.get_value("ARCHITECTURE")
    }

    /// Returns the lower-case string identifying the operating system extensions support level, to indicate which extension images are supported.
    ///
    /// For more information, see the [`SYSEXT_LEVEL=`] section of [`os-release(5)`]
    ///
    /// [`SYSEXT_LEVEL=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#SYSEXT_LEVEL=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn sysext_level(&self) -> Option<&str> {
        self.get_value("SYSEXT_LEVEL")
    }

    /// Returns the lower-case string identifying the operating system confext support level, to indicate which confext images are supported.
    ///
    /// For more information, see the [`CONFEXT_LEVEL=`] section of [`os-release(5)`]
    ///
    /// [`CONFEXT_LEVEL=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#CONFEXT_LEVEL=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn confext_level(&self) -> Option<&str> {
        self.get_value("CONFEXT_LEVEL")
    }

    /// Returns the list of one or more of the strings `"system"`, `"initrd"` and `"portable"`.
    ///
    /// For more information, see the [`SYSEXT_SCOPE=`] section of [`os-release(5)`]
    ///
    /// [`SYSEXT_SCOPE=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#SYSEXT_SCOPE=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn sysext_scope(&self) -> Option<impl Iterator<Item = &str>> {
        self.get_value_as_list("SYSEXT_SCOPE")
    }

    /// Returns the list of one or more of the strings `"system"`, `"initrd"` and `"portable"`.
    ///
    /// For more information, see the [`CONFEXT_SCOPE=`] section of [`os-release(5)`]
    ///
    /// [`CONFEXT_SCOPE=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#CONFEXT_SCOPE=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn confext_scope(&self) -> Option<impl Iterator<Item = &str>> {
        self.get_value_as_list("CONFEXT_SCOPE")
    }

    /// Returns the list of one or more valid prefix match strings for the [Portable Services Documentation] logic.
    ///
    /// For more information, see the [`PORTABLE_PREFIXES=`] section of [`os-release(5)`]
    ///
    /// [Portable Services Documentation]: https://systemd.io/PORTABLE_SERVICES
    /// [`PORTABLE_PREFIXES=`]: https://www.freedesktop.org/software/systemd/man/os-release.html#PORTABLE_PREFIXES=
    /// [`os-release(5)`]: https://www.freedesktop.org/software/systemd/man/os-release.html
    pub fn portable_prefixes(&self) -> Option<impl Iterator<Item = &str>> {
        self.get_value_as_list("PORTABLE_PREFIXES")
    }
}

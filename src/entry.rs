use std::{borrow::Cow, convert::Infallible, str::FromStr};

#[cfg(feature = "date")]
use chrono::NaiveDate;
#[cfg(feature = "url")]
use url::Url;

/// An entry in the os-release file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OsReleaseEntry<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> OsReleaseEntry<'a> {
    /// Create a new `OsReleaseEntry`.
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        let key = key.into();
        let value = value.into();
        Self { key, value }
    }

    /// Returns the key of the entry.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the value of the entry.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the value of the entry as a list of strings.
    pub fn value_as_list(&self) -> impl Iterator<Item = &str> {
        self.value.split_whitespace()
    }

    /// Returns the value of the entry as a URL.
    #[cfg(feature = "url")]
    #[cfg_attr(docsrs, doc(cfg(feature = "url")))]
    pub fn value_as_url(&self) -> Result<Url, url::ParseError> {
        Url::parse(&self.value)
    }

    /// Returns the value of the entry as a date.
    #[cfg(feature = "date")]
    #[cfg_attr(docsrs, doc(cfg(feature = "date")))]
    pub fn value_as_date(&self) -> Result<NaiveDate, chrono::ParseError> {
        NaiveDate::parse_from_str(&self.value, "%Y-%m-%d")
    }
}

/// A line in the os-release file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OsReleaseLine<'a> {
    /// An empty line or a comment.
    Empty,
    /// An entry.
    Entry(OsReleaseEntry<'a>),
}

impl<'a> OsReleaseLine<'a> {
    /// Returns the [`OsReleaseEntry`], if any.
    pub fn into_entry(self) -> Option<OsReleaseEntry<'a>> {
        match self {
            Self::Empty => None,
            Self::Entry(entry) => Some(entry),
        }
    }
}

impl FromStr for OsReleaseLine<'static> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_line(s).map_or(Self::Empty, Self::Entry))
    }
}

/// Parse a line from the os-release file.
///
/// Returns `None` if the line is empty or a comment.
/// Otherwise, returns the key and value.
///
/// For simplicity, this function assumes that the file is well-formed.
fn parse_line(line: &str) -> Option<OsReleaseEntry<'static>> {
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let (key, value) = line.split_once('=')?;

    let key = key.to_owned();

    let value = match trim_quote(value) {
        // For Bourne shell compatibility, don't unescape single-quoted values.
        (value, Some('\'')) => value.to_owned(),
        // Unescape double-quoted values or unquoted values.
        (value, _) => unescape(value),
    };

    Some(OsReleaseEntry::new(key, value))
}

/// Trim the outermost quotes from a string.
///
/// Returns the trimmed string and the quote character, if any.
///
/// For simplicity, this function assumes that the file is well-formed.
fn trim_quote(value: &str) -> (&str, Option<char>) {
    let quotes = &['"', '\''];
    for &quote in quotes {
        if let Some(value) = value.strip_prefix(quote) {
            let value = value.strip_suffix(quote).unwrap_or(value);
            return (value, Some(quote));
        }
    }
    (value, None)
}

/// Unescape a string.
///
/// This function assumes that the os-release file is well-formed.
///
/// For simplicity, only simple unescaping is performed.
fn unescape(value: &str) -> String {
    let mut output = String::new();
    let mut escaped = false;
    for c in value.chars() {
        if escaped {
            escaped = false;
            output.push(c);
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        output.push(c);
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        fn entry<'a>(key: &'a str, value: &'a str) -> OsReleaseEntry<'a> {
            OsReleaseEntry::new(key, value)
        }

        // empty
        assert!(parse_line("").is_none());

        // comment
        assert!(parse_line("# comment").is_none());

        // key-value
        assert_eq!(parse_line("A=B").unwrap(), entry("A", "B"));
        assert_eq!(parse_line(r#"A="B C""#).unwrap(), entry("A", "B C"));
        assert_eq!(
            parse_line(r#"A="B C\"\"""#).unwrap(),
            entry("A", r#"B C"""#)
        );
        assert_eq!(
            parse_line(r#"A='B C\"\"'"#).unwrap(),
            entry("A", r#"B C\"\""#)
        );
    }
}

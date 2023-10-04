use std::{
    convert::Infallible,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{entry::OsReleaseLine, OsRelease, OsReleaseEntry};

/// Errors that can occur while parsing the os-release file.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The os-release file was not found.
    #[error("no os-release file found")]
    NoOsRelease,
    /// The os-release file could not be opened.
    #[error("failed to open os-release file: {err:?}")]
    Open {
        /// The path to the os-release file.
        path: PathBuf,
        /// The error that occurred while opening the file.
        #[source]
        err: std::io::Error,
    },
    /// The os-release file could not be read.
    #[error("failed to read os-release file: {err:?}")]
    Read {
        /// The error that occurred while reading the file.
        #[source]
        err: std::io::Error,
    },
}

/// Methods to construct an `OsRelease`.
impl OsRelease {
    /// Open the os-release file and parse it.
    ///
    /// If `/etc/os-release` exists, it is opened.
    /// Otherwise, `/usr/lib/os-release` is opened.
    /// If neither file exists, an error is returned.
    ///
    /// For simplicity, this function assumes that the file is well-formed.
    pub fn open() -> Result<Self, Error> {
        let path = os_release_path().ok_or(Error::NoOsRelease)?;
        let file = File::open(path).map_err(|err| Error::Open {
            path: path.to_owned(),
            err,
        })?;
        Self::from_reader(file)
    }

    /// Parse the os-release file from a reader.
    ///
    /// For simplicity, this function assumes that the file is well-formed.
    pub fn from_reader(reader: impl io::Read) -> Result<Self, Error> {
        let reader = BufReader::new(reader);
        reader
            .lines()
            .collect::<Result<_, _>>()
            .map_err(|err| Error::Read { err })
    }
}

impl<'a> FromIterator<OsReleaseEntry<'a>> for OsRelease {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = OsReleaseEntry<'a>>,
    {
        Self {
            fields: iter
                .into_iter()
                .map(|entry| (entry.key().to_owned(), entry.value().to_owned()))
                .collect(),
        }
    }
}

impl<'a> FromIterator<&'a str> for OsRelease {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        iter.into_iter()
            .filter_map(|line| {
                OsReleaseLine::from_str(line)
                    .ok()
                    .and_then(|line| line.into_entry())
            })
            .collect()
    }
}

impl FromIterator<String> for OsRelease {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        iter.into_iter()
            .filter_map(|line| {
                OsReleaseLine::from_str(&line)
                    .ok()
                    .and_then(|line| line.into_entry())
            })
            .collect()
    }
}

impl FromStr for OsRelease {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines().collect())
    }
}

/// Find the os-release file to parse.
fn os_release_path() -> Option<&'static Path> {
    [
        Path::new("/etc/os-release"),
        Path::new("/usr/lib/os-release"),
    ]
    .into_iter()
    .find(|path| path.exists())
}

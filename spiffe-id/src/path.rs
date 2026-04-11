//! Represents the path component of a SPIFFE ID.
//!
//! The [`Path`] struct encapsulates SPIFFE path validation and provides
//! borrowed/owned conversion helpers similar to [`crate::TrustDomain`].
//!
//! # Examples
//!
//! Creating a new [`Path`] from a [`str`]:
//!
//! ```
//! use spiffe_id::Path;
//!
//! let path = Path::new("/service/backend").unwrap();
//! assert_eq!(path.as_str(), "/service/backend");
//! ```
//!
//! Using the constant constructor for compile-time paths:
//!
//! ```
//! use spiffe_id::Path;
//!
//! const PATH: Path = Path::const_new("/service/backend");
//! assert_eq!(PATH.as_str(), "/service/backend");
//! ```
//!
//! # Errors
//!
//! Returns a [`SpiffeIdError`] if the provided path is invalid according to SPIFFE specification.

use alloc::{borrow::Cow, string::String};
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{SpiffeIdError, tri, validate_path};

/// The path component of a SPIFFE ID.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Path<'a> {
    path: Cow<'a, str>,
}

impl<'a> Path<'a> {
    /// Creates a new `Path` from the given string, validating its format.
    ///
    /// # Examples
    /// ```
    /// use spiffe_id::Path;
    ///
    /// let path = Path::new("/service/backend").unwrap();
    /// assert_eq!(path.as_str(), "/service/backend");
    /// ```
    pub fn new(path: &'a str) -> Result<Self, SpiffeIdError> {
        tri!(validate_path(path.as_bytes()));

        Ok(Self {
            path: Cow::Borrowed(path),
        })
    }

    /// Creates a new `Path` in a constant context from a static string slice.
    ///
    /// # Panics
    ///
    /// Panics if the path is invalid.
    ///
    /// # Examples
    /// ```
    /// use spiffe_id::Path;
    ///
    /// const PATH: Path = Path::const_new("/service/backend");
    /// assert_eq!(PATH.as_str(), "/service/backend");
    /// ```
    #[track_caller]
    pub const fn const_new(path: &'static str) -> Self {
        if validate_path(path.as_bytes()).is_err() {
            panic!("invalid path");
        }

        Path {
            path: Cow::Borrowed(path),
        }
    }

    /// Returns a borrowed version of this `Path`.
    pub const fn borrow(&'a self) -> Self {
        Path {
            path: Cow::Borrowed(match &self.path {
                Cow::Borrowed(x) => x,
                Cow::Owned(x) => x.as_str(),
            }),
        }
    }

    /// Converts this `Path` into an owned `'static` value.
    pub fn into_owned(self) -> Path<'static> {
        Path {
            path: Cow::Owned(self.path.into_owned()),
        }
    }

    /// Returns this path as a string slice.
    pub const fn as_str(&self) -> &str {
        match &self.path {
            Cow::Borrowed(path) => path,
            Cow::Owned(path) => path.as_str(),
        }
    }

    /// Creates a `Path` without validation.
    pub(crate) const fn new_unchecked(path: &'a str) -> Self {
        Self {
            path: Cow::Borrowed(path),
        }
    }
}

impl<'a> TryFrom<&'a str> for Path<'a> {
    type Error = SpiffeIdError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Path<'static> {
    type Error = SpiffeIdError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        tri!(Path::new(&path));

        Ok(Path {
            path: Cow::Owned(path),
        })
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Path<'a> {
    type Error = SpiffeIdError;

    fn try_from(path: Cow<'a, str>) -> Result<Self, Self::Error> {
        match path {
            Cow::Borrowed(path) => path.try_into(),
            Cow::Owned(path) => path.try_into(),
        }
    }
}

impl<'a> From<Path<'a>> for Cow<'a, str> {
    fn from(value: Path<'a>) -> Self {
        value.path
    }
}

impl AsRef<str> for Path<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for Path<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("Path").field(&self.path).finish()
    }
}

impl Display for Path<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.path)
    }
}

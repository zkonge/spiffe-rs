//! Represents a SPIFFE Trust Domain, which is the administrative boundary for identities in SPIFFE.
//!
//! The [`TrustDomain`] struct encapsulates the trust domain portion of a SPIFFE ID, providing
//! validation and utility methods for working with trust domains. It supports both borrowed and
//! owned string data via [`Cow`], allowing for efficient usage in various contexts.
//!
//! # Examples
//!
//! Creating a new [`TrustDomain`] from a [`str`]:
//!
//! ```
//! use spiffe_id::TrustDomain;
//!
//! let td = TrustDomain::new("example.org").unwrap();
//! assert_eq!(td.as_str(), "example.org");
//! ```
//!
//! Creating a new [`TrustDomain`] from a string with the `spiffe://` scheme:
//!
//! ```
//! use spiffe_id::TrustDomain;
//!
//! let td = TrustDomain::new("spiffe://example.org").unwrap();
//! assert_eq!(td.as_str(), "example.org");
//! ```
//!
//! Using the constant constructor for compile-time trust domains:
//!
//! ```
//! use spiffe_id::TrustDomain;
//!
//! const TD: TrustDomain = TrustDomain::const_new("spiffe://example.org");
//! ```
//!
//! # Errors
//!
//! Returns a [`TrustDomainError`] if the provided trust domain is invalid according to SPIFFE specification.
use alloc::{borrow::Cow, string::String};
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{SPIFFE_SCHEME, error::TrustDomainError, tri, validate_trust_domain};

/// The administrative boundary for identities within the SPIFFE ecosystem.
///
/// The trust domain is typically expressed as a string, such as "example.org",
/// and is used to scope SPIFFE IDs and related resources.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TrustDomain<'a> {
    pub td: Cow<'a, str>,
}

impl<'a> TrustDomain<'a> {
    /// Creates a new `TrustDomain` from the given string, validating its format.
    ///
    /// Strips the `spiffe://` prefix if present and validates the trust domain.
    ///
    /// # Examples
    /// ```
    /// use spiffe_id::TrustDomain;
    ///
    /// let td = TrustDomain::new("example.org").unwrap();
    /// assert_eq!(td.as_str(), "example.org");
    ///
    /// let td = TrustDomain::new("spiffe://example.org").unwrap();
    /// assert_eq!(td.as_str(), "example.org");
    /// ```
    pub fn new(td: &'a str) -> Result<Self, TrustDomainError> {
        let td = td.strip_prefix(SPIFFE_SCHEME).unwrap_or(td);

        tri!(validate_trust_domain(td.as_bytes()));

        Ok(Self {
            td: Cow::Borrowed(td),
        })
    }

    /// Creates a new `TrustDomain` in a constant context from a static string slice.
    ///
    /// Strips the `spiffe://` prefix if present and validates the trust domain at compile time.
    ///
    /// # Panics
    ///
    /// Panics if the trust domain is invalid.
    ///
    /// # Examples
    /// ```
    /// use spiffe_id::TrustDomain;
    ///
    /// const TD: TrustDomain = TrustDomain::const_new("spiffe://example.org");
    /// assert_eq!(TD.as_str(), "example.org");
    /// ```
    #[track_caller]
    pub const fn const_new(td: &'static str) -> Self {
        const fn str_equal(a: &'static str, b: &'static str) -> bool {
            let a_bytes = a.as_bytes();
            let b_bytes = b.as_bytes();

            if a_bytes.len() != b_bytes.len() {
                return false;
            }

            let mut i = 0;
            while i < a_bytes.len() {
                if a_bytes[i] != b_bytes[i] {
                    return false;
                }
                i += 1;
            }

            true
        }

        // strip the prefix if it exists in const context
        let td = match td.split_at_checked(SPIFFE_SCHEME.len()) {
            Some((maybe_scheme, rem)) => {
                if str_equal(maybe_scheme, SPIFFE_SCHEME) {
                    rem
                } else {
                    td
                }
            }
            None => td,
        };

        if validate_trust_domain(td.as_bytes()).is_err() {
            panic!("Invalid trust domain");
        }

        TrustDomain {
            td: Cow::Borrowed(td),
        }
    }

    /// Returns a borrowed version of the current `TrustDomain`.
    ///
    /// Converts an owned trust domain to a borrowed one if necessary.
    pub const fn borrow(&'a self) -> Self {
        TrustDomain {
            td: Cow::Borrowed(match &self.td {
                Cow::Borrowed(x) => x,
                Cow::Owned(x) => x.as_str(),
            }),
        }
    }

    /// Converts the current `TrustDomain` to an owned version with a `'static` lifetime.
    pub fn into_owned(self) -> TrustDomain<'static> {
        TrustDomain {
            td: Cow::Owned(self.td.into_owned()),
        }
    }

    /// Returns the trust domain as a string slice.
    pub const fn as_str(&self) -> &str {
        match &self.td {
            Cow::Borrowed(td) => td,
            Cow::Owned(td) => td.as_str(),
        }
    }

    /// Creates a new `TrustDomain` from the given string without validation.
    pub(crate) const fn new_unchecked(td: &'a str) -> Self {
        Self {
            td: Cow::Borrowed(td),
        }
    }
}

impl<'a> TryFrom<&'a str> for TrustDomain<'a> {
    type Error = TrustDomainError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for TrustDomain<'static> {
    type Error = TrustDomainError;

    fn try_from(td: String) -> Result<Self, Self::Error> {
        // Delegate validation to the `TrustDomain::new` method.
        //
        // Here we can't use [`Self::new`] because [`Self`] is [`TrustDomain<'static>`],
        // and it requires a static lifetime reference, but "&td" only has a temporary one.
        tri!(TrustDomain::new(&td));

        let td = match td.strip_prefix(SPIFFE_SCHEME) {
            Some(stripped) => stripped.into(),
            None => td,
        };

        Ok(TrustDomain { td: Cow::Owned(td) })
    }
}

impl<'a> TryFrom<Cow<'a, str>> for TrustDomain<'a> {
    type Error = TrustDomainError;

    fn try_from(td: Cow<'a, str>) -> Result<Self, Self::Error> {
        match td {
            Cow::Borrowed(td) => td.try_into(),
            Cow::Owned(td) => td.try_into(),
        }
    }
}

impl<'a> From<TrustDomain<'a>> for Cow<'a, str> {
    fn from(value: TrustDomain<'a>) -> Self {
        value.td
    }
}

impl AsRef<str> for TrustDomain<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for TrustDomain<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("TrustDomain").field(&self.td).finish()
    }
}

impl Display for TrustDomain<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.td)
    }
}

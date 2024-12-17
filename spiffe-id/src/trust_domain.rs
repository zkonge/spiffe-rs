use alloc::{borrow::Cow, string::String};
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{validate_trust_domain, SpiffeIdError};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TrustDomain<'a> {
    td: Cow<'a, str>,
}

impl<'a> TrustDomain<'a> {
    pub const fn new(td: &'a str) -> Result<Self, SpiffeIdError> {
        if let Err(e) = validate_trust_domain(td.as_bytes()) {
            return Err(e);
        }

        Ok(Self {
            td: Cow::Borrowed(td),
        })
    }

    pub fn borrow(&'a self) -> Self {
        TrustDomain {
            td: Cow::Borrowed(&self.td),
        }
    }

    pub fn to_owned(&self) -> TrustDomain<'static> {
        TrustDomain {
            td: Cow::Owned(self.td.clone().into_owned()),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.td
    }

    pub(crate) const fn new_unchecked(td: &'a str) -> Self {
        Self {
            td: Cow::Borrowed(td),
        }
    }
}

impl<'a> TryFrom<&'a str> for TrustDomain<'a> {
    type Error = SpiffeIdError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for TrustDomain<'static> {
    type Error = SpiffeIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_trust_domain(value.as_bytes())?;

        Ok(TrustDomain {
            td: Cow::Owned(value),
        })
    }
}

impl<'a> TryFrom<Cow<'a, str>> for TrustDomain<'a> {
    type Error = SpiffeIdError;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        validate_trust_domain(value.as_bytes())?;

        Ok(TrustDomain { td: value })
    }
}

impl<'a> From<TrustDomain<'a>> for Cow<'a, str> {
    fn from(value: TrustDomain<'a>) -> Self {
        value.td
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

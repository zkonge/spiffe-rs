use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
};
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{validate_trust_domain, SpiffeIdError, SPIFFE_SCHEMA};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TrustDomain<'a> {
    td: Cow<'a, str>,
}

impl<'a> TrustDomain<'a> {
    pub fn new(td: &'a str) -> Result<Self, SpiffeIdError> {
        let td = td.strip_prefix(SPIFFE_SCHEMA).unwrap_or(td);

        validate_trust_domain(td.as_bytes())?;

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

    fn try_from(td: String) -> Result<Self, Self::Error> {
        let td = td
            .strip_prefix(SPIFFE_SCHEMA)
            .map(ToOwned::to_owned)
            .unwrap_or(td);

        validate_trust_domain(td.as_bytes())?;

        Ok(TrustDomain { td: Cow::Owned(td) })
    }
}

impl<'a> TryFrom<Cow<'a, str>> for TrustDomain<'a> {
    type Error = SpiffeIdError;

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

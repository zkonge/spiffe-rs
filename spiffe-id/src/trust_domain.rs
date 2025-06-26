use alloc::{borrow::Cow, string::String};
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{SPIFFE_SCHEME, SpiffeIdError, tri, validate_trust_domain};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TrustDomain<'a> {
    td: Cow<'a, str>,
}

impl<'a> TrustDomain<'a> {
    pub fn new(td: &'a str) -> Result<Self, SpiffeIdError> {
        let td = td.strip_prefix(SPIFFE_SCHEME).unwrap_or(td);

        tri!(validate_trust_domain(td.as_bytes()));

        Ok(Self {
            td: Cow::Borrowed(td),
        })
    }

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

    pub const fn borrow(&'a self) -> Self {
        TrustDomain {
            td: Cow::Borrowed(match &self.td {
                Cow::Borrowed(x) => x,
                Cow::Owned(x) => x.as_str(),
            }),
        }
    }

    pub fn to_owned(&self) -> TrustDomain<'static> {
        TrustDomain {
            td: Cow::Owned(self.td.clone().into_owned()),
        }
    }

    pub const fn as_str(&self) -> &str {
        match &self.td {
            Cow::Borrowed(td) => td,
            Cow::Owned(td) => td.as_str(),
        }
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
        // Delegate validation to the `TrustDomain::new` method.
        //
        // Here we can't use [`Self::new`] because [`Self`] is [`TrustDomain<'static>`],
        // and it requires a static lifetime reference, but "&td" only has a temporary one.
        TrustDomain::new(&td)?;

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

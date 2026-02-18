use prost::Message;

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct FederatesWithMatch {
    /// The set of trust domain names to match on (e.g., "example.org").
    #[prost(string, repeated, tag = "1")]
    pub trust_domains: Vec<String>,

    /// How to match the trust domains.
    #[prost(enumeration = "federates_with_match::MatchBehavior", tag = "2")]
    pub r#match: i32,
}

pub mod federates_with_match {
    use prost::Enumeration;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    #[repr(i32)]
    pub enum MatchBehavior {
        /// Indicates that the federated trust domains in this match are
        /// equal to the candidate trust domains, independent of ordering.
        MatchExact = 0,

        /// Indicates that all candidates which have a non-empty subset
        /// of the provided set of trust domains will match.
        MatchSubset = 1,

        /// Indicates that all candidates which are a superset
        /// of the provided set of trust domains will match.
        MatchSuperset = 2,

        /// Indicates that all candidates which have at least one
        /// of the provided set of trust domains will match.
        MatchAny = 3,
    }

    impl MatchBehavior {
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::MatchExact => "MATCH_EXACT",
                Self::MatchSubset => "MATCH_SUBSET",
                Self::MatchSuperset => "MATCH_SUPERSET",
                Self::MatchAny => "MATCH_ANY",
            }
        }

        pub fn from_str_name(value: &str) -> Option<Self> {
            match value {
                "MATCH_EXACT" => Some(Self::MatchExact),
                "MATCH_SUBSET" => Some(Self::MatchSubset),
                "MATCH_SUPERSET" => Some(Self::MatchSuperset),
                "MATCH_ANY" => Some(Self::MatchAny),
                _ => None,
            }
        }
    }
}

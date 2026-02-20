use prost::Message;

#[derive(Clone, PartialEq, Eq, Hash, Message)]
pub struct Selector {
    /// The type of the selector. This is typically the name of the plugin that
    /// produces the selector.
    #[prost(string, tag = "1")]
    pub r#type: String,

    /// The value of the selector.
    #[prost(string, tag = "2")]
    pub value: String,
}

pub mod selector_match {
    use prost::Enumeration;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    #[repr(i32)]
    pub enum MatchBehavior {
        /// Indicates that the selectors in this match are equal to the
        /// candidate selectors, independent of ordering.
        MatchExact = 0,

        /// Indicates that all candidates which have a non-empty subset
        /// of the provided set of selectors will match.
        MatchSubset = 1,

        /// Indicates that all candidates which are a superset
        /// of the provided selectors will match.
        MatchSuperset = 2,

        /// Indicates that all candidates which have at least one
        /// of the provided set of selectors will match.
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

#[derive(Clone, PartialEq, Message)]
pub struct SelectorMatch {
    /// The set of selectors to match on.
    #[prost(message, repeated, tag = "1")]
    pub selectors: Vec<Selector>,

    /// How to match the selectors.
    #[prost(enumeration = "selector_match::MatchBehavior", tag = "2")]
    pub r#match: i32,
}

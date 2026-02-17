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

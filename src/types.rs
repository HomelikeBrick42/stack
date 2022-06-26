use derive_more::{Display, IsVariant, Unwrap};

#[derive(Clone, PartialEq, Debug, Display, IsVariant, Unwrap)]
pub enum Type {
    #[display(fmt = "type")]
    Type,
    #[display(fmt = "int")]
    Integer,
    #[display(fmt = "bool")]
    Bool,
}

use derive_more::{Display, IsVariant, Unwrap};

#[derive(Clone, PartialEq, Debug, Display, IsVariant, Unwrap)]
pub enum Type {
    #[display(fmt = "type")]
    Type,
    #[display(fmt = "int")]
    Integer,
    #[display(fmt = "bool")]
    Bool,
    #[display(fmt = "proc{:?} -> {:?}", _0, _1)]
    Procedure(Vec<Type>, Vec<Type>),
}

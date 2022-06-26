use derive_more::{Display, IsVariant, Unwrap};

use crate::types::Type;

#[derive(Clone, Debug, PartialEq, Display, IsVariant, Unwrap)]
pub enum Value {
    Type(Type),
    Integer(isize),
    Bool(bool),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Type(_) => Type::Type,
            Value::Integer(_) => Type::Integer,
            Value::Bool(_) => Type::Bool,
        }
    }
}

#[derive(Clone, Debug, PartialEq, IsVariant)]
pub enum Instruction {
    Push(Value),
    Dup,
    Drop,
    Add,
    Subtract,
    LessThan,
    GreaterThan,
    Equal,
    Not,
    Print,
    If {
        then_block: Vec<Instruction>,
        else_block: Vec<Instruction>,
    },
    While {
        condition_block: Vec<Instruction>,
        body_block: Vec<Instruction>,
    },
}

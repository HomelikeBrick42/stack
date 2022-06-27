use std::rc::Rc;

use derive_more::{Display, IsVariant, Unwrap};

use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub struct Procedure {
    pub typ: Type,
    pub instructions: Vec<Instruction>,
}

#[derive(Clone, Debug, PartialEq, Display, IsVariant, Unwrap)]
pub enum Value {
    Type(Type),
    Integer(isize),
    Bool(bool),
    #[display(fmt = "{} {{ ... }}", "_0.typ")]
    Procedure(Rc<Procedure>),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Type(_) => Type::Type,
            Value::Integer(_) => Type::Integer,
            Value::Bool(_) => Type::Bool,
            Value::Procedure(proc) => proc.typ.clone(),
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
    Call,
}

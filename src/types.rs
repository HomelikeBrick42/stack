use std::fmt::Display;

use derive_more::{IsVariant, Unwrap};

#[derive(Clone, PartialEq, Debug, IsVariant, Unwrap)]
pub enum Type {
    Type,
    Integer,
    Bool,
    Procedure(Vec<Type>, Vec<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Type => write!(f, "type"),
            Type::Integer => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::Procedure(parameter_types, return_types) => {
                write!(f, "proc[")?;
                for (i, typ) in parameter_types.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", typ)?;
                }
                write!(f, "] -> [")?;
                for (i, typ) in return_types.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", typ)?;
                }
                write!(f, "]")
            }
        }
    }
}

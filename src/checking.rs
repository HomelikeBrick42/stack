use crate::{instructions::Instruction, types::Type};

fn expect_at_least_type_count(type_stack: &[Type], type_count: usize) -> Result<(), String> {
    if type_stack.len() >= type_count {
        return Ok(());
    }
    Err(format!(
        "Expected at least {} types, but got only {} types",
        type_count,
        type_stack.len()
    ))
}

fn expect_at_least_types(type_stack: &[Type], types: &[Type]) -> Result<(), String> {
    if type_stack.len() >= types.len()
        && &type_stack[type_stack.len() - types.len()..type_stack.len()] == types
    {
        return Ok(());
    }
    Err(format!(
        "Expected at least {types:?}, but got {type_stack:?}"
    ))
}

fn expect_types_equal(type_stack: &[Type], types: &[Type]) -> Result<(), String> {
    if type_stack == types {
        return Ok(());
    }
    Err(format!("Expected {types:?}, but got {type_stack:?}"))
}

pub fn type_check(instructions: &[Instruction], type_stack: &mut Vec<Type>) -> Result<(), String> {
    for instruction in instructions {
        match instruction {
            Instruction::Push(value) => {
                type_stack.push(value.get_type());
                if value.is_procedure() {
                    let procedure = value.clone().unwrap_procedure();
                    let (parameter_types, return_types) = procedure.typ.clone().unwrap_procedure();
                    let mut proc_type_stack = parameter_types.clone();
                    type_check(&procedure.instructions, &mut proc_type_stack)?;
                    expect_types_equal(&proc_type_stack, &return_types)?;
                }
            }
            Instruction::Dup => {
                expect_at_least_type_count(type_stack, 1)?;
                type_stack.push(type_stack.last().unwrap().clone());
            }
            Instruction::Drop => {
                expect_at_least_type_count(type_stack, 1)?;
                type_stack.pop();
            }
            Instruction::Add | Instruction::Subtract => {
                expect_at_least_types(type_stack, &[Type::Integer, Type::Integer])?;
                type_stack.pop();
            }
            Instruction::LessThan | Instruction::GreaterThan => {
                expect_at_least_types(type_stack, &[Type::Integer, Type::Integer])?;
                type_stack.pop();
                type_stack.pop();
                type_stack.push(Type::Bool);
            }
            Instruction::Equal => {
                expect_at_least_type_count(type_stack, 2)?;
                type_stack.pop();
                type_stack.pop();
                type_stack.push(Type::Bool);
            }
            Instruction::Not => {
                expect_at_least_types(type_stack, &[Type::Bool])?;
            }
            Instruction::Print => {
                expect_at_least_type_count(type_stack, 1)?;
                type_stack.pop();
            }
            Instruction::If {
                then_block,
                else_block,
            } => {
                expect_at_least_types(type_stack, &[Type::Bool])?;
                type_stack.pop();

                type_check(then_block, type_stack)?;
                let mut else_types = Vec::new();
                type_check(else_block, &mut else_types)?;
                expect_types_equal(&else_types, type_stack)?;
            }
            Instruction::While {
                condition_block: condition,
                body_block,
            } => {
                type_check(condition, type_stack)?;
                expect_at_least_types(type_stack, &[Type::Bool])?;
                type_stack.pop();

                let mut body_types = type_stack.clone();
                type_check(body_block, &mut body_types)?;
                expect_types_equal(&body_types, type_stack)?;
            }
            Instruction::Call => {
                expect_at_least_type_count(type_stack, 1)?;
                let typ = type_stack.pop().unwrap();
                if !typ.is_procedure() {
                    return Err(format!("Expected a procedure to call, but got {}", typ));
                }
                let (parameter_types, mut return_types) = typ.unwrap_procedure();
                expect_at_least_types(type_stack, &parameter_types)?;
                for _ in parameter_types {
                    type_stack.pop();
                }
                type_stack.append(&mut return_types);
            }
        }
    }
    Ok(())
}

use crate::instructions::{Instruction, Value};

pub fn execute(
    instructions: &[Instruction],
    stack: &mut Vec<Value>,
    stdout: &mut dyn std::fmt::Write,
) -> Result<(), std::fmt::Error> {
    let mut i = 0;
    while i < instructions.len() {
        match &instructions[i] {
            Instruction::Push(value) => {
                stack.push(value.clone());
            }
            Instruction::Dup => {
                stack.push(stack.last().unwrap().clone());
            }
            Instruction::Drop => {
                stack.pop().unwrap();
            }
            Instruction::Add => {
                let b = stack.pop().unwrap().unwrap_integer();
                let a = stack.pop().unwrap().unwrap_integer();
                stack.push(Value::Integer(a + b));
            }
            Instruction::Subtract => {
                let b = stack.pop().unwrap().unwrap_integer();
                let a = stack.pop().unwrap().unwrap_integer();
                stack.push(Value::Integer(a - b));
            }
            Instruction::LessThan => {
                let b = stack.pop().unwrap().unwrap_integer();
                let a = stack.pop().unwrap().unwrap_integer();
                stack.push(Value::Bool(a < b));
            }
            Instruction::GreaterThan => {
                let b = stack.pop().unwrap().unwrap_integer();
                let a = stack.pop().unwrap().unwrap_integer();
                stack.push(Value::Bool(a > b));
            }
            Instruction::Equal => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Value::Bool(a == b));
            }
            Instruction::Not => {
                let value = stack.pop().unwrap().unwrap_bool();
                stack.push(Value::Bool(!value));
            }
            Instruction::Print => {
                writeln!(stdout, "{}", stack.pop().unwrap())?;
            }
            Instruction::If {
                then_block,
                else_block,
            } => {
                let condition = stack.pop().unwrap().unwrap_bool();
                if condition {
                    execute(then_block, stack, stdout)?;
                } else {
                    execute(else_block, stack, stdout)?;
                };
            }
            Instruction::While {
                condition_block: condition,
                body_block,
            } => loop {
                execute(condition, stack, stdout)?;
                let condition = stack.pop().unwrap().unwrap_bool();
                if !condition {
                    break;
                }
                execute(body_block, stack, stdout)?;
            },
            Instruction::Call => {
                let procedure = stack.pop().unwrap().unwrap_procedure();
                execute(&procedure.instructions, stack, stdout)?;
            }
        }
        i += 1;
    }
    Ok(())
}

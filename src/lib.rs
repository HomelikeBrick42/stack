pub mod checking;
pub mod execute;
pub mod instructions;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{
        checking::type_check,
        execute::execute,
        instructions::{Instruction, Value},
        types::Type,
    };

    #[test]
    fn add() {
        let instructions = [
            Instruction::Push(Value::Integer(5)),
            Instruction::Push(Value::Integer(6)),
            Instruction::Add,
            Instruction::Print,
        ];

        let mut type_stack = Vec::new();
        type_check(&instructions, &mut type_stack).unwrap();
        assert_eq!(type_stack, []);

        let mut output = String::new();
        let mut stack = Vec::new();
        execute(&instructions, &mut stack, &mut output).unwrap();
        assert_eq!(stack, []);
        assert_eq!(output, "11\n");
    }

    #[test]
    fn if_else() {
        let instructions = [
            Instruction::Push(Value::Bool(true)),
            Instruction::If {
                then_block: vec![Instruction::Push(Value::Integer(69))],
                else_block: vec![Instruction::Push(Value::Integer(420))],
            },
            Instruction::Print,
            Instruction::Push(Value::Bool(false)),
            Instruction::If {
                then_block: vec![Instruction::Push(Value::Integer(5))],
                else_block: vec![
                    Instruction::Push(Value::Integer(34)),
                    Instruction::Push(Value::Integer(35)),
                    Instruction::Add,
                ],
            },
        ];

        let mut type_stack = Vec::new();
        type_check(&instructions, &mut type_stack).unwrap();
        assert_eq!(type_stack, [Type::Integer]);

        let mut output = String::new();
        let mut stack = Vec::new();
        execute(&instructions, &mut stack, &mut output).unwrap();
        assert_eq!(stack, [Value::Integer(69)]);
        assert_eq!(output, "69\n");
    }

    #[test]
    fn while_() {
        let instructions = [
            Instruction::Push(Value::Integer(1)),
            Instruction::While {
                condition_block: vec![
                    Instruction::Dup,
                    Instruction::Push(Value::Integer(10)),
                    Instruction::GreaterThan,
                    Instruction::Not,
                ],
                body_block: vec![
                    Instruction::Dup,
                    Instruction::Print,
                    Instruction::Push(Value::Integer(1)),
                    Instruction::Add,
                ],
            },
            Instruction::Drop,
        ];

        let mut type_stack = Vec::new();
        type_check(&instructions, &mut type_stack).unwrap();
        assert_eq!(type_stack, []);

        let mut output = String::new();
        let mut stack = Vec::new();
        execute(&instructions, &mut stack, &mut output).unwrap();
        assert_eq!(stack, []);
        assert_eq!(output, "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
    }
}

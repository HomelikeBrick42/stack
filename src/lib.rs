pub mod checking;
pub mod execute;
pub mod instructions;
pub mod types;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        checking::type_check,
        execute::execute,
        instructions::{Instruction, Procedure, Value},
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
        /*
            5 6 + print
        */

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
        /*
            // `true if` is also valid
            if true {
                69
            } else {
                420
            }
            print

            if false {
                5
            } else {
                34 35 +
            }
        */

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
        /*
            1 while dup 10 <= {
                dup print
                1 +
            } drop
        */

        let mut type_stack = Vec::new();
        type_check(&instructions, &mut type_stack).unwrap();
        assert_eq!(type_stack, []);

        let mut output = String::new();
        let mut stack = Vec::new();
        execute(&instructions, &mut stack, &mut output).unwrap();
        assert_eq!(stack, []);
        assert_eq!(output, "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
    }

    #[test]
    fn procedure() {
        let instructions = [
            Instruction::Push(Value::Integer(5)),
            Instruction::Push(Value::Procedure(Rc::new(Procedure {
                typ: Type::Procedure(vec![Type::Integer], vec![Type::Integer]),
                instructions: vec![],
            }))),
            Instruction::Call,
            Instruction::Print,
            Instruction::Push(Value::Integer(34)),
            Instruction::Push(Value::Integer(35)),
            Instruction::Push(Value::Procedure(Rc::new(Procedure {
                typ: Type::Procedure(vec![Type::Integer, Type::Integer], vec![]),
                instructions: vec![Instruction::Add, Instruction::Print],
            }))),
            Instruction::Call,
        ];
        /*
            5
            proc[int] -> [int] {}
            call print

            34 35
            proc[int int] -> [] {
                + print
            }
            call
        */

        let mut type_stack = Vec::new();
        type_check(&instructions, &mut type_stack).unwrap();
        assert_eq!(type_stack, []);

        let mut output = String::new();
        let mut stack = Vec::new();
        execute(&instructions, &mut stack, &mut output).unwrap();
        assert_eq!(stack, []);
        assert_eq!(output, "5\n69\n");
    }
}

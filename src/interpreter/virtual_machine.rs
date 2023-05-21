use crate::compiler::{Chunk, OpCode};
use crate::compiler::Value;
use crate::logging;

#[allow(dead_code)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    instruction_index: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            instruction_index: 0,
            stack: Vec::with_capacity(1024),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        use OpCode::*;

        while self.instruction_index < chunk.code_nb() {
            let instruction = chunk.get_instruction(self.instruction_index);
            logging::log_stack(&self.stack);
            logging::log_instruction(&chunk, &instruction, self.instruction_index);
            self.instruction_index += 1;
            match instruction {
                Return => {
                    self.pop_value();
                    break;
                }
                Negate => {
                    let Value::Number(num) = self.pop_value();
                    self.stack.push(Value::Number(-num));
                }
                Add | Sub | Mul | Mod | Div => {
                    let res = self.binary_op(instruction);
                    self.stack.push(res);
                }
                Constant(index) => {
                    let constant = chunk.get_constant(index);
                    self.stack.push(constant);
                }
            }
        }
        logging::log_stack(&self.stack);
        InterpretResult::Ok
    }

    fn pop_value(&mut self) -> Value {
        self.stack
            .pop()
            .expect("Expected a constant in the value stack")
    }

    fn binary_op(&mut self, instruction: OpCode) -> Value {
        // This function is only called with binary operators:
        // [Add, Sub, Mul, Div, Mod]
        use OpCode::*;

        let Value::Number(rhs) = self.pop_value();
        let Value::Number(lhs) = self.pop_value();
        let result = match instruction {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs + rhs,
            Div => lhs / rhs,
            Mod => lhs % rhs,
            _ => unreachable!(),
        };
        Value::Number(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unary_negate() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        let num = 20.0;

        let constant = chunk.add_constants(Value::Number(num));
        chunk.write_opcode(OpCode::Constant(constant), 0);
        chunk.write_opcode(OpCode::Negate, 0);
        vm.interpret(chunk);

        assert_eq!(vm.pop_value(), Value::Number(-num));
    }

    #[test]
    fn binary_add() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        let lhs = 10.0;
        let rhs = 20.0;

        let constant = chunk.add_constants(Value::Number(lhs));
        chunk.write_opcode(OpCode::Constant(constant), 0);
        let constant = chunk.add_constants(Value::Number(rhs));
        chunk.write_opcode(OpCode::Constant(constant), 0);
        chunk.write_opcode(OpCode::Add, 0);

        vm.interpret(chunk);
        assert_eq!(vm.pop_value(), Value::Number(lhs + rhs));
    }
}

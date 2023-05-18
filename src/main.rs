mod chunk;
mod logging;
mod value;
mod virtual_machine;

use chunk::{Chunk, OpCode};
use std::env;
use value::Value;
use virtual_machine::VM;

fn main() {
    let _args: Vec<String> = env::args().collect();
    let mut vm = VM::new();
    let mut chunk = Chunk::new();
    let constant = chunk.add_constants(Value::Number(1.2));
    chunk.write_opcode(OpCode::Constant(constant), 0);
    let constant = chunk.add_constants(Value::Number(42.0));
    chunk.write_opcode(OpCode::Constant(constant), 0);
    chunk.write_opcode(OpCode::Negate, 1);
    chunk.write_opcode(OpCode::Add, 1);
    chunk.write_opcode(OpCode::Return, 1);
    vm.interpret(chunk);
}

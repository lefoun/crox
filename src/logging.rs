use crate::{value::Value, Chunk, OpCode};

// created this log to be able to specify #[allow(dead_code)] on the entire file
#[allow(dead_code)]
pub mod log {
    use super::*;
    #[cfg(not(feature = "logging"))]
    pub fn log_stack(_stack: &Vec<Value>) {}

    #[cfg(feature = "logging")]
    pub fn log_stack(stack: &Vec<Value>) {
        print!("-- stack log:         ");
        for v in stack.iter().rev() {
            print!("[ {v} ]");
        }
        if stack.is_empty() {
            println!("empty");
        } else {
            println!();
        }
    }

    #[cfg(feature = "logging")]
    pub fn desassemble_chunk(chunk: &Chunk, name: &str) {
        println!("== {name} ==");
        for (offset, instruction) in chunk.code().iter().enumerate() {
            log_instruction(chunk, instruction, offset);
        }
    }

    #[cfg(not(feature = "logging"))]
    pub fn log_instruction(_chunk: &Chunk, _instruction: &OpCode, _offset: usize) {}

    #[cfg(feature = "logging")]
    pub fn log_instruction(chunk: &Chunk, instruction: &OpCode, offset: usize) {
        print!("** {:04} {:03}  ", offset, chunk.get_line(offset));
        match instruction {
            OpCode::Return => println!("RETURN"),
            OpCode::Negate => println!("NEGATE"),
            OpCode::Add => println!("ADD"),
            OpCode::Sub => println!("SUB"),
            OpCode::Mul => println!("MUL"),
            OpCode::Div => println!("DIV"),
            OpCode::Mod => println!("MOD"),
            OpCode::Constant(index) => {
                println!(
                    "CONSTANT {} '{}'",
                    index,
                    chunk.get_constant(*index).to_string()
                )
            }
        }
    }
}

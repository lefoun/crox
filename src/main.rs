mod virtual_machine;
use virtual_machine::{Chunk, OpCode};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut chunk = Chunk::new();
    chunk.write_chunk(OpCode::Return);
    chunk.write_chunk(OpCode::Return);
    chunk.write_chunk(OpCode::Return);
    chunk.write_chunk(OpCode::Return);
    chunk.write_chunk(OpCode::Return);
    virtual_machine::desassemble_chunk(&chunk, "first");
}

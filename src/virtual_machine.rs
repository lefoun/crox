pub enum OpCode {
    Return,
}

pub struct Chunk {
    code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: OpCode) {
        self.code.push(byte);
    }

}

pub fn desassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {name} ==");
    for (offset, instruction) in chunk.code.iter().enumerate() {
        print!("** {:04} ", offset);
        match instruction {
            OpCode::Return => println!("OP_RETURN"),
        }
    }
}
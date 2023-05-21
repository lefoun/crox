use super::value::Value;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum OpCode {
    Return,
    // Unary Op
    Negate,
    // Binary Op
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // usize reprensent the index of the constant in the chunk
    Constant(usize),
}

pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_opcode(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constants(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn code_nb(&self) -> usize {
        self.code().len()
    }

    pub fn code(&self) -> &Vec<OpCode> {
        &self.code
    }

    pub fn get_instruction(&self, index: usize) -> OpCode {
        *self
            .code()
            .get(index)
            .expect("Expected a correct index of instruction code")
    }

    pub fn get_constant(&self, index: usize) -> Value {
        *self
            .constants
            .get(index)
            .expect("Expected a correct index of instruction code")
    }

    pub fn get_line(&self, index: usize) -> usize {
        *self
            .lines
            .get(index)
            .expect("Expected a correct index of lines")
    }
}

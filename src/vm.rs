#[repr(u8)]
pub enum OpCode {
    Constant,
    Print,
    Return,
}

pub struct Chunk {
    name: String,
    pub instructions: Vec<u8>,
    pub constants: Vec<f64>,
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== {} ===", self.name)?;
        let mut i = 1 as u8;
        for entry in &self.instructions {
            writeln!(f, "{i} - {entry}")?;
            i += 1;
        }
        Ok(())
    }
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }
}

pub struct InstructionDisassembler<'c> {
    chunk: &'c Chunk,
}

impl<'c> InstructionDisassembler<'c> {
    pub fn new(chunk: &'c Chunk) -> Self {
        Self { chunk }
    }
}

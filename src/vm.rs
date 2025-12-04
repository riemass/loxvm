use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Print,
    Return,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Constant => write!(f, "Const"),
            OpCode::Print => write!(f, "Print"),
            OpCode::Return => write!(f, "Return"),
        }
    }
}

// TODO: Introduce Value type instead of f64
pub struct Chunk {
    name: String,
    pub code: Vec<u8>,
    pub constants: Vec<f64>,
    pub variables: Vec<f64>,
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== {} ===", self.name)?;
        let mut i = 0;
        while i < self.code.len() {
            write!(f, "{:04} - ", i + 1)?;
            i += self.dissasemble_instruction(f, i)?;
        }
        Ok(())
    }
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            code: Vec::new(),
            constants: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn emit<T>(&mut self, val: T)
    where
        T: Into<u8>,
    {
        self.code.push(val.into());
    }

    pub fn write_constant(&mut self, val: f64) -> usize {
        self.constants.push(val);
        self.constants.len() - 1
    }

    pub fn write_variable(&mut self, val: f64) -> usize {
        self.variables.push(val);
        self.variables.len() - 1
    }

    fn dissasemble_instruction(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        offset: usize,
    ) -> Result<usize, std::fmt::Error> {
        let instruction = OpCode::try_from(self.code[offset]).map_err(|_| std::fmt::Error)?;
        match instruction {
            OpCode::Constant | OpCode::Print => {
                let id = self.code[offset + 1];
                writeln!(f, "{instruction} {id} ({})", self.constants[id as usize])?;
                return Ok(2);
            }
            OpCode::Return => {
                writeln!(f, "{instruction}")?;
                return Ok(1);
            }
        }
    }
}

// TODO: take chunk as ref.
pub struct VM {
    chunk: Chunk,
    ip: usize,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self { chunk, ip: 0 }
    }

    pub fn interpret(&mut self) -> Result<(), String> {
        loop {
            let instruction = OpCode::try_from(self.chunk.code[self.ip])
                .map_err(|_| String::from("Invalid instruction decode"))?;
            self.ip += 1;
            match instruction {
                OpCode::Return => return Ok(()),
                _ => continue,
            }
        }
        Ok(())
    }
}

#[test]
fn tests() {
    let mut chunk = Chunk::new("test bytes");
    let id = chunk.write_constant(1.25);
    chunk.emit(OpCode::Constant);
    chunk.emit(id as u8);
    chunk.emit(OpCode::Return);
    let output = format!("{:?}", chunk);
    let mut dissassembled = output.lines();
    assert_eq!(dissassembled.next(), Some("=== test bytes ==="));
    assert_eq!(dissassembled.next(), Some("0001 - Const 0 (1.25)"));
    assert_eq!(dissassembled.next(), Some("0003 - Return"));
}

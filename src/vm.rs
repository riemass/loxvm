use crate::Value;
use crate::error::Error;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpPop,
    Print,
    Return,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Constant => write!(f, "Const"),
            OpCode::Print => write!(f, "Print"),
            OpCode::Return => write!(f, "Return"),
            OpCode::OpPop => write!(f, "Pop"),
            OpCode::OpAdd => write!(f, "Add"),
            OpCode::OpSubtract => write!(f, "Sub"),
            OpCode::OpMultiply => write!(f, "Mul"),
            OpCode::OpDivide => write!(f, "Div"),
            OpCode::OpNegate => write!(f, "Neg"),
        }
    }
}

pub struct Chunk {
    pub name: String,
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub variables: Vec<Value>,
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
        self.constants.push(Value::from(val));
        self.constants.len() - 1
    }

    pub fn write_variable(&mut self, val: f64) -> usize {
        self.variables.push(Value::from(val));
        self.variables.len() - 1
    }

    fn dissasemble_instruction(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        offset: usize,
    ) -> Result<usize, std::fmt::Error> {
        let instruction = OpCode::try_from(self.code[offset]).map_err(|_| std::fmt::Error)?;
        match instruction {
            OpCode::Constant => {
                let id = self.code[offset + 1];
                writeln!(f, "{instruction} {id} ({})", self.constants[id as usize])?;
                return Ok(2);
            }
            OpCode::Print | OpCode::OpPop | OpCode::Return => {
                writeln!(f, "{instruction}")?;
                return Ok(1);
            }
            OpCode::OpAdd
            | OpCode::OpSubtract
            | OpCode::OpMultiply
            | OpCode::OpDivide
            | OpCode::OpNegate => {
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
    // TODO: Does it need to be public?
    pub stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    pub fn interpret(&mut self) -> Result<(), Error> {
        loop {
            let next_byte = self.read_byte();
            let instruction = OpCode::try_from(next_byte)
                .map_err(|_| Error::invalid_instruction(next_byte, self.ip - 1))?;
            match instruction {
                OpCode::Return => return Ok(()),
                OpCode::Constant => {
                    let const_id = self.read_byte() as usize;
                    self.stack.push(self.chunk.constants[const_id].clone());
                }
                OpCode::OpAdd => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        let result = Value::checked_add(a, b)?;
                        self.stack.push(result);
                    } else {
                        return Err(Error::stack_underflow(
                            "Corruption while doing binary operator",
                        ));
                    }
                }
                OpCode::OpSubtract => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        let result = Value::checked_sub(a, b)?;
                        self.stack.push(result);
                    } else {
                        return Err(Error::stack_underflow(
                            "Corruption while doing binary operator",
                        ));
                    }
                }
                OpCode::OpMultiply => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        let result = Value::checked_mul(a, b)?;
                        self.stack.push(result);
                    } else {
                        return Err(Error::stack_underflow(
                            "Corruption while doing binary operator",
                        ));
                    }
                }
                OpCode::OpDivide => {
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        let result = Value::checked_div(a, b)?;
                        self.stack.push(result);
                    } else {
                        return Err(Error::stack_underflow(
                            "Corruption while doing binary operator",
                        ));
                    }
                }
                OpCode::OpNegate => {
                    // TODO: Examine this code.
                    if let Some(a) = self.stack.last_mut() {
                        *a = Value::checked_neg(a.clone())?;
                    } else {
                        return Err(Error::stack_underflow(
                            "Corruption while doing unary operator",
                        ));
                    }
                }
                OpCode::OpPop => {
                    self.stack
                        .pop()
                        .ok_or_else(|| Error::stack_underflow("No value to pop"))?;
                }
                OpCode::Print => {
                    let val = self
                        .stack
                        .pop()
                        .ok_or_else(|| Error::stack_underflow("No value to print"))?;
                    println!("> {val}");
                }
            }
        }
    }
}

#[test]
fn tests() {
    {
        let mut chunk = Chunk::new("test bytes 1");

        let id = chunk.write_constant(1.25);
        chunk.emit(OpCode::Constant);
        chunk.emit(id as u8);
        chunk.emit(OpCode::Return);

        let output = format!("{:?}", chunk);
        let mut dissassembled = output.lines();
        assert_eq!(dissassembled.next(), Some("=== test bytes 1 ==="));
        assert_eq!(dissassembled.next(), Some("0001 - Const 0 (1.25)"));
        assert_eq!(dissassembled.next(), Some("0003 - Return"));

        let mut vm = VM::new(chunk);
        vm.interpret().unwrap();
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack.first(), Some(&Value::from(1.25)));
    }
    {
        let mut chunk = Chunk::new("test bytes 2");

        // - ((1.25 + 3.5) / 4.75)
        let id = chunk.write_constant(1.25);
        chunk.emit(OpCode::Constant);
        chunk.emit(id as u8);
        let id = chunk.write_constant(3.5);
        chunk.emit(OpCode::Constant);
        chunk.emit(id as u8);
        chunk.emit(OpCode::OpAdd);
        let id = chunk.write_constant(4.75);
        chunk.emit(OpCode::Constant);
        chunk.emit(id as u8);
        chunk.emit(OpCode::OpDivide);
        chunk.emit(OpCode::OpNegate);
        chunk.emit(OpCode::Return);

        let output = format!("{:?}", chunk);
        let mut dissassembled = output.lines();
        assert_eq!(dissassembled.next(), Some("=== test bytes 2 ==="));
        assert_eq!(dissassembled.next(), Some("0001 - Const 0 (1.25)"));
        assert_eq!(dissassembled.next(), Some("0003 - Const 1 (3.5)"));
        assert_eq!(dissassembled.next(), Some("0005 - Add"));
        assert_eq!(dissassembled.next(), Some("0006 - Const 2 (4.75)"));
        assert_eq!(dissassembled.next(), Some("0008 - Div"));
        assert_eq!(dissassembled.next(), Some("0009 - Neg"));
        assert_eq!(dissassembled.next(), Some("0010 - Return"));

        let mut vm = VM::new(chunk);
        vm.interpret().unwrap();
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack.first(), Some(&Value::from(-1.0)));
    }
}

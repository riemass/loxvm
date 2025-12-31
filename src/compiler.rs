use crate::{ast, vm};

pub fn compile(statement: &ast::Statement) -> vm::Chunk {
    let mut chunk = vm::Chunk::new("main");

    match statement {
        ast::Statement::Expression(expr) => compile_expression(&mut chunk, expr),
        ast::Statement::Print(print_stmt) => compile_print_expression(&mut chunk, print_stmt),
        _ => todo!(),
    }

    chunk
}

pub fn compile_expression(chunk: &mut vm::Chunk, expr: &ast::ExpressionStmt) {
    match expr {
        ast::ExpressionStmt::Number(x) => compile_number(chunk, *x),
        ast::ExpressionStmt::Identifier(_) => todo!(),
        ast::ExpressionStmt::Unary(op, expr) => compile_unary(chunk, op, expr),
        ast::ExpressionStmt::Binary(op, expr_pair) => compile_binary(chunk, op, expr_pair),
    }
}

fn compile_number(chunk: &mut vm::Chunk, x: f64) {
    let id = chunk.write_constant(x);
    chunk.emit(vm::OpCode::Constant);
    chunk.emit(id as u8);
}

fn compile_unary(chunk: &mut vm::Chunk, op: &str, expr: &ast::ExpressionStmt) {
    compile_expression(chunk, expr);
    // TODO: refactor
    if op.starts_with("+") {
        chunk.emit(vm::OpCode::OpAdd);
    } else if op.starts_with("-") {
        chunk.emit(vm::OpCode::OpNegate);
    } else {
        panic!("opcode error");
    }
}

fn compile_binary(
    chunk: &mut vm::Chunk,
    op: &str,
    expr_pair: &(ast::ExpressionStmt, ast::ExpressionStmt),
) {
    let (lhs, rhs) = expr_pair;
    compile_expression(chunk, lhs);
    compile_expression(chunk, rhs);
    // TODO: refactor
    if op.starts_with("+") {
        chunk.emit(vm::OpCode::OpAdd);
    } else if op.starts_with("-") {
        chunk.emit(vm::OpCode::OpSubtract);
    } else if op.starts_with("/") {
        chunk.emit(vm::OpCode::OpDivide);
    } else if op.starts_with("*") {
        chunk.emit(vm::OpCode::OpMultiply);
    } else {
        panic!("opcode error");
    }
}

fn compile_print_expression(chunk: &mut vm::Chunk, print_stmt: &ast::PrintStmt) {
    compile_expression(chunk, &print_stmt.expr);
    chunk.emit(vm::OpCode::Print);
}

#[test]
fn tests() {
    {
        let input = "1.25;";
        let mut parser = crate::Parser::new(input);
        let stmt = parser.statement().unwrap();
        let chunk = compile(&stmt);

        let output = format!("{:?}", chunk);
        let mut dissassembled = output.lines();
        assert_eq!(dissassembled.next(), Some("=== main ==="));
        assert_eq!(dissassembled.next(), Some("0001 - Const 0 (1.25)"));
    }
    {
        let input = "-((1.25 + 3.5) / 5.75);";
        let mut parser = crate::Parser::new(input);
        let stmt = parser.statement().unwrap();
        let chunk = compile(&stmt);

        let output = format!("{:?}", chunk);
        let mut dissassembled = output.lines();
        assert_eq!(dissassembled.next(), Some("=== main ==="));
        assert_eq!(dissassembled.next(), Some("0001 - Const 0 (1.25)"));
        assert_eq!(dissassembled.next(), Some("0003 - Const 1 (3.5)"));
        assert_eq!(dissassembled.next(), Some("0005 - Add"));
        assert_eq!(dissassembled.next(), Some("0006 - Const 2 (5.75)"));
        assert_eq!(dissassembled.next(), Some("0008 - Div"));
        assert_eq!(dissassembled.next(), Some("0009 - Neg"));
    }
}

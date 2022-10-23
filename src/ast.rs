#[derive(Debug,PartialEq)]
pub enum AST<'node> {
    IntVal(i32),
    Prog(Vec<Box<AST<'node>>>),
    BinaryOperation(&'node str, Box<AST<'node>>, Box<AST<'node>>),
    Parentheses(Box<AST<'node>>),
}

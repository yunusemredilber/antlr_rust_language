#[derive(Debug, PartialEq)]
pub enum AST<'node> {
    Prog(Vec<AST<'node>>),
    Stat(Box<AST<'node>>),
    Negative(Box<AST<'node>>),
    Positive(Box<AST<'node>>),
    BinaryExpr(&'node str, Box<AST<'node>>, Box<AST<'node>>),
    ParenthesizedExpr(Box<AST<'node>>),
    NumberLiteral(f64),
    Never(),
}

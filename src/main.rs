mod ast;
mod grammar;
mod parse_tree_visitor;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::Visitable;
use antlr_rust::InputStream;
use grammar::{LangLexer, LangParser};
use parse_tree_visitor::CustomParseTreeVisitor;
use std::io;

fn main() {
    println!("Write something like '(3+4)*2' to see the generated AST. ^C to exit:\n");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("reading user input");
        let output = str_to_ast(&input);
        // Codegen or evaluation etc
        dbg!(output);
    }
}

fn str_to_ast(input: &str) -> Option<ast::AST> {
    // Lexical analysis
    dbg!(input);
    let lexer = LangLexer::new(InputStream::new(input.into()));
    let source = CommonTokenStream::new(lexer);
    let mut parser = LangParser::new(source);

    let parse_tree_root = parser.prog().expect("parse tree root node");

    // Syntax analysis
    let mut parse_tree_visitor = CustomParseTreeVisitor { _nodes: vec![] };

    parse_tree_root.accept(&mut parse_tree_visitor);
    let ast_root = parse_tree_visitor._nodes.pop();
    ast_root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::AST;

    #[test]
    fn parse_complex_match() {
        let ast = str_to_ast("5-4++8+-1*16;");
        let exp_ast = AST::Prog(Vec::from([AST::Stat(Box::new(AST::BinaryExpr(
            "+",
            Box::new(AST::BinaryExpr(
                "+",
                Box::new(AST::BinaryExpr(
                    "-",
                    Box::new(AST::NumberLiteral(5.0)),
                    Box::new(AST::NumberLiteral(4.0)),
                )),
                Box::new(AST::Positive(Box::new(AST::NumberLiteral(8.0)))),
            )),
            Box::new(AST::BinaryExpr(
                "*",
                Box::new(AST::Negative(Box::new(AST::NumberLiteral(1.0)))),
                Box::new(AST::NumberLiteral(16.0)),
            )),
        )))]));
        assert!(ast.unwrap().eq(&exp_ast));
    }
}

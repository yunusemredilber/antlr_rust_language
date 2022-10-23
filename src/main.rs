mod grammar;
mod ast;
mod parse_tree_visitor;
use grammar::{
    LangLexer,
    LangParser,
};
use antlr_rust::InputStream;
use antlr_rust::tree::Visitable;
use antlr_rust::common_token_stream::CommonTokenStream;
use parse_tree_visitor::CustomParseTreeVisitor;
use std::io;

fn main() {
    println!("Write something like '(3+4)*2' to see the generated AST. ^C to exit:\n");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("reading user input");
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
    let mut parse_tree_visitor = CustomParseTreeVisitor {
        _nodes: vec![]
    };

    parse_tree_root.accept(&mut parse_tree_visitor);
    let ast_root = parse_tree_visitor._nodes.pop();
    ast_root
}

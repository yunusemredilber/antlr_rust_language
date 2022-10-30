use crate::ast::AST;
use crate::grammar::{
    Binary_exprContext, LangParserContextType, LangVisitor, Numeric_literalContext,
    Parenthesized_exprContext, ProgContext, ProgContextAttrs, Unary_exprContext,
};
use antlr_rust::tree::{ParseTreeVisitor, Visitable};
use std::borrow::Cow;

pub struct CustomParseTreeVisitor<'node> {
    pub _nodes: Vec<AST<'node>>,
}

impl<'node> ParseTreeVisitor<'node, LangParserContextType> for CustomParseTreeVisitor<'node> {}

impl<'node> LangVisitor<'node> for CustomParseTreeVisitor<'node> {
    fn visit_prog(&mut self, ctx: &ProgContext<'node>) {
        ctx.stat_all().iter().for_each(|item| item.accept(self));

        let nodes_length = self._nodes.len();
        let mut nodes: Vec<AST> = vec![0; nodes_length]
            .iter()
            .map(|_item| {
                return AST::Stat(Box::new(self._nodes.pop().expect("ast node at stack")));
            })
            .collect();
        nodes.reverse();

        let prog = AST::Prog(nodes);
        self._nodes.push(prog);
    }

    fn visit_unary_expr(&mut self, ctx: &Unary_exprContext<'node>) {
        ctx.right.as_ref().unwrap().accept(self);
        let right_node = self._nodes.pop().unwrap();

        let node = match ctx.op.as_ref().unwrap().token_type {
            crate::grammar::langlexer::MINUS => AST::Negative(Box::new(right_node)),
            crate::grammar::langlexer::PLUS => AST::Positive(Box::new(right_node)),
            _ => AST::Never(),
        };

        self._nodes.push(node);
    }

    fn visit_binary_expr(&mut self, ctx: &Binary_exprContext<'node>) {
        ctx.left.as_ref().unwrap().accept(self);
        ctx.right.as_ref().unwrap().accept(self);

        let right_node = self._nodes.pop().unwrap();
        let left_node = self._nodes.pop().unwrap();

        if let Cow::Borrowed(operator_content) = ctx.op.as_ref().unwrap().text {
            let node = AST::BinaryExpr(operator_content, Box::new(left_node), Box::new(right_node));
            self._nodes.push(node);
        } else {
            panic!("Could not resolve binary operation operator content from parse tree");
        }
    }

    fn visit_parenthesized_expr(&mut self, ctx: &Parenthesized_exprContext<'node>) {
        ctx.ch.as_ref().unwrap().accept(self);

        let child = self._nodes.pop().unwrap();

        let prog = AST::ParenthesizedExpr(Box::new(child));
        self._nodes.push(prog);
    }

    fn visit_numeric_literal(&mut self, ctx: &Numeric_literalContext<'node>) {
        let value: f64 = ctx
            .value
            .as_ref()
            .expect("Could not resolve float value from parse tree")
            .text
            .parse()
            .unwrap();

        let node = AST::NumberLiteral(value);
        self._nodes.push(node);
    }
}

use std::borrow::Cow;
use crate::grammar::{
    ProgContext,
    Int_valueContext,
    ArithmeticParserContextType,
    Binary_operationContext,
    ParenthesesContext,
    ProgContextAttrs,
    ArithmeticVisitor
};
use antlr_rust::tree::{ParseTreeVisitor, Visitable};
use crate::ast::AST;

pub struct CustomParseTreeVisitor<'node> {
    pub _nodes: Vec<AST<'node>>
}

impl<'node> ParseTreeVisitor<'node, ArithmeticParserContextType> for CustomParseTreeVisitor<'node> {}

impl<'node> ArithmeticVisitor<'node> for CustomParseTreeVisitor<'node> {
    fn visit_prog(&mut self, ctx: &ProgContext<'node>) {
        ctx.expr_all().iter().for_each(|item| item.accept(self));

        let nodes_length = self._nodes.len();
        let nodes = vec![0; nodes_length].iter().map(|_item| {
            return Box::new(self._nodes.pop().expect("ast node at stack"));
        }).collect();

        let prog = AST::Prog(nodes);
        self._nodes.push(prog);
    }

    fn visit_int_value(&mut self, ctx: &Int_valueContext<'node>) {
        if let Cow::Borrowed(value_content) = ctx.number.as_ref().unwrap().text {
            let value: i32 = value_content.parse().unwrap();
            let node = AST::IntVal(value);
            self._nodes.push(node);
        } else {
            panic!("Could not resolve int value content from parse tree");
        }
    }

    fn visit_parentheses(&mut self, ctx: &ParenthesesContext<'node>) {
        self.visit_children(ctx);
    }

    fn visit_binary_operation(&mut self, ctx: &Binary_operationContext<'node>) {
        ctx.left.as_ref().unwrap().accept(self);
        ctx.right.as_ref().unwrap().accept(self);

        let right_node = self._nodes.pop().unwrap();
        let left_node = self._nodes.pop().unwrap();

        if let Cow::Borrowed(operator_content) = ctx.op.as_ref().unwrap().text {
            let node = AST::BinaryOperation(operator_content, Box::new(left_node), Box::new(right_node));
            self._nodes.push(node);
        } else {
            panic!("Could not resolve binary operation operator content from parse tree");
        }
    }
}

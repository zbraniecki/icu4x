use super::ast;
use super::lexer::{Lexer, Token};
use std::iter::Peekable;

pub struct Parser<'p> {
    lexer: Peekable<Lexer<'p>>,
    condition: Vec<ast::AndCondition>,
}

impl<'p> Parser<'p> {
    pub fn new(input: &'p [u8]) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            condition: vec![],
        }
    }

    pub fn parse(mut self) -> Result<ast::Condition, ()> {
        loop {
            self.add_and_condition()?;
            if let Some(Token::Or) = self.lexer.peek() {
                self.lexer.next();
            } else {
                break;
            }
        }
        Ok(ast::Condition(self.condition.into_boxed_slice()))
    }

    fn add_and_condition(&mut self) -> Result<bool, ()> {
        let mut rel: Vec<ast::Relation> = vec![];
        loop {
            rel.push(self.get_relation()?);
            if let Some(Token::And) = self.lexer.peek() {
                self.lexer.next();
            } else {
                break;
            }
        }
        self.condition
            .push(ast::AndCondition(rel.into_boxed_slice()));
        Ok(false)
    }

    fn get_relation(&mut self) -> Result<ast::Relation, ()> {
        let expression = self.get_expression()?;
        let operator = match self.lexer.next() {
            Some(Token::Operator(op)) => op,
            _ => panic!(),
        };
        let range_list = self.get_range_list()?;
        Ok(ast::Relation {
            expression,
            operator,
            range_list,
        })
    }

    fn get_expression(&mut self) -> Result<ast::Expression, ()> {
        let operand = match self.lexer.next() {
            Some(Token::Operand(op)) => op,
            _ => panic!(),
        };
        let modulus = if let Some(Token::Modulo) = self.lexer.peek() {
            self.lexer.next();
            match self.lexer.next() {
                Some(Token::Number(n)) => Some(ast::Value(n)),
                _ => panic!(),
            }
        } else {
            None
        };
        let expression = ast::Expression { operand, modulus };
        Ok(expression)
    }

    fn get_range_list(&mut self) -> Result<ast::RangeList, ()> {
        let mut range_list = vec![];
        loop {
            range_list.push(self.get_range_list_item()?);
            if let Some(Token::Comma) = self.lexer.peek() {
                self.lexer.next();
            } else {
                break;
            }
        }
        Ok(ast::RangeList(range_list.into_boxed_slice()))
    }

    fn get_range_list_item(&mut self) -> Result<ast::RangeListItem, ()> {
        let value = self.get_value()?;
        if let Some(Token::DotDot) = self.lexer.peek() {
            self.lexer.next();
            let value2 = self.get_value()?;
            Ok(ast::RangeListItem::Range((value, value2)))
        } else {
            Ok(ast::RangeListItem::Value(value))
        }
    }

    fn get_value(&mut self) -> Result<ast::Value, ()> {
        match self.lexer.next() {
            Some(Token::Number(r)) => Ok(ast::Value(r)),
            _ => panic!(),
        }
    }
}

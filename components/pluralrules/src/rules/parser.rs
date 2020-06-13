use super::ast;
use super::lexer::{Lexer, Token};
use std::iter::Peekable;

pub struct Parser<'p> {
    lexer: Peekable<Lexer<'p>>,
}

impl<'p> Parser<'p> {
    pub fn new(input: &'p [u8]) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(mut self) -> Result<ast::Condition, &'static str> {
        let mut result = vec![];

        if let Some(cond) = self.get_and_condition()? {
            result.push(cond);
        } else {
            return Ok(ast::Condition(result.into_boxed_slice()));
        }

        while self.take_if(Token::Or) {
            if let Some(cond) = self.get_and_condition()? {
                result.push(cond);
            } else {
                return Err("Expected AndCondition");
            }
        }
        // If lexer is not done, error?
        Ok(ast::Condition(result.into_boxed_slice()))
    }

    fn get_and_condition(&mut self) -> Result<Option<ast::AndCondition>, &'static str> {
        if let Some(relation) = self.get_relation()? {
            let mut rel = vec![relation];

            while self.take_if(Token::And) {
                if let Some(relation) = self.get_relation()? {
                    rel.push(relation);
                } else {
                    return Err("Expected Relation");
                }
            }
            Ok(Some(ast::AndCondition(rel.into_boxed_slice())))
        } else {
            Ok(None)
        }
    }

    fn get_relation(&mut self) -> Result<Option<ast::Relation>, &'static str> {
        if let Some(expression) = self.get_expression()? {
            let operator = match self.lexer.next() {
                Some(Token::Operator(op)) => op,
                _ => return Err("Expected Operator"),
            };
            let range_list = self.get_range_list()?;
            Ok(Some(ast::Relation {
                expression,
                operator,
                range_list,
            }))
        } else {
            Ok(None)
        }
    }

    fn get_expression(&mut self) -> Result<Option<ast::Expression>, &'static str> {
        let operand = match self.lexer.next() {
            Some(Token::Operand(op)) => op,
            Some(Token::At) | None => return Ok(None),
            _ => return Err("Expected Operand"),
        };
        let modulus = if self.take_if(Token::Percent) {
            Some(self.get_value()?)
        } else {
            None
        };
        Ok(Some(ast::Expression { operand, modulus }))
    }

    fn get_range_list(&mut self) -> Result<ast::RangeList, &'static str> {
        let mut range_list = Vec::with_capacity(1);
        loop {
            range_list.push(self.get_range_list_item()?);
            if !self.take_if(Token::Comma) {
                break;
            }
        }
        Ok(ast::RangeList(range_list.into_boxed_slice()))
    }

    fn take_if(&mut self, token: Token) -> bool {
        if self.lexer.peek() == Some(&token) {
            self.lexer.next();
            true
        } else {
            false
        }
    }

    fn get_range_list_item(&mut self) -> Result<ast::RangeListItem, &'static str> {
        let value = self.get_value()?;
        if self.take_if(Token::DotDot) {
            let value2 = self.get_value()?;
            Ok(ast::RangeListItem::Range((value, value2)))
        } else {
            Ok(ast::RangeListItem::Value(value))
        }
    }

    fn get_value(&mut self) -> Result<ast::Value, &'static str> {
        if let Some(Token::Number(v)) = self.lexer.next() {
            Ok(ast::Value(v))
        } else {
            Err("Expected Value")
        }
    }
}

use std::cmp::PartialEq;
use std::slice::Iter;
use crate::pratt::Exp::{Infix, Prefix};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Exp {
    Number(i32),
    Prefix(Token, Box<Exp>),
    Infix(Box<Exp>, Token,  Box<Exp>)
}

impl Exp {
    pub fn evaluate(&self) -> i32 {
        match self {
            Exp::Number(n) => *n,
            Exp::Prefix(op, on) => match op {
                Token::Minus => - on.evaluate(),
                _ => panic!("not a prefix operator")
            }
            Exp::Infix(left, op, right) => match op {
                Token::Plus => left.evaluate() + right.evaluate(),
                Token::Minus => left.evaluate() - right.evaluate(),
                Token::Star => left.evaluate() * right.evaluate(),
                Token::Slash => left.evaluate() / right.evaluate(),
                _ => panic!("not a math operator")
            }
        }
    }
}

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        let mut parser = Parser { tokens: tokens.iter(), current: None };
        parser.advance();
        parser
    }

    pub fn parse(&mut self) -> Exp {
        let exp = self.parse_expression(0);
        if self.current != Some(Token::EOF) {
            panic!("Expected to be done but still some tokens remaining")
        }
        exp

    }

    fn advance(&mut self) {
        self.current = self.tokens.next().copied();
    }

    fn parse_expression(&mut self, precedence: i32) -> Exp {
        let mut left_exp = match self.current {
            Some(Token::Number(_)) => self.parse_number(),
            Some(Token::Minus) => {
                self.advance();
                Prefix(Token::Minus, Box::new(self.parse_expression(self.precedence(Token::Minus))))
            }
            _ => panic!("expected expression")
        };

        while let Some(op) = self.current {
            let p = self.precedence(op);

            if p > precedence {
                self.advance();
                let right_exp = self.parse_expression(p);
                left_exp = Infix(Box::new(left_exp), op, Box::new(right_exp));
            } else {
                break;
            }
        }

        left_exp
    }

    fn parse_number(&mut self) -> Exp {
        if let Some(Token::Number(n)) = self.current {
            self.advance();
            Exp::Number(n)
        } else {
            panic!("expected number")
        }
    }

    fn precedence(&self, token: Token) -> i32 {
        match token {
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash => 2,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pratt::Token::*;
    use super::*;

    #[test]
    fn test_1() {
        let tokens = vec![Number(15), EOF];
        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.parse(), Exp::Number(15));
    }

    #[test]
    #[should_panic(expected = "Expected to be done but still some tokens remaining")]
    fn test_2() {
        let tokens = vec![Number(15), Number(30), EOF];
        let mut parser = Parser::new(&tokens);
        println!("{:?}", parser.parse());
    }

    #[test]
    fn test_3() {
        let tokens = vec![Number(15), Plus, Number(6), EOF];
        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.parse(), Exp::Infix(Box::new(Exp::Number(15)), Plus, Box::new(Exp::Number(6))));
    }

    #[test]
    #[should_panic(expected = "expected expression")]
    fn test_4() {
        let tokens = vec![Number(15), Plus, Plus, EOF];
        let mut parser = Parser::new(&tokens);
        println!("{:?}", parser.parse());
    }

    #[test]
    fn test_5() {
        let tokens = vec![Number(15), Plus, Number(6), Minus, Number(2), EOF];
        let mut parser = Parser::new(&tokens);
        let left = Exp::Infix(Box::new(Exp::Number(15)), Plus, Box::new(Exp::Number(6)));
        assert_eq!(parser.parse(), Exp::Infix(Box::new(left), Minus, Box::new(Exp::Number(2))));
    }

    #[test]
    fn test_6() {
        let tokens = vec![Number(15), Plus, Number(6), Star, Number(2), EOF];
        let mut parser = Parser::new(&tokens);
        let right = Exp::Infix(Box::new(Exp::Number(6)), Star, Box::new(Exp::Number(2)));
        assert_eq!(parser.parse(), Exp::Infix(Box::new(Exp::Number(15)), Plus, Box::new(right)));
    }

    #[test]
    fn test_prefix() {
        let tokens = vec![Minus, Number(15), EOF];
        let mut parser = Parser::new(&tokens);
        assert_eq!(parser.parse(), Exp::Prefix(Minus, Box::new(Exp::Number(15))));
    }

    #[test]
    fn test_prefix_2() {
        let tokens = vec![Minus, Number(15), Minus, Number(2), EOF];
        let mut parser = Parser::new(&tokens);
        let left = Exp::Prefix(Minus, Box::new(Exp::Number(15)));
        assert_eq!(parser.parse(), Exp::Infix(Box::new(left), Minus, Box::new(Exp::Number(2))));
    }

}
use std::{iter::Peekable, str::Chars};

use rust_decimal::{Decimal, prelude::FromPrimitive};

use crate::calcu::token::Token;

pub struct Tokenizer<'a> {
    expression: Peekable<Chars<'a>>,
    reached_end: bool,
    unexpected_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        Tokenizer {
            expression: expression.chars().peekable(),
            reached_end: false,
            unexpected_char: None,
        }
    }
    pub fn get_expected_char(&self) -> Option<char> {
        self.unexpected_char
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_end {
            return None;
        }
        let next_char = self.expression.next();
        match next_char {
            Some(chr) if chr.is_numeric() => {
                let mut number = String::from(chr);
                while let Some(next) = self.expression.next_if(|c| c.is_numeric()) {
                    number.push(next);
                }
                let value: f64 = number.parse().unwrap();
                Some(Token::Number(Decimal::from_f64(value).unwrap()))
            }
            Some(chr) if chr == '+' => Some(Token::Add),
            Some(chr) if chr == '-' => Some(Token::Subtract),
            Some(chr) if chr == '*' => Some(Token::Multiply),
            Some(chr) if chr == '/' => Some(Token::Divide),
            Some(chr) if chr == '^' => Some(Token::Caret),
            Some(chr) if chr == '(' => Some(Token::LeftParen),
            Some(chr) if chr == ')' => Some(Token::RightParen),
            Some(chr) if chr.is_whitespace() => {
                while let Some(_) = self.expression.next_if(|c| c.is_whitespace()) {}
                self.next()
            }
            None => {
                self.reached_end = true;
                Some(Token::EOF)
            }
            Some(chr) => {
                self.unexpected_char = Some(chr);
                self.reached_end = true;
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use rust_decimal::dec;

    use super::*;

    #[test]
    fn test_tokenizer() {
        // let expression = "3 + 5 * (2 - 8)^2 ";
        let expression = "1+  2*3";
        let tokenizer = Tokenizer::new(expression);
        let ts = tokenizer.collect::<Vec<Token>>();
        let expected = vec![
            Token::Number(dec!(1)),
            Token::Add,
            Token::Number(dec!(2)),
            Token::Multiply,
            Token::Number(dec!(3)),
            Token::EOF,
        ];
        assert_eq!(ts, expected);
    }
    #[test]
    fn test_tokenize_with_minus() {
        let expression = "1-2*3";
        let tokenizer = Tokenizer::new(expression);
        let ts = tokenizer.collect::<Vec<Token>>();
        let expected = vec![
            Token::Number(dec!(1)),
            Token::Subtract,
            Token::Number(dec!(2)),
            Token::Multiply,
            Token::Number(dec!(3)),
            Token::EOF,
        ];
        assert_eq!(ts, expected);
    }
    #[test]
    fn test_tokenizer_with_divide() {
        let expression = "1/2";
        let expression = "1/2";
        let tokenizer = Tokenizer::new(expression);
        let ts = tokenizer.collect::<Vec<Token>>();
        let expected = vec![
            Token::Number(dec!(1)),
            Token::Divide,
            Token::Number(dec!(2)),
            Token::EOF,
        ];
        assert_eq!(ts, expected);
    }
    #[test]
    fn test_tokenizer_with_add() {
        let expression = "1+2";
        let tokenizer = Tokenizer::new(expression);
        let ts = tokenizer.collect::<Vec<Token>>();
        let expected = vec![
            Token::Number(dec!(1)),
            Token::Add,
            Token::Number(dec!(2)),
            Token::EOF,
        ];
        assert_eq!(ts, expected);
    }
    #[test]
    fn test_tokenizer_with_sub() {
        let expression = "1-2";
        let tokenizer = Tokenizer::new(expression);
        let ts = tokenizer.collect::<Vec<Token>>();
        let expected = vec![
            Token::Number(dec!(1)),
            Token::Subtract,
            Token::Number(dec!(2)),
            Token::EOF,
        ];
        assert_eq!(ts, expected);
    }
    #[test]
    fn test_tokenizer_with_mul() {
        let expression = "1*2";
        let tokenizer = Tokenizer::new(expression);
        let ts = tokenizer.collect::<Vec<Token>>();
        let expected = vec![
            Token::Number(dec!(1)),
            Token::Multiply,
            Token::Number(dec!(2)),
            Token::EOF,
        ];
        assert_eq!(ts, expected);
    }
    #[test]
    fn test_tokenize_with_crate() {
        let expression = "3 + 5 * (2 - 8)^2 ";
        let tokenizer = Tokenizer::new(expression);
        let ts = tokenizer.collect::<Vec<Token>>();
        let expected = vec![
            Token::Number(dec!(3)),
            Token::Add,
            Token::Number(dec!(5)),
            Token::Multiply,
            Token::LeftParen,
            Token::Number(dec!(2)),
            Token::Subtract,
            Token::Number(dec!(8)),
            Token::RightParen,
            Token::Caret,
            Token::Number(dec!(2)),
            Token::EOF,
        ];
        assert_eq!(ts, expected);
    }
}

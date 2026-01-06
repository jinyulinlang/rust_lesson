use crate::calcu::{
    ast::Node,
    error::{CalcError, CalcResult},
    token::{OperatorPrecedence, Token},
    tokenizer::Tokenizer,
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> CalcResult<Self> {
        let mut tokenizer = Tokenizer::new(expression);
        let current_token = tokenizer.next().ok_or_else(|| {
            CalcError::ParseError(tokenizer.get_expected_char().unwrap().to_string())
        })?;
        Ok(Parser {
            tokenizer,
            current_token: current_token,
        })
    }
    pub fn parse(&mut self) -> CalcResult<Node> {
        self.parse_expression(OperatorPrecedence::Default)
    }
}

impl<'a> Parser<'a> {
    fn next_token(&mut self) -> CalcResult<()> {
        // Parsing logic to be implemented
        self.current_token = self.tokenizer.next().ok_or_else(|| {
            CalcError::ParseError(
                self.tokenizer
                    .get_expected_char()
                    .unwrap_or_default()
                    .to_string(),
            )
        })?;
        Ok(())
    }
    fn parse_expression(&mut self, precedence: OperatorPrecedence) -> CalcResult<Node> {
        // Parsing logic to be implemented
        let mut expr = self.parse_number_or_expression()?;
        while precedence < self.current_token.get_precedence() {
            expr = self.parse_binary_expression(expr)?;
        }
        Ok(expr)
    }
    fn parse_binary_expression(&mut self, left_expr: Node) -> CalcResult<Node> {
        // Parsing logic to be implemented
        let operator = self.current_token;
        match operator {
            Token::Add => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::AddOrSubtract)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::AddOrSubtract)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::MultiplyOrDivide)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::MultiplyOrDivide)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.next_token()?;
                let right_expr = self.parse_expression(OperatorPrecedence::Power)?;
                Ok(Node::Power(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => unreachable!(),
        }
    }
    fn parse_number_or_expression(&mut self) -> CalcResult<Node> {
        // Parsing logic to be implemented
        match self.current_token {
            Token::Number(num) => {
                self.next_token()?;
                Ok(Node::Number(num))
            }
            Token::Subtract => {
                self.next_token()?;
                let node = self.parse_expression(OperatorPrecedence::Negative)?;
                Ok(Node::Negative(Box::new(node)))
            }

            Token::LeftParen => {
                let node = self.parse_expression(OperatorPrecedence::Default)?;
                if self.current_token != Token::RightParen {
                    if self.current_token == Token::EOF {
                        return Err(CalcError::ParseError("不完整的运算表达式 ')'".to_string()));
                    }
                    return Err(CalcError::ParseError(format!(
                        "期望 ‘）’ ,但是发现 {}'",
                        self.tokenizer.get_expected_char().unwrap(),
                    )));
                }
                self.next_token()?;
                Ok(node)
            }
            _ => {
                if self.current_token == Token::EOF {
                    return Err(CalcError::ParseError("不完整的运算表达式".to_string()));
                }
                Err(CalcError::ParseError(format!(
                    "期望数字, 但是发现 '{}'",
                    self.tokenizer.get_expected_char().unwrap()
                )))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal::dec;

    #[test]
    fn test_parser() {
        let mut parser = Parser::new("1+2 *3").unwrap();
        let node = parser.parse();
        let expected = Ok(Node::Add(
            Box::new(Node::Number(dec!(1))),
            Box::new(Node::Multiply(
                Box::new(Node::Number(dec!(2))),
                Box::new(Node::Number(dec!(3))),
            )),
        ));
        assert_eq!(node, expected);
    }
}

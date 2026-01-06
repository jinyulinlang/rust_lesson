use rust_decimal::Decimal;

mod ast;
mod error;
mod parser;
mod token;
mod tokenizer;

pub fn calculate(expression: &str) -> error::CalcResult<Decimal> {
    let mut parser = parser::Parser::new(expression)?;
    let ast = parser.parse()?;
    Ok(ast.eval())
}

#[cfg(test)]
mod tests {
    use rust_decimal::dec;

    use super::*;

    #[test]
    fn test_calculate_simple_add() {
        let result = calculate("1 + 2").unwrap();
        assert_eq!(result, dec!(3));
    }
    #[test]
    fn test_calculate_simple_sub() {
        let result = calculate("1 - 2").unwrap();
        assert_eq!(result, dec!(-1));
    }
    #[test]
    fn test_calculate_simple_mul() {
        let result = calculate("1 * 2").unwrap();
        assert_eq!(result, dec!(2));
    }
    #[test]
    fn test_calculate_simple_div() {
        let result = calculate("1 / 2").unwrap();
        assert_eq!(result, dec!(0.5));
    }
    #[test]
    fn test_calculate_simple_crate() {
        let result = calculate("1 ^ 2").unwrap();
        assert_eq!(result, dec!(1));
    }
}

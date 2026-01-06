use thiserror::Error;

pub type CalcResult<T> = Result<T, CalcError>;
#[derive(Debug, PartialEq, Error)]
pub enum CalcError {
    #[error("Invalid input")]
    InvalidInput,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Unknown error")]
    Unknown,
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),
}

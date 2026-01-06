use rust_decimal::{Decimal, MathematicalOps};

#[derive(Debug, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Power(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(Decimal),
}

impl Node {
    pub fn eval(&self) -> Decimal {
        use Node::*;
        match self {
            Add(left, right) => left.eval() + right.eval(),
            Subtract(left, right) => left.eval() - right.eval(),
            Multiply(left, right) => left.eval() * right.eval(),
            Divide(left, right) => left.eval() / right.eval(),
            Power(left, right) => left.eval().powd(right.eval()),
            Negative(node) => -node.eval(),
            Number(n) => *n,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal::dec;
    #[test]
    fn test_eval() {
        let node = Node::Number(dec!(10));
        assert_eq!(node.eval(), dec!(10));
    }
    #[test]
    fn test_eval_add() {
        let node = Node::Add(
            Box::new(Node::Number(dec!(10))),
            Box::new(Node::Number(dec!(20))),
        );
        assert_eq!(node.eval(), dec!(30));
    }
    #[test]
    fn test_eval_sub() {
        let node = Node::Subtract(
            Box::new(Node::Number(dec!(10))),
            Box::new(Node::Number(dec!(20))),
        );
        assert_eq!(node.eval(), dec!(-10));
    }
    #[test]
    fn test_eval_mul() {
        let node = Node::Multiply(
            Box::new(Node::Number(dec!(10))),
            Box::new(Node::Number(dec!(20))),
        );
        assert_eq!(node.eval(), dec!(200));
    }
    #[test]
    fn test_eval_div() {
        let node = Node::Divide(
            Box::new(Node::Number(dec!(10))),
            Box::new(Node::Number(dec!(20))),
        );
        assert_eq!(node.eval(), dec!(0.5));
    }
}

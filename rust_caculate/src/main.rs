use std::io::{self, Write};
fn main() {
    // 创建一个计算器实例，使用命令行计算机
    let mut calculator = Calculator::new(CommandLineComputer);
    // 设置表达式
    calculator.set_expression();
    // 计算结果
    let result = calculator.calculate();
    // 输出结果
    println!("Result: {}", result);
}

struct CommandLineComputer;

trait Computer {
    fn compute(&self, input: &str) -> i32;
}

impl Computer for CommandLineComputer {
    fn compute(&self, input: &str) -> i32 {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut op: Option<char> = None;
        for c in input.trim().chars() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if op.is_none() {
                        num1.push(c);
                    } else {
                        num2.push(c);
                    }
                }
                '+' | '-' | '*' | '/' if op.is_none() => {
                    op = Some(c);
                }
                _ if c.is_whitespace() => continue,
                _ => {
                    panic!("invalid input: {}", input);
                }
            }
        }
        if num1.is_empty() || num2.is_empty() || op.is_none() {
            panic!("invalid input: {}", input);
        }
        let num1: i32 = num1.parse().unwrap_or(0);
        let num2: i32 = num2.parse().unwrap_or(0);
        let op = op.unwrap_or('+');
        match op {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => unreachable!(),
        }
    }
}

struct Calculator<C: Computer> {
    computer: C,
    expr: String,
}

impl<C: Computer> Calculator<C> {
    fn new(computer: C) -> Self {
        Self {
            computer,
            expr: String::new(),
        }
    }

    fn set_expression(&mut self) {
        println!("Please enter an expression :");
        // 清空输入缓冲区
        self.expr.clear();
        // 刷新输出缓冲区
        io::stdout().flush().unwrap();
        // 读取用户输入
        io::stdin()
            .read_line(&mut self.expr)
            .expect("Failed to read line");
    }

    fn calculate(&self) -> i32 {
        self.computer.compute(&self.expr)
    }
}

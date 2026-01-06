mod calcu;
fn main() {
    println!("欢迎使用 Rust 高性能计算器！");
    println!("请输入一个数学表达式进行计算：");
    println!("输入 'q' 退出程序");
    let mut input = String::new();

    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "q" {
            break;
        }
        if input.trim().is_empty() {
            continue;
        }

        let result = calcu::calculate(&input);
        match result {
            Ok(value) => println!("计算结果是: {}", value),
            Err(e) => println!("计算错误: {:?}", e),
        }
    }
}

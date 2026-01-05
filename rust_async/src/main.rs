#[tokio::main]
async fn main() {
    // fs::write("demo.txt", "Hello, world!").expect("Unable to write file");
    let ret = tokio::fs::read_to_string("demo.txt").await;
    match ret {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    };
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        println!("Hello, world!");
    })
    .await
    .unwrap();
    // 同时并发的运行多个 future
    tokio::join!(say_hello(), say_goodbye());
    // 竞争多个 future，谁先完成就执行谁
    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_secs(3)) => {
            println!("Timeout reached!");
        }
        _ = say_hello() => {
            println!("say_hello completed first!");
        }
        _ = say_goodbye() => {
            println!("say_goodbye completed first!");
        }
        _ = tokio::signal::ctrl_c() => {
            println!("Ctrl-C received!");
        }
    }
}

async fn say_hello() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Hello, async world!");
}
async fn say_goodbye() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Goodbye, async world!");
}

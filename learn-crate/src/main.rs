use serde::{Deserialize, Serialize};

use crate::error::AppResult;

mod error;
mod math;
mod utils;

fn main() -> AppResult<()> {
    let ret = math::add(1, 2);
    println!("{}", ret);
    utils::test();
    utils::get();
    let p = Person {
        name: "Rust".to_string(),
        age: 18,
    };
    // 序列化person
    let json = utils::to_json(&p)?;
    println!("serialized json is {}", &json);
    // 反序列化person
    let p: Person = utils::from_json(&json)?;
    println!("deserialized person is {:?}", &p);
    let name = "Alice";
    let desc = "A software engineer";
    let employee = Employee {
        name: name,
        desc: desc,
        age: 30,
    };
    let emp_json = utils::to_json(&employee)?;
    println!("serialized employee json is {}", &emp_json);
    let emp: Employee = utils::from_json_with_life_cycle(&emp_json)?;
    println!("deserialized employee is {:?}", &emp);
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug, Deserialize, Serialize)]
struct Employee<'a> {
    name: &'a str,
    desc: &'a str,
    age: u8,
}

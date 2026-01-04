use std::{cell::RefCell, rc::Rc};
fn main() {
    let name = RefCell::new(String::from("Link"));
    println!("name is {}", name.borrow());
    name.borrow_mut().push_str(" the hero");
    println!("name is {}", name.borrow());

    let name = Rc::new(String::from("Tomer"));
    let user = User {
        name: Rc::clone(&name),
    };
    let employee = Employee {
        name: Rc::clone(&name),
    };
    println!("user name is {}", user.name);
    println!("employee name is {}", employee.name);
}

struct User {
    name: Rc<String>,
}
struct Employee {
    name: Rc<String>,
}

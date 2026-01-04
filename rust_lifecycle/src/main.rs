use std::fmt::Debug;

fn main() {
    let player = Player::new("Tom");
    player.say();
    let game = Game::new(player);
    game.play();
}

trait Person: Debug {
    fn get_name(&self) -> String;
    fn say(&self) {
        println!("My name is {}", self.get_name());
    }
}

#[derive(Debug)]
struct Player {
    name: String,
}
impl Player {
    fn new<T: AsRef<str>>(name: T) -> Self {
        Self {
            name: String::from(name.as_ref()),
        }
    }
    // fn new(name: impl Into<String>) -> Player {
    //     Self { name: name.into() }
    // }
}
impl Person for Player {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug)]
struct NewBorn;

impl Person for NewBorn {
    fn get_name(&self) -> String {
        String::from("Baby")
    }
}

#[derive(Debug)]
struct Poet {
    name: String,
}
impl Person for Poet {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct Game<T: Person> {
    person: T,
}

impl<T: Person> Game<T> {
    fn new(person: T) -> Game<T> {
        Game { person }
    }

    fn play(&self) {
        println!("{} playing games!", self.person.get_name());
    }
}

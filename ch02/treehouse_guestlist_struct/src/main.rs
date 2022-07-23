#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn main() {
    let visitor_list = [
        Visitor::new("a", "hello a"),
        Visitor::new("b", "hello b b"),
        Visitor::new("c", "hello c c c"),
    ];

    println!("Hello, what is your name?");
    let name = what_is_your_name();

    let know_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);
    match know_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("Not in a list"),
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name
        .trim()
        .to_lowercase()
}

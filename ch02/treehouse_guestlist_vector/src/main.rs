#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
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
    let mut visitor_list = vec![
        Visitor::new("a", "hello a"),
        Visitor::new("b", "hello b b"),
    ];

    loop {
        println!("What is your name? (empty for exit)");
        let name = what_is_your_name();
        if name.is_empty() { break; }

        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                println!("{} is not in the list", name);
                visitor_list.push(Visitor::new(&name, "hello new friend"));
            }
        }
    }

    println!("The final list of visitors");
    println!("{:#?}", visitor_list);
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
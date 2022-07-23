#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome { }", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome { }", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("no alcohol");
                }
            }
            VisitorAction::Probation => {
                println!("{} is prob memner", self.name);
            }
            _ => println!("Go away"),
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn main() {
    let visitor_list = vec![
        Visitor::new("a", VisitorAction::Accept, 45),
        Visitor::new("b", VisitorAction::AcceptWithNote {
            note: String::from("milk")
        }, 15),
        Visitor::new("c", VisitorAction::Refuse, 49),
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
            }
        }
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

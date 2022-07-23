use std::io::stdin;

fn main() {
    let visitors_list = ["a", "b", "c"];

    println!("Hello, what is your name?");
    let name = what_is_your_name();

    let mut allow_them_in = false;

    for visitor in visitors_list {
        if visitor == name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Welcome {}", name);
    } else {
        println!("Sorry you're not in the list");
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

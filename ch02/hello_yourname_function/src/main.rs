use std::io::stdin;

fn main() {
    println!("Hello, what is your name?");
    let name = what_is_your_name();
    println!("Hello {:?}", name);
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

use std::io;

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let reversed_string = reverse_string(input);
    println!("Reversed string: {}", reversed_string);
}

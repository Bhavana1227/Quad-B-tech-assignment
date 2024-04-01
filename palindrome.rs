use std::io;

fn is_palindrome(s: &str) -> bool {
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim(); // Remove trailing newline

    println!("Is '{}' a palindrome? {}", input, is_palindrome(input));
}

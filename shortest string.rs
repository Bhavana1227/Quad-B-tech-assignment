use std::io;

fn shortest_word(input: &str) -> Option<&str> {
    input.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    if let Some(shortest) = shortest_word(&input) {
        println!("The shortest word is: {}", shortest);
    } else {
        println!("No words found in the string");
    }
}

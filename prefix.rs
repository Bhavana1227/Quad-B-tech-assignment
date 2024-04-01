use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = strings[0].clone();
    
    for s in strings.iter().skip(1) {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    
    prefix
}

fn main() {
    println!("Enter a set of strings separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let strings: Vec<String> = input.trim().split_whitespace().map(String::from).collect();

    let longest_prefix = longest_common_prefix(&strings);
    if longest_prefix.is_empty() {
        println!("No common prefix found.");
    } else {
        println!("The longest common prefix is: {}", longest_prefix);
    }
}

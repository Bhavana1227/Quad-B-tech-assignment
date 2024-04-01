use std::io;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; // Return None if k is greater than the length of the array
    }

    let mut sorted_arr = arr.to_vec(); // Create a mutable copy of the array
    sorted_arr.sort(); // Sort the array in ascending order

    Some(sorted_arr[k - 1]) // Return the kth smallest element (index k - 1)
}

fn main() {
    println!("Enter the elements of the array separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter the value of k:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Invalid input");

    if let Some(kth) = kth_smallest(&arr, k) {
        println!("The {}th smallest element is: {}", k, kth);
    } else {
        println!("Array length is smaller than k");
    }
}

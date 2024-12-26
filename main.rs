use std::fs;
use std::error::Error;
use std::io;


fn get_input_as_string()-> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read Path");
    let input = input.trim();
    //println!("{}", input);
    return input.to_string();
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize lists and count
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    let mut count: i64 = 0;

    // Read the input txt file into a String from the input
    let input = fs::read_to_string(get_input_as_string())?;



    // Transform the String into a Vector (with each number separated)
    let string_to_integer: Vec<i32> = input
        .split_whitespace() // Split by whitespace
        .filter_map(|s| s.parse().ok()) // Parse integers
        .collect();

    // Split the Vector into two lists
    for (i, &num) in string_to_integer.iter().enumerate() {
        if i % 2 == 0 {
            list_1.push(num); // Even indices
        } else {
            list_2.push(num); // Odd indices
        }
    }

    // Sort the lists
    list_1.sort_unstable();
    list_2.sort_unstable();

    // Iterate through the lists and calculate the differences
    let len = std::cmp::min(list_1.len(), list_2.len()); // Ensure we don't go out of bounds
    for i in 0..len {
        count += (list_1[i] - list_2[i]).abs() as i64; // Add absolute differences
    }

    // Print the total difference
    println!("{}", count);

    Ok(())
}

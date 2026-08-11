use std::io;

fn main() {
    let array = [2, 50, 502, 5020, 20500];

    let mut index = String::new();

    println!("Please enter an index to retrieve a number from the mystery array :)");

    io::stdin()
        .read_line(&mut index)
        .expect("Could not parse :(");

    // Rust errors when the index is out of bounds, unlike C!
    let index: usize = index
        .trim()
        .parse()
        .expect("Index not valid.");

    let element = array[index];

    println!("Your number at index {index} is {element}");
}
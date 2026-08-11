use std::io;

fn main() {
    auxiliary(5);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not parse!");

    let num_input :u32 = input.trim().parse().expect("Not a valid numerical input.");

    auxiliary(num_input);
}

fn auxiliary(x: u32) {
    println!("Auxiliary has been given argument {x}");
}
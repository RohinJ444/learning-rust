use std::io;

fn main() {
    auxiliary(5);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not parse!");

    let num_input :u32 = input
                            .trim()
                            .parse()
                            .expect("Not a valid numerical input.");

    auxiliary(num_input);

    let mut unit: String = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Could not parse!");

    labeled_num(num_input, unit);

    let x_signed: i32 = num_input.try_into().expect("Lossy conversion: {x} is too big to convert to i32 from u32");

    println!("Adder test: {num_input} and 5 gets you {adder_test}", adder_test = five_adder(x_signed));

}

fn auxiliary(x: u32) {
    println!("Auxiliary has been given argument {x}");
}

fn labeled_num(x: u32, label: String) {
    println!("Value {x} measured in unit {label}")
}

fn five_adder(x: i32) -> i32 {
    println!("We will now add 5 to {x}");

    return x + 5;
}
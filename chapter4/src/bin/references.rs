fn main() {
    let s1 = String::from("hello");

    // Passing s1 as a reference into calculate_length allows calculate_length to use the value of s1
    // without having to touch ownership. since s is an immutable reference, it is immutable in calculate_length 
    // and attempting to modify s will throw an error
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("hello");

    // Passing s2 as a reference into calculate_length allows calculate_length to use the value of s1
    // without having to touch ownership. Here, some_string is a mutable reference to s2, so we can change it in change.
    // The catch with mutable references is that they must be the sole reference to that value. Attempting
    // to create any other reference to a value that already has a mutable reference will error.
    change(&mut s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn main() {
    let mut str = String::from("Hello there, world!");

    let word = first_word(&str);

    println!("{word}");

    str.clear();

    // no compile or runtime, errors for the below function calls, str just prints as blank! 
    // this is unclean behavior, and overall a clunky implementation of first_word. how can we do better...?
    println!("{word}");
    println!("{str}");

    // intro to how slices work (they offer a soln to above unclean behavior)
    slice_example();

    let mut str2 = String::from("Hello there, world!");
    let word_slice = first_word_slice(&str2);

    println!("{word_slice}");
}

fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn slice_example() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}");
    println!("{world}");

    let hello = &s[..5];
    let world = &s[6..s.len()];

    println!("{hello}");
    println!("{world}");
}

// slice type is indicated by &str
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
                                      // s is not yet valid or in scope
    let s0 = String::from("hello");    // s is valid from this point forward

    // String types have unpredictable length at compile time, so s is itself 
    // a String struct on the stack that contains a pointer to a sequence of char bytes in heap. 

    let mut s1 = s0; // s is no longer valid from this point forward as there can only be one
                // owner of a specific chunk of heap-allocated memory at a time. The heap memory
                // remains intact and untouched at this point; the String structs on stack have shuffled around\

    s1 = String::from("ahoy"); // at this point, Rust will drop the original memory allocated for "hello" on heap
                                   // and free it. s1 now has a new pointer to newly allocated heap memory for "ahoy"

    // to do a deep copy of a String, rather than move ownership like we did earlier from s0 to s1, use .clone()
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // s1 and s2 will remain valid until it goes out of scope, then Rust will free the corresponding memory
    // on heap
}

// An aside: Integers and types that can be easily stored on the stack use simple, deep copies by default. Rust has a Copy trait for types
// that indicates whether or not a type uses deep copies or moves. If a type or its parts have implemented the Drop trait, which
// provisions instructions for what to do upon freeing memory, we cannot implement a Copy trait (will get compile time error).
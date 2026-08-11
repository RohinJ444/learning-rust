fn main() {
    let cond = true;
    let number = if cond {5} else {6};

    println!("The value of number is {number} seeing as condition is {cond}");

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("Some loopitly loop turned that into {loop_num}", loop_num = loop_de_loop(number));
}

fn loop_de_loop(mut counter: u32) -> u32 {
    let counter_init: u32 = counter;

    let result = loop {
        counter += 1;

        if counter == (counter_init * 10) {
            break counter * 2;
        }
    };

    return result;
}
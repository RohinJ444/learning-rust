fn main() {
    println!("Hello, world!");

    let vignesh: i32 = 5;
    println!("Hey there Vignesh I'm {vignesh}");

    let x = (500, 64, 'a');
    let mut five_hundo = x.0;
    five_hundo = 600;
    println!("{x:?}");

    let mut y = (300, 200, 100);
    let y_naught = y.0;
    let mut y_one = y.1;
    y_one = y_naught;
    println!("{y_naught}");
    println!("{y:?}");
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    //const MAX_POINTS: u32 = 100_000;

    let t = 5;
    let t = t + 1;
    let t = t * 2;
    println!("The value of t is {}", t);

    let spaces = "  ";
    let spaces = spaces.len();

    println!("Spaces:{}",spaces);

    //let mut spaces = "  ";
    //spaces = spaces.len();
    //
    let guess: u16  = "42".parse().expect("Not a number!");
    println!("Guess {}",guess);

    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

    //addition
    let sum = 5 + 10;
    //subtraction
    let difference = 95.5 - 4.3;
    //
}

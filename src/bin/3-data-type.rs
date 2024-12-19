fn main() {
    // Floating-Point Types
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32


    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


    //Boolean Type
    let t = true;

    let f: bool = false;


    // Character Type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    println!("Typle {:?}", tup2);
    println!("Typle {:?}", tup2.0);

    let (x, y, z) = tup; //destruct

    println!("The value of y is: {y}");

    println!("Empty tuple: {:?}", ());

    //array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array {:?}", a);

    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("month {:?}", months[11]);

}
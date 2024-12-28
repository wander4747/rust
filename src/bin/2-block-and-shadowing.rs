fn main() {
    println!("Block and Shadowing");

    const X: i32 = 5;
    let y: i32 = 6;
    let mut z:i32 = 10;

    z+= 1;

    println!("Values: X= {X}, y={y}, z={z}");

    { //internal block
        const X: i32 = 100;
        let y: i32 = 200;
        let mut z:i32 = 300;
    
        z+= 1;

        println!("Internal Values: X= {X}, y={y}, z={z}");
    }

    println!("Values ​​after the inner block are: X= {X}, y={y}, z={z}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The internal value is: {x}");
    }

    println!("The value after the inner block is: {x}");

    let spaces:&str = "   ";
    let spaces: usize = spaces.len();
    println!("The value of spaces is: {spaces}");

	let mut spaces2 = "   ";
    println!("The value of spaces2 is: {spaces2}");
	spaces2 = "qwerty";
    println!("The value of spaces2 is: {spaces2}");
}
fn main() {
    println!("Block and Shadowing");

    const X: i32 = 5;
    let y: i32 = 6;
    let mut z:i32 = 10;

    z+= 1;

    println!("Valores são: X= {X}, y={y}, z={z}");

    { //internal block
        const X: i32 = 100;
        let y: i32 = 200;
        let mut z:i32 = 300;
    
        z+= 1;

        println!("Valores internos são: X= {X}, y={y}, z={z}");
    }

    println!("Valores depois do bloco interno são: X= {X}, y={y}, z={z}");


    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("O valor interno é: {x}");
    }

    println!("O valor depois do bloco interno é: {x}");

    let spaces:&str = "   ";
    let spaces: usize = spaces.len();
    println!("O valor de spaces é: {spaces}");

	let mut spaces2 = "   ";
    println!("O valor de spaces2 é: {spaces2}");
	spaces2 = "qwerty";
    println!("O valor de spaces2 é: {spaces2}");
}
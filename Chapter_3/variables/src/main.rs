
const MAX_POINTS: u32 = 100_000;

fn main() {

    // Declaring a variable as mutable, so it can be changed
    let mut abc = 5;

    println!("The value of x is: {}", abc);

    // Changing a mutable variable
    abc = 6;

    println!("The value of x is: {}", abc);

    // Print out the global const 
    println!("MAX_POINTS CONST: {}", MAX_POINTS);

    // Using shadowing to declare variables and change its type 
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces length: {}", &spaces);

}

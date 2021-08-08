
// Declaring the return type value 
fn five() -> i32 {
    5
}

fn main() {

    // Calling a function with argument 
    another_function(10, 22);

    // ERR: Expected statement, found expression 
    // let x = (let y = 6);

    let mut x = 5;

    let y = {
        x = x + 3;
        x = x + 1;
        x + 1
    };

    println!("The value of y is: {} ", y);

    let x = five(); 

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x); // Comments <- Comments.. 

    // Comments, comments, comments, <- Chapter 3.4 is comments, this is how you comment // comments. 
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Defining a function with parameters
fn another_function(x: i32, y: i32) {

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

}
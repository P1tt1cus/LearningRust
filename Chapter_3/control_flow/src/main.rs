#![allow(unreachable_code)]

fn main() {
    
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut thing = false;

    // Error: Expects a bool
    if number == 3 {
        thing = true;
    }

    if thing {
        println!("number was three");
    }

    if number != 0 {
        println!("number was something other than zer0");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut counter = 0;

    // Loop that breaks and returns result
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    // Nesting labels for loops 
    'outer: loop {
        println!("Entered the outer loop!");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("You'll never see me");
    }

    println!("The result is: {}", result);

    // Conditional loop with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // While loop over the elements of a collection 

    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 99];

    println!("The length of A is: {}", a.len());

    for number in (0..a.len()).rev() {
        println!("Array value for pos: {} is {}", number, a[number]);
        println!("{}!", number);
    }
    println!("LIFTOFF!!!!!!!!");
}

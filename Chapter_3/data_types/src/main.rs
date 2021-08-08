fn main() {
 // You must declare a type annotation  
    // let guess: u32 = "42".parse().expect("Not a number!");

    // Floating-point types 
    let float = 2.0; //f64
    let float32: f32 = 3.0; //f32

    println!("F64: {}",float);
    println!("F32: {}", float32);

    // Numeric Operations 

    // addition 
    let _sum = 5+10;

    // subtraction 
    let _difference = 95.5 - 4.3; 

    // multiplication 
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2; 

    // remainder
    let _remainder = 43 % 5; 

    // Boolean Type
    let _bool_true = true;

    // with explicit type annotation 
    let _bool_false: bool = false;

    // Hell yes, emoji's 
    let emoji = "😻😻😻😻😻";
    println!("{}",emoji);

    // Declaring a tuple with annotations 
    let tup: (char, u32, u32) = ('1', 4, 0x44);
    
    // Declaring variables from the tuple 
    let (_a, _b, _c) = tup;
    
    // Declaring a variable from a tuple element
    let one = tup.0;

    println!("{}", one);

    // Array Type
    let _array_nums = [1,2,3,4,5,6,7,8,9];

    let months = ["Jan", "Feb", "March"];

    let _array_num_annotations: [i32;5] = [1,2,3,4,5];

    // Initialising an array with duplicate values 
    let dupe_array = [5; 10];
    println!("dupeArray: {:?}", dupe_array);

    // Selecting a element from an array by specifying the index
    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");
    
    let chosen_element = months[index];

    println!("The value of the element at index {} is: {}", index, chosen_element);
}

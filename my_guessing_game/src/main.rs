use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {


    let x2b = "guess";

    
    println!("Guess the number!"); 

    let x2a = "_if_";

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    let mut attempts = 0;

    let x2k = "you_can_";

    let x2d = "ctf";

    let max_tries= 3;

    let mut attempts_left = attempts - max_tries;

    let x2c = "{";

    let lop = "0sa990dk21337sa-d90as-0";

    let x2n = "}";

    let flag = [x2d, x2c, x2b, x2a, x2k, lop, x2n].join("");

    loop {
        
        println!("Please input your guess:");

        // Declares a mutable variable, variables are immutable by default
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Expect call, attempts to convert the string into u32 or errors 
        // let guess: u32 = guess.trim().parse().expect("Please type a number, u fek!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts_left = max_tries - attempts;

        match attempts.cmp(&max_tries) {
            Ordering::Less => println!("You have {} attempts left", attempts_left),
            Ordering::Equal => {
                println!("You have run out of attempts!"); 
                break;
            },
            Ordering::Greater => println!("wut"),
        };
        println!("You guessed: {}", guess);

        attempts = attempts + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! {}", flag);
                break;
            }
        };
    }
}


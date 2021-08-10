fn main() {
    
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}. ", s1, len);

    let mut s = String::from("ello");

    change(&mut s);

    println!("{}", s);

    let test: String = String::from("woop de scoop boop");

    let abc = first_word(&test);

    println!("This: {}", abc);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

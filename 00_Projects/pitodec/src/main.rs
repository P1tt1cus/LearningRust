
// fn get_type<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
// }

fn main() {
    // String to encode
    let plain_text = String::from("thisIsAString++==!!()");

    // 
    println!("This is the string: {}", plain_text);

    // Declare a VEC to append decimals 
    let mut binary_vec: Vec<String> = Vec::new();

    for c in plain_text.clone().into_bytes() {

        let binary_value = &format!("{:b}", c);

        let binary_len = binary_value.len();

        if binary_len == 7 {
            let binary_value: &String = &format!("0{}", binary_value);
            let formatted_binary: String = binary_value.clone();
            binary_vec.push(formatted_binary);
        } 
        else {
            let binary_value = &format!("00{}", binary_value);
            let formatted_binary: String = binary_value.clone();
            binary_vec.push(formatted_binary);
        }

    };

    println!("{:?}", binary_vec);

}

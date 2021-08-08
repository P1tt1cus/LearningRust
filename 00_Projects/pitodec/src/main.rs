
// fn get_type<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
// }

fn main() {
    // String to encode
    let message = String::from("thisIsAString++=@#IlikeCatsandFISHANDCATs=!!()");

    encode(message)

}

fn encode(message: String) {
    // Display line of string to encode 
    println!("This is the string: {}", message);

    // Padded binary value
    let mut full_binary_val: String = "".to_string();

    // Mutable Strings used to store the first four and last four binary characters
    let mut first_four: String = "".to_string();
    let mut last_four: String = "".to_string();

    // Vector something something
    let mut unencoded_vec: Vec<String> = Vec::new();  
    let mut encoded_vec: Vec<String> = Vec::new();

    let first_four_enc: String = "1100".to_string();
    let last_four_enc: String = "1000".to_string();

    // For loop over the cloned string and convert it into bytes 
    for c in message.clone().into_bytes() {

        // Return binary string value of byte
        let binary_value = &format!("{:b}", c);

        // Calculate length of binary value (binary lengths have no leading 0's)
        let binary_len = binary_value.len();

        // Add missing 0's 
        if binary_len == 7 {

            // Add missing 0 to binary_value
            let binary_value: &String = &format!("0{}", binary_value);
            
            // Store formatted binary value in full_binary_val
            full_binary_val = (&binary_value).to_string();

        } 
        else {
            
            // Add missing 00 to binary_value
            let binary_value = &format!("00{}", binary_value);
        
            // Store formatted binary value in full_binary_val
            full_binary_val = (&binary_value).to_string();
        }

        // Pushes the full binary value to the binary vec
        for (index, char) in full_binary_val.chars().enumerate() {
            
            // Append first four binary values to first_four
            if index < 4 {
                first_four.push(char);
            } 
            // Append last four binary values to last_four
            else {
                last_four.push(char);
            }

        }

        // Add unencoded padded binary value to vector
        unencoded_vec.push(full_binary_val);

        // First four encoded with 0101 at the front
        let mut encoded_binary_value = first_four_enc.to_string();

        // First four first
        for char in first_four.chars() {
            encoded_binary_value.push(char);
        } 

        // Debugging: Confirming first four was encoded correctly
        // println!("FIRST FOUR: {} FIRST FOUR ENCODED: {}", first_four, encoded_binary_value);

        // Push the encoded first four into vector 
        encoded_vec.push(encoded_binary_value);
        

        // Last four encoded with 1100 at the front
        encoded_binary_value = last_four_enc.to_string();

        // Last four second
        for char in last_four.chars() {
            encoded_binary_value.push(char);
        }

        // Push the encoded second four into vector
        encoded_vec.push(encoded_binary_value);

        // Clear out the first and last four mutable String's
        first_four = "".to_string();
        last_four = "".to_string();

    };

    // Display unencoded message in binary
    println!("\nUNENCODED VECTOR: {:?}\n", unencoded_vec);
    
    let mut unencoded_value = "".to_string();

    for binary in unencoded_vec {
        for char in binary.chars() {
            unencoded_value.push(char);
        }
    }

    println!("UNENCODED: {}\n", unencoded_value);

    // Display encoded message in binary 
    println!("ENCODED VECTOR: {:?}\n", encoded_vec);

    let mut encoded_value = "".to_string();

    for binary in encoded_vec {
        for char in binary.chars() {
            encoded_value.push(char);
        }
    }

    println!("ENCODED: {}\n", encoded_value);

}


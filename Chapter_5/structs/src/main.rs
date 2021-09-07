
struct Analyst {

    username: i32,
    age: bool,
    competency: String, 

}


fn main() {

    let mut aaron = Analyst {
        username: 2, 
        age: false, 
        competency: String::from("potato")
    };

    println!("Username: {}, Age: {}, Competency: {}", 
        aaron.username, 
        aaron.age, 
        aaron.competency
    );
    
    aaron.age = true;

    println!("Username: {}, Age: {}, Competency: {}", 
    aaron.username, 
    aaron.age, 
    aaron.competency
    );    
}

fn build_analyst(username: i32, age: bool, comepetency: String) -> Analyst {

    Analyst {
        username: username, 
        age: age, 
        competency: competency, 
    }
}

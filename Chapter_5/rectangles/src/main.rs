
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    
    let aarons_box = Rectangle {
        height: 40, 
        width: 10,
    };

    let a_box = aarons_box.area();

    println!(
        "The area of the rectangle is {} square pixels.",
        a_box
    );

    println!(
        "Struct! {:?}",
        a_box
    );

}

// fn area(rect: &Rectangle) -> u32 {
//     rect.height * rect.width    
// }

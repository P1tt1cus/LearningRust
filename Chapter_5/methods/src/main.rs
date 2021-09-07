#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {

        self.height >= rect.height && self.width >= rect.width

    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30, 
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Can rect1 hold rect2? {:?}", rect1.can_hold(&rect2));
    
}

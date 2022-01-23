#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("area of rect1 is {}", area(&rect1));

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
  
    dbg!(&rect2);
}


fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
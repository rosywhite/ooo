#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

impl Rectangle {
  fn area2(&self) -> u32 {
      self.width * self.height
  }
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

    let rect3 = Rectangle {
      width: 30,
      height: 50,
  };

  println!(
      "The area of the rectangle is {} square pixels.",
      rect3.area2()
  );
}



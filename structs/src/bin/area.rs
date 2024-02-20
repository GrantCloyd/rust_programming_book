#[derive(Debug)]
struct Rectangle {
  width: u32, 
  height: u32
} // end struct

impl Rectangle {
  fn area(&self) -> u32 {
    &self.height * &self.width
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    (self.width > other.width && self.height > other.height) ||
    (self.height > other.width && self.width > other.height)
  }

  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size, 
      height: size
    }
  } 
} // end Rectangle impl

fn main() {
  let rect = Rectangle {
    width: 30, 
    height: 50
  };

  let rect2 = Rectangle {
    width: 25, 
    height: 30
  };

  let rect3 = Rectangle {
    width: 40, 
    height: 25
  };

  let rect4 = Rectangle {
    height: 60, 
    width: 35
  };

  println!("rect: {:#?}", rect);

  println!(
    "The area of the rectangle os {} sqaure pixels.",
    rect.area()
  );

  println!(
    "Rectangle one can hold rectangle two? Answer: {}",
    rect.can_hold(&rect2)
  );

  println!(
    "Rectangle one can hold rectangle three? Answer: {}",
    rect.can_hold(&rect3)
  );

  println!(
    "Rectangle one can hold rectangle four? Answer: {}",
    rect.can_hold(&rect4)
  );

  let rect5 = Rectangle::square(12);

  println!("Sqaure impl: {:#?}", rect5)
}
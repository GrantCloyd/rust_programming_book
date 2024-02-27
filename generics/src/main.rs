struct Point<T> {
   x: T, 
   y: T, 
}

//valid for any generic type
impl <U> Point<U>{
   fn x(&self) -> &U{
      &self.x
   }
}

// only valid for f64 values
impl Point<f64> {
   fn y(&self) -> f64 {
      self.y
   }
}

fn main() {
   let number_list = vec![34, 50, 25, 100, 65];

   let largest = get_largest(number_list);
   println!("The largest number is {largest}");

   let char_list = vec!['c', 'd', 'f', 'g'];
   let next_largest = get_largest(char_list);
   println!("The largest char is {next_largest}");

   let p = Point {x: 5, y: 5};
   p.x();
   let p1 = Point {x: 5.0, y: 1.0};
   p1.y();
}

// T is a stand-in for Type - but could be called anything (foo)
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
   let mut largest: T = list[0];

   for number in list {
      if number > largest {
         largest = number;
      }
   };

   largest
}
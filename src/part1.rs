// fn main() {
//     let is_rust_fun: bool = true;
//     let letter: char = 'R';
//     let number: i32 = 42;
//     let decimal: f64 = 3.14;
    
//     println!("Rust fun: {}, Letter: {}, Number: {}, Decimal: {}", is_rust_fun, letter, number, decimal);
// }

// fn main() {
//     let tup: (i32, f64, char) = (42, 6.28, 'Z');

//     let (x, y, z) = tup; // Destructuring
//     println!("x: {}, y: {}, z: {}", x, y, z);
// }


// fn main() {
//     let arr = [10, 20, 30, 40];

//     for element in arr.iter() {
//         println!("Value: {}", element);
//     }

//     // Loop with range
//     for i in 1..5 { // `..=` includes 5
//         println!("{}", i);
//     }
// }

fn add(a: i32, b: i32) {
  println!("Sum: {}", a + b);
}

fn main() {
  add(5, 7);
}

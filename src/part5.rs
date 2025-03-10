use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct User {
  name : String,
  age : i32
}
fn main() {
    let user = User { 
      name: String::from("John Doe"), 
      age: 30 
    };
  print!("{:?}",user)
}


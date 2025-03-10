use std::fs;
// use std::vec;

// fn main() {
//     println!("Hello, world!");
//     let v = vec![1, 2, 3];
//     println!("{:?}", v);
// }


fn main() {
	let greeting_file_result = fs::read("hello.txt");
    match greeting_file_result {
        Ok(greeting) => {print!("OKay : {}", String::from_utf8_lossy(&greeting))},
        Err(er)=> {println!("Error: {}", er)}
    }
}
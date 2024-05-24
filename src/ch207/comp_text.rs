use std::fs;

fn main() {
    let a_file = "./book_python.txt";
    let b_file = "./book_rust.txt";

    let a_str = fs::read_to_string(a_file).unwrap();
    let b_str = fs::read_to_string(b_file).unwrap();

    let a_str = a_str.trim();
    let b_str = b_str.trim();

    if a_str == b_str {
        println!("ok");
    } else {
        println!("ng");
    }
}
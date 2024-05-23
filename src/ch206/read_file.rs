use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("");
        return;
    }

    let file_name = &args[1];
    let text = fs::read_to_string(file_name).unwrap();

    println!("{:?}", text);
}
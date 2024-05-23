use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 { return; }

    let text = fs::read_to_string(&args[1]).unwrap();

    let mut sum: i64 = 0;

    for i in text.split('\n') {
        match i.parse::<i64>() {
            Ok(v) => sum += v,
            Err(_) => (),
        }
    }

    println!("{}", sum);
}
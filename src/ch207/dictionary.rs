use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let dict_file = "dict.txt";
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("[USAGE] ./dictionary word");
        return;
    }

    let word = &args[1];
    let fp = File::open(dict_file).unwrap();
    let reader = BufReader::new(fp);

    for line in reader.lines() {
        match line {
            Ok(v) => match v.find(word) {
                Some(_) => { println!("{}", v); },
                None => (),
            },
            Err(_) => (),
        }
    }
}
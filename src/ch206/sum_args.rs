use std::env;

fn main() {
    let args = env::args();
    let mut total = 0.0;

    for (i, s) in args.enumerate() {
        if i ==0 { continue; }
        match s.parse::<f64>() {
            Ok(v) => total += v,
            Err(_) => (),
        }
    }

    println!("total: {}", total);
}
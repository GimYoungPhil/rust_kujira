fn main() {
    let s = "365k";
    let i = match s.parse() {
        Ok(v) => v,
        Err(_) => 0,
    };

    println!("{}", i);
}
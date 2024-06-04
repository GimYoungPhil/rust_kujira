fn main() {
    let m = gen_message();
    println!("{}", m);
}

fn gen_message() -> &str {
    let msg = String::from("Rust");
    println!("{}", msg);
    &msg
}
fn main() {
    let m = gen_message();
    println!("{}", m);
}

fn gen_message() -> String {
    let msg = String::from("Rust");
    println!("{}", msg);
    msg
}
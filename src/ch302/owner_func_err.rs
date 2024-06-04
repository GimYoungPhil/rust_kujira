fn main() {
    let g1 = String::from("Rust");
    show_message(g1);
    // println!("{}", g1);
}

fn show_message(msg: String) {
    println!("{}", msg);
}
fn main() {
    let s = "안녕하세요";

    let ch = &s[0..3];
    println!("{}", ch);

    let ch = &s[6..9];
    println!("{}", ch);
}
fn main() {
    let s = "안녕하세요";

    let ch = &s[0..3];
    println!("{}", ch);

    let ch = &s[6..9];
    println!("{}", ch);

    let ch = &s[9..];
    println!("{}", ch);

    let s2 = "abcde";
    println!("{}", &s2[..1]);
    println!("{}", &s2[2..3]);
    println!("{}", &s2[3..5]);
}
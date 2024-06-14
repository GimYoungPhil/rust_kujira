fn main() {
    let ss: &str = "호랑이도 제 말하면 온다.";

    let s1: String = String::from(ss);
    let s2: String = ss.to_string();

    let ss2: &str = &s1;
    let ss3: &str = s1.as_str();

    println!("{}\n{}\n{}\n{}", s1, s2, ss2, ss3);

    println!("{:p}\n{:p}\n{:p}", ss, ss2, ss3);
}
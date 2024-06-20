fn main() {
    let pr = "지혜는 무기보다 가치가 있다.";

    let sub1: String = pr.chars().take(2).collect();
    println!("{:?}", sub1);

    let pr_chars: Vec<char> = pr.chars().collect();
    let sub_chars: &[char] = &pr_chars[4..=5];
    let sub: String = sub_chars.into_iter().collect();
    println!("{:?}", sub);

}
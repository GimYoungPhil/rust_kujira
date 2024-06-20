fn main() {
    let pr = "😀🐶🍏😍😂";

    let mut sub1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 { sub1.push(c); continue; }
        break;
    }
    println!("sub1: {}", sub1);

    let mut sub2 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i >= 3 && i <= 4 { sub2.push(c); }
    }
    println!("sub2: {}", sub2);

    for (i, c) in pr.chars().enumerate() {
        println!("{}: {}", i, c);
    }
}

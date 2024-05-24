fn main () {
    println!("sum: {}", sum(10));
}

fn sum(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else {
        return sum(n - 1) + n;
    }
}
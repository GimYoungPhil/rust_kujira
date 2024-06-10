fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("a = {}", sum_slice(&a));

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("v = {}", sum_slice(&v));
}

fn sum_slice(slice: &[i64]) -> i64 {
    let mut sum = 0;

    for i in slice {
        sum += i;
    }
    sum
}
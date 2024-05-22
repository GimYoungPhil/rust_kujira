// fn vec_mecro() -> () {
//     let nums = vec![1, 2, 3];
//     println!("{:?}", nums);
// }

// fn vec_no_mecro() -> () {
//     let mut nums = Vec::new();
//     nums.push(1);
//     nums.push(2);
//     nums.push(3);
//     println!("{:?}", nums);
// }

fn test_1() -> () {
    let v = vec![1, 2, 3, 4];
    let a = [1, 2, 3, 4];

    let rv = &v;
    let ra = &a;
}

fn test_2() -> () {
    let v = vec![1, 2, 3, 4];
    let a = [1, 2, 3, 4];

    let rv: &[i32] = &v;
    let ra: &[i32] = &a;
}

fn test_string_0() {
    let s = String::from("Hello World");
    let rs = s;
}

fn test_string_1() {
    let s = String::from("Hello World");
    let rs = &s;
}

fn test_string_2() {
    let s = String::from("Hello World");
    let rs: &str = &s;
}

fn test_str_0() {
    let s = "Good morning";
    let rs = s;
}

fn test_str_1() {
    let s = "Good morning";
    let rs = &s;
}
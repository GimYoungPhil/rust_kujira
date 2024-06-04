fn main() {
    let b;
    {
        let a = String::from("hello");
        println!("{}", a);
        b = a;
        // println!("{}", a);
    }
    println!("{}", b);
}
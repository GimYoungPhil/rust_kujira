fn main() {
    let s: String = String::from("hello");

    println!("{}", s);

    let rs: &String = &s;

    println!("{}", rs);


    let mut ms: String = String::from("world");
    ms.push('!');
    println!("{}", ms);

    let mrs: &mut String = &mut ms;
    mrs.push('@');
    println!("{}", mrs);


    println!("{}", return_str());

}

fn return_str() -> &'static str {
    let result = "good morning~";
    result
}
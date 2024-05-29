fn main() {
    {
        let s1 = String::from("hot");
        let s3 = String::from("cool");
        {
            let s2 = s1;
            println!("{}", s2);
        }
        println!("{}", s3);
    }
}
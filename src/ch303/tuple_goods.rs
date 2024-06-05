fn main() {
    let banana = ("바나나", 1500);
    let apple = ("사과", 1000);
    let total = banana.1 + apple.1;

    print_goods(&banana);
    print_goods(&apple);
    println!("합계는 {}원입니다.", total);
}

fn print_goods(good: &(&str, u32)) {
    println!("{}를 {}원에 구입.", good.0, good.1);
}

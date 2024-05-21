fn main() {

    let numbers_zero: [u8; 10] = [0; 10];
    let numbers_ten: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];



    for item in numbers_ten.iter() {
        let x: &mut u8 = item;
        
        println!("{}", x);
        // numbers_zero[i] = i;
    }

    println!("{:?}", numbers_zero);
    println!("{:?}", numbers_ten);
}


const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";

fn main() {

    println!("{}", V_DATA);

    let mut count_a = 0;
    let mut count_b = 0;
    let mut count_c = 0;

    for ch in V_DATA.chars() {
        match ch {
            'A' => count_a += 1,
            'B' => count_b += 1,
            'C' => count_c += 1,
            _ => (),
        }
    }

    println!("a: {}, b: {}, c: {}", count_a, count_b, count_c);
}
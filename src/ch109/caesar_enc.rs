

fn encrypt(sss: &str, num: u8) -> u8 {
    let code_a = 'A' as u8;
    let code_z = 'Z' as u8;

    let rekk: [char; 11];

    for i in sss.bytes() {
        if i >= code_a && i <= code_z {
            // println!("{} => {}", i, (i + 3) as char);
            rekk.push((i + 3) as char);
        } else {
            // println!("{}", i as char);
            rekk.push(i as char);
        }

    }

    u8::MIN
}

fn main() {

    let rust: &str = "I LOVE RUST.";
    let enc = encrypt(rust, 3);
    // let dec = encrypt(enc, -3);

    println!("{} => {}", enc, 12);
}


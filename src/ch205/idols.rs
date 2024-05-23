use std::collections::HashMap;

fn main() {
    let idols = ["권은비" , "미야와키 사쿠라", "강혜원", "최예나", "이채연", "김채원", "김민주", "야부키 나코", "혼다 히토미", "조유리", "안유진", "장원영"];

    let mut idols_map: HashMap<&str, usize> = HashMap::new();

    for (i, v) in idols.iter().enumerate() {
        idols_map.insert(v, i + 1);
    }

    println!("{:?}", idols_map);

    let idols_jp = ["미야와키 사쿠라", "야부키 나코", "혼다 히토미", "츠바사 마이", "카에데 카렌", "카와키타 사이카"];
    
    for idol in idols_jp {
        match idols_map.get(idol) {
            Some(v) => println!("{}: {}", v, idol),
            None => println!("None: {}", idol),
        }
    }
}
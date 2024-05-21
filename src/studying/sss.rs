fn main() {

    let v: Vec<f32> = vec![0.3, 0.5, 0.7, 0.9];
    let a: [f32; 4] = [-0.3, -0.5, -0.7, -0.9];
    let rv: &[f32] = &v;
    let ra: &[f32] = &a;

    println!("{:?}", v);
    println!("{:?}", a);
    println!("{:?}", rv);
    println!("{:?}", ra);
}
fn main() {
    // let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];
    // let third: &i32 = &v[2];
    // v.push(4);

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // println!("Third element is {}", third);
}

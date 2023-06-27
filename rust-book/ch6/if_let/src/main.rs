fn main() {
    let v = Some(7);

    if let Some(3) = v {
        println!("Three!");
    } else {
        println!("Others!");
    }
}

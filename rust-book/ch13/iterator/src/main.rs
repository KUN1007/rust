fn main() {
    let v1 = vec![1, 2, 4];
    let iter = v1.iter();

    for i in iter {
        println!("Got {} ", i);
    }
}

fn main() {
    let s1 = String::from("hello");
    let mut s2 = String::from("world");
    let s3 = s1 + &s2;
    s2.clear();
    println!("s2:{:?}", s2); // 输出 s2:
    println!("s3:{}", s3); // 输出 s3:helloworld
}

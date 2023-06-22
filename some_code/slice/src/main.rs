fn main() {
    let s = String::from("KUN IS CUTEST!");

    let index: &str = find_blank(&s);

    println!("{}", index);

    let str1 = &s[..=2];
    // let str2 = &s[3..];

    let whole = &s[..];
    println!("{}, {}", str1, whole);
}

fn find_blank(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

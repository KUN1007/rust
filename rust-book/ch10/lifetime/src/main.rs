fn main() {
    let s1 = String::from("KUN");
    let s2 = "CUTEST!";

    let result = longest(s1.as_str(), s2);

    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

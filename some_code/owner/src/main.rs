fn main() {
    // let s = String::from("KUN");

    // let mut s1 = s;
    // s1.push_str(" IS CUTEST!");

    // println!("{}", s1);

    let s = String::from("KUN");

    take_ownership(s);

    let x: i32 = 17;

    make_copy(x);

    // println!("{}", s);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_number: i32) {
    println!("{}", some_number);
}

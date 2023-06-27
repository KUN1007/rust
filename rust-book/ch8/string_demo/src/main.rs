fn main() {
    // let s = "KUN IS CUTEST";

    // let ss = s.to_string();

    // let sss = s + "!";

    // println!("{}", s);
    /*
    let s1 = String::from("KUN");

    let s2 = String::from("IS");

    let s3 = String::from("CUTEST!");

    let ss = format!("{}-{}-{}", s1, s2, s3);
    let s = s1 + "~" + &s2 + "~" + &s3;

    println!("{}", ss);

    println!("{}", s); */

    // println!("{}", ss);

    let s = "啊这可海星";

    let ss = &s[..6];
    // let sss = &s[..8];

    for i in s.chars() {
        println!("{}", i);
    }

    println!("{}", ss);
}

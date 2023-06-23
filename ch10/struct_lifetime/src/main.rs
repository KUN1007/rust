struct Important<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("What color is your attribute. I think it's lightblue.");

    let first_sentence = novel.split('.').next().expect("Could not found a '.'");

    let i = Important {
        part: first_sentence,
    };
}

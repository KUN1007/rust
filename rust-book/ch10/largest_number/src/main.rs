fn main() {
    let number_list = vec![134, 24, 476, 22, 78];
    let la = largest(&number_list);

    println!("The largest number is {}", la);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

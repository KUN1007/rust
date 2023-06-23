fn main() {
    let number_list = vec![134, 24, 476, 22, 78];
    let la = largest(&number_list);

    println!("The largest number is {}", la);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

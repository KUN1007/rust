fn main() {
    let tup: (i32, f64, u8) = (500, 1.1, 2);

    println!("{}, {}, {}, ", tup.0, tup.1, tup.2);

    let arr: [i32; 5] = [1; 5];

    println!("{}", arr[1]);
}

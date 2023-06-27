fn add(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let ans = do_twice(add, 7);
    println!("ans is: {}", ans);
}

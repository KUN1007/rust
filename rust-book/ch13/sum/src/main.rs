fn main() {
    let v = vec![1, 2, 3];

    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();

    let mut res = 0;

    for i in v2 {
        res += i;
    }

    println!("{}", res);
}

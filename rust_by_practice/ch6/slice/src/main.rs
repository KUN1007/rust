// 修复代码中的错误，不要新增代码行!
/* fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr;

    let s2: &str = "hello, world";
}
 */

/* fn main() {
    let arr: [char; 4] = ['鲲', '最', '可', '爱'];

    let slice = &arr[..2];

    // 修改数字 `8` 让代码工作
    assert!(std::mem::size_of_val(&slice) == 16);
} */

/* fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // 填空让代码工作起来
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
} */

/* fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // 填空，不要再使用 0..2
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);
}
 */

/* fn main() {
    let s = "你好，世界";
    // 修改以下代码行，让代码工作起来
    let slice = &s[0..=2];

    assert!(slice == "你");
} */

// 修复所有错误
fn main() {
    let mut s = String::from("hello world");

    // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    let ch = first_character(&s);

    s.clear(); // error!

    // println!("the first character is: {}", ch);
}
fn first_character(s: &str) -> &str {
    &s[..1]
}

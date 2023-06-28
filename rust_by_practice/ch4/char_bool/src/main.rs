// 修改2处 `assert_eq!` 让代码工作

/* use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '鲲';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!")
}
 */

// 修改一行让代码正常打印
/* fn main() {
    let c1 = '鲲';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
 */

// 使成功打印
/* fn main() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!")
    }
}
 */

/* fn main() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!")
}
 */

// 让代码工作，但不要修改 `implicitly_ret_unit` !
/* fn main() {
    let _v: () = ();

    let v = ();
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}
 */

// 让代码工作：修改 `assert!` 中的 `4`
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}

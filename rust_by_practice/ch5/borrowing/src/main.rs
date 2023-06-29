/* fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x;
    println!("{}", y);
}
 */

/* fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}", x, y);
}
 */

/* fn main() {
    // 使用尽可能多的方法来通过编译
    let x = "hello, world";
    let y = x;
    println!("{},{}", x, y);
}
 */

/* fn main() {
    // 使用尽可能多的方法来通过编译
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}", x, y);
} */

/*
fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = x.as_str();
    let y = &x;
    println!("{},{}", x, y);
} */

/* // 不要修改 main 中的代码
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    s
} */

/* fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.as_bytes();
    s
} */

/* fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    s
} */

// 修复错误，不要删除任何代码行
/* fn main() {
    let s = String::from("hello, world");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &str) {
    println!("{}", s);
} */

// 修复错误，不要删除任何代码行
/* fn main() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}", s)
}
 */

// 不要使用 clone，使用 copy 的方式替代
/* fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
 */

/* fn main() {
    let s = String::from("hello, ");

    // modify this line only !
    let mut s1 = s;

    s1.push_str("world")
} */

/* fn main() {
    let x = Box::new(5);

    let mut y = Box::new(3); // 完成该行代码，不要修改其它行！

    *y = 4;

    assert_eq!(*x, 5);
}
 */

/* fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
} */

/* fn main() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // 仅修改下面这行代码，且不要使用 `_s`
    println!("{:?}", t.1);
} */

fn main() {
    let t = (String::from("hello"), String::from("world"));

    // 填空，不要修改其它代码
    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

/* fn main() {
    let t = (String::from("hello"), String::from("world"));

    // 填空，不要修改其它代码
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
} */

fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} - {}", value, index);
    }
}

// match 的最后一句是 irrefutable 的，其它子句是 refutable 的
// let 和 while let 只支持 irrefutable 的模式
// match 中的 _, ..=, .., | 都有自己的用法

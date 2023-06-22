fn lift_off() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!");
}

fn main() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    for element in arr.iter() {
        println!("The value is {}", element);
    }

    lift_off();

    println!("The result is {result}")
}

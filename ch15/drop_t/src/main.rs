struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Dropping with data {}", self.data);
    }
}

fn main() {
    let c = CustomerSmartPointer {
        data: String::from("KUN"),
    };
    drop(c);
    let d = CustomerSmartPointer {
        data: String::from("YUYU"),
    };
    println!("azkhx");
}

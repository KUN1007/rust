mod kun {
    pub struct KUN {
        pub name: String,
        age: String,
    }

    impl KUN {
        pub fn moe(name: &str) -> KUN {
            KUN {
                name: String::from(name),
                age: String::from("16"),
            }
        }
    }
}

pub fn kun_moe() {
    let mut name = kun::KUN::moe("KUN");
    name.name = String::from("azkhx");
    println!("Say {}", name.name);
}

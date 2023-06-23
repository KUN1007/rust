use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("kun.txt");

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("kun.txt") {
                Ok(file) => file,
                Err(error) => panic!("ERROR while create file kun.txt {}", error),
            },
            err => panic!("ERROR while opening files kun.txt {}", err),
        },
    };

    File::open("azkhx.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("azkhx.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });
}

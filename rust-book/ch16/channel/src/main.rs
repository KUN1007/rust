use core::time::Duration;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = vec![
            String::from("KUN 1"),
            String::from("moemoe 1"),
            String::from("Cute 1"),
        ];

        for v in val {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let val = vec![
            String::from("KUN"),
            String::from("moemoe"),
            String::from("Cute"),
        ];

        for v in val {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got {}", received);
    }
}

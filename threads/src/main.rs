use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod basic_thread_example {
    use std::thread::{self};
    use std::time::Duration;

    pub fn spawning_thread() {
        // Spawning a thread
        thread::spawn(|| {
            for i in 1..10 {
                println!("Hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
    }
}

fn main() {
    // Simple thread spawn
    // basic_thread_example::spawning_thread();
    //
    // for i in 1..5 {
    //     println!("Hi number {} from the MAIN thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // Adding join handler to warranty full thread execution
    // main thread finish, but handle thread keep running until finish

    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // If we want that handle finish first, then execute main thread.
    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("Hi number {} from the MAIN thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();

    // Multiples productores un solo consumidor mpsc
    // Compartir datos entre hilos correctamente usando canales.

    let (tx, rx) = mpsc::channel();

    // Basic usage
    // thread::spawn(move || {
    //     let val = String::from("Hi thread");
    //     tx.send(val).unwrap();
    // });
    //
    // let received = rx.recv().unwrap();
    // println!("Got from channel: {}", received);

    // Recibiendo multiples mensajes

    thread::spawn(move || {
        let msgs = vec![
            String::from("Hi"),
            String::from("Im msg 1"),
            String::from("Im msg 2"),
            String::from("wait im msg 3"),
            String::from("holly crab im msg 4"),
        ];

        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

use std::thread;
use std::time::Duration;

mod basic_thread_example {
    use std::thread;
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
    basic_thread_example::spawning_thread();

    for i in 1..5 {
        println!("Hi number {} from the MAIN thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

use std::thread;
fn main() {
    let mut threads = Vec::new();

    for i in 0..2 {
        let thread = thread::spawn(move || {
            println!(
                "Hello, world - из потока номер {} его ID {:?}",
                i,
                thread::current().id()
            );
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}

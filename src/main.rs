use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let thread = thread::current();
    println!("{:?}: main thread start", thread.id());
    let (tx, rx) = channel();

    let sender = thread::spawn(move || {
        let thread = thread::current();
        let data = "Hello, thread!";
        println!("{:?}: send \"{}\"", thread.id(), data);
        tx.send(data).expect("Unable to send on channel");
    });

    let receiver = thread::spawn(move || {
        let thread = thread::current();
        let data = rx.recv().expect("Unable to receive from channel");
        println!("{:?}: reveive \"{}\"", thread.id(), data);
    });

    sender.join().expect("The sender thread has panicked");
    receiver.join().expect("The receiver thread has panicked");
    println!("{:?}: main thread end", thread.id());
}

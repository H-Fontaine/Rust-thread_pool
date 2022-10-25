extern crate core;

use threadPool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let thread_poll = ThreadPool::new(18);

    let (sender, receiver) = channel();
    for i in 0..40 {
        thread_poll.add_task().send((Box::new(move || {
            let mut _f = 0;
            for _ in 0..1000 {
                _f+= 1;
            };
            Box::new(i)
        }), sender.clone())).unwrap();
    }

    drop(sender);

    for res in receiver {
        match res.downcast_ref::<i32>() {
            Some(b) => println!("{}", b),
            None => println!("Error"),
        }
    }
}
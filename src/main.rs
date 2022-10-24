extern crate core;

use threadPool::ThreadPool;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::any::Any;

/*
fn main() {
    let nb_of_threads = usize::from(thread::available_parallelism().unwrap()) - 1;
    println!("Nombre de thread physique : {}", nb_of_threads);

    let (results_sender, results_receiver) = channel();
    let mut tasks_senders = Vec::<Sender<Box<dyn Fn() -> Box<dyn Send + Display> + Send>>>::new();
    let nb_times = 10;

    for i in 0..nb_of_threads {
        let results_sender_clone = results_sender.clone();
        let (tasks_sender, tasks_receiver) = channel::<Box<dyn Fn() -> Box<dyn Send + Display> + Send>>();
        tasks_senders.push(tasks_sender);
        thread::spawn(move || {
            for task in tasks_receiver {
                results_sender_clone.send(task()).unwrap();
            }
        });
        for j in 0..nb_times {
            tasks_senders.last().unwrap().send(Box::new(move || {
                let mut f = 0;
                for _ in 0..100000000 {
                    f += 1;
                }
                Box::new(j + i * nb_times)})).unwrap();
        }
    }

    for sender in tasks_senders {
        drop(sender);
    }
    drop(results_sender);
    println!("arrived");

    for result in results_receiver {
        println!("{}", result)
    }
}
 */


fn main() {
    let mut threadPoll = ThreadPool::new(18);

    let function = Box::new(move || println!("Hello world"));
    let (sender, receiver) = channel();
    threadPoll.add_task().send((Box::new(|| Box::new(1 + 1)), sender)).unwrap();

    let res = match receiver.recv().unwrap().downcast::<Box<usize>>() {
        Ok(b) => b,
        Err(_) => panic!(),
    };

    println!("{}", res);
}
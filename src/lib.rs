use std::iter::zip;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use worker::Worker;
use std::any::Any;

mod worker;

type Task = Box<dyn Fn() -> Box<dyn Send + Any> + Send>;
type Returned = Box<dyn Send + Any>;

pub struct ThreadPool {
    nb_thread : usize,
    workers : Vec<Worker>,
    to_receive_task : Sender<(Task, Sender<Returned>)>,
    queen : thread::JoinHandle<()>,
}

impl ThreadPool {
    pub fn new(nb_thread : usize) -> ThreadPool {
        let (to_receive_task, received_tasks) = channel::<(Task, Sender<Returned>)>();

        let mut workers = Vec::with_capacity(nb_thread);
        let mut senders = Vec::with_capacity(nb_thread);
        let (id_sender, ids_receiver) = channel();
        for i in 0..nb_thread {
            let id_sender_clone = id_sender.clone();
            let (sender, receiver) = channel();
            senders.push(sender);
            workers.push(Worker::new(i, receiver,id_sender_clone));
        }

        let queen = thread::spawn(move || {
            for (thread_index, task_and_sender) in zip(ids_receiver, received_tasks) {
                senders[thread_index].send(task_and_sender).unwrap();
            }
        });

        ThreadPool {
            nb_thread,
            workers,
            to_receive_task,
            queen,
        }
    }

    pub fn add_task(&self) -> &Sender<(Task, Sender<Returned>)>  {
        &self.to_receive_task
    }

    pub fn end(self) {
        drop(self.to_receive_task);
        self.queen.join().unwrap();
        for worker in self.workers {
            worker.end();
        }
    }
}
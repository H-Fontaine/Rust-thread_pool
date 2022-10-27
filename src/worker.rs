use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::{Returned, Task};

pub struct Worker {
    thread : thread::JoinHandle<()>,
    id : usize,
}

impl Worker {
    pub fn new(id : usize, tasks : Receiver<(Task, Sender<Returned>)>, id_sender : Sender<usize>) -> Worker {
        let thread = thread::spawn(move || {
            id_sender.send(id).unwrap();
            for (task, send_to) in tasks {
                send_to.send(task()).unwrap();
                id_sender.send(id).unwrap();
            }
        });
        Worker {
            thread,
            id,
        }
    }

    pub fn end(self) {
        self.thread.join().unwrap();
    }

}
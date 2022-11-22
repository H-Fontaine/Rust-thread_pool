use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::{Runnable, Task};


/*
This structure contains :
 - id : usize                               The id of the worker to be send to the thread pool to know id the thread is waiting for tasks or not
 - thread : thread::JoinHandle<()>          The JoinHandle of the thread of the worker
*/
pub struct Worker {
    id : usize,
    thread : thread::JoinHandle<()>
}

impl Worker {
    /*
    Constructor of the Worker :
     - id : usize                                   This is the id of the worker
     - im_ready : Sender<usize>                     This sender is to specify when the thread is ready to get a new task, when so its sends its id to the queen thread
     - tasks_receiver : Receiver<Runnable<T>>       This Receiver is to receive the tasks send by the queen thread and
    */
    pub fn new<T : Send + 'static>(id : usize, tasks_receiver : Receiver<Task<T>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move || {
                for (runnable, result_sender) in tasks_receiver {            //While task_receiver exist we wait for tasks
                    result_sender.send(runnable()).unwrap();                                    //Sending runnable result
                }
            }),
        }
    }

    //Finishing the thread
    pub fn join(self) {
        self.thread.join().unwrap();
    }
}
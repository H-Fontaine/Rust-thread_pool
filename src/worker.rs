use std::sync::mpsc::Receiver;
use std::thread;
use crate::Task;


/*
This structure contains :
 - thread : thread::JoinHandle<()>          The JoinHandle of the thread of the worker
*/
pub struct Worker {
    thread : thread::JoinHandle<()>
}

impl Worker {
    /*
    /*
    Constructor of the Worker :
     - im_ready : Sender<usize>                     This sender is to specify when the thread is ready to get a new task, when so its sends its id to the queen thread
     - tasks_receiver : Receiver<Runnable<T>>       This Receiver is to receive the tasks send by the queen thread and
    */
    pub fn new<T : Send + 'static>(tasks_receiver : Receiver<Task<T>>) -> Worker {
        Worker {
            thread: thread::spawn(move || {
                for (runnable, result_sender) in tasks_receiver {            //While task_receiver exist we wait for tasks
                    result_sender.send(runnable()).unwrap();                                            //Sending runnable result
                }
            }),
        }
    }
    */

    /*
    Constructor of the Worker :
     - im_ready : Sender<usize>                     This sender is to specify when the thread is ready to get a new task, when so its sends its id to the queen thread
     - tasks_receiver : Receiver<Runnable<T>>       This Receiver is to receive the tasks send by the queen thread and
    */
    pub fn new<T : Send + 'static>(tasks_receiver : Receiver<Task<T>>) -> Worker {
        Worker {
            thread: thread::spawn(move || {
                for (runnable, result_sender) in tasks_receiver {            //While task_receiver exist we wait for tasks
                    match result_sender {
                        Some(sender) => sender.send(runnable()).unwrap(),   //Sending runnable result
                        None => {runnable();},                                          //If there is no sender we assume there is no return for the function
                    }
                }
            }),
        }
    }

    //Finishing the thread
    pub fn join(self) {
        self.thread.join().unwrap();
    }
}
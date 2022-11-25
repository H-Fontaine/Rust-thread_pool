use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::available_parallelism;
use crate::worker::Worker;

mod worker;

type Runnable<T> = Box<dyn Fn() -> T + Send>; //Runnable is an undefined function that returns a type T
type Task<T> = (Runnable<T>, Sender<T>);


/*
This structure contains :
 - workers : Vec<Worker>                                    The workers that will execute the Runnable
 - queen : thread::JoinHandle<()>                           This is the join handle of the queen thread that will dispatch all the Runnable to the workers
 - tasks_waiting_queue_sender : Sender<task>>               This is the sender that will serve to send all the tasks the the queen thread
 */
pub struct ThreadPool<T : Send> {
    workers : Vec<Worker>,
    queen : thread::JoinHandle<()>,
    tasks_waiting_queue_sender : Sender<Task<T>>,
}


impl<T : Send + 'static> ThreadPool<T> {
    /*
    Constructor of ThreadPool :
    - mut number_of_thread : usize          This is the number of worker that will be created added to the queen thread
    */
    pub fn new(mut number_of_thread : usize) -> ThreadPool<T> {
        //Deciding the number of workers to create
        number_of_thread = {
            let available_threads = usize::from(available_parallelism().unwrap());
            if available_threads < number_of_thread {
                available_threads
            } else {
                number_of_thread
            }
        };

        //Creating workers
        let mut tasks_senders = Vec::with_capacity(number_of_thread);        //Creating the vector that will carry the sender witch's will be use to send tasks to the workers
        let mut workers = Vec::with_capacity(number_of_thread);
        for _ in 0..number_of_thread {
            let (tasks_sender, tasks_receiver) = channel();               //Create a channel per worker to send them the tasks
            tasks_senders.push(tasks_sender);                                                      //Saving senders
            workers.push(Worker::new(tasks_receiver));                                       // Creating workers
        }

        //Creating the queen threed
        let (tasks_waiting_queue_sender, tasks_waiting_queue_receiver) = channel(); //Channel which will send tasks to the queen thread

        let queen_closure = move || {
            let mut id = 0;
            for task in tasks_waiting_queue_receiver {
                tasks_senders[id].send(task).unwrap();
                id = (id + 1) % number_of_thread;                       //Choice of worker is simple, it just roll from 0 to number_of_thread - 1
            }
            drop(tasks_senders); //Dropping tasks_senders to stop workers
        };

        let queen = thread::spawn(queen_closure);

        ThreadPool {
            workers,
            queen,
            tasks_waiting_queue_sender,
        }
    }

    /*
    Method to give a task to the thread pool returning an Option<Receiver<T>> depending on opt_result_sender :
     - runnable : Runnable<T>                           The boxed closure to send has a task
     - opt_result_sender : Option<Sender<T>>            This is made to provide or not a Sender, depends if the user already has a channel or not
    */
    pub fn add_task(&self, runnable : Runnable<T>, opt_result_sender : Option<Sender<T>>) -> Option<Receiver<T>> {
        match opt_result_sender {
            Some(result_sender) => {
                self.tasks_waiting_queue_sender.send((runnable, result_sender)).unwrap();            //Sending the task to the queen
                None                                                                                    //Returning nothing because the user already has the receiver
            },
            None => {
                let (result_sender, result_receiver) = channel();                  //Creating a channel to return the result
                self.tasks_waiting_queue_sender.send((runnable, result_sender)).unwrap();            //Sending the task
                Some(result_receiver)                                                                   //Returning the receiver because the user doesn't have it
            }
        }
    }

    //To terminate the ThreadPool
    pub fn join(self) {
        drop(self.tasks_waiting_queue_sender);
        for worker in self.workers {
            worker.join();
        }
        self.queen.join().unwrap();
    }
}
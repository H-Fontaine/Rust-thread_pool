extern crate core;

use std::sync::Arc;
use threadPool::ThreadPool;
use std::sync::mpsc::channel;
use matrix::Matrix;


use std::time::Instant;
fn main() {
    let size = 20;

    let now = Instant::now();
    // Code block to measure.
    {
        let thread_pool = ThreadPool::new(19);
        let matrix_arc1 = Arc::new(Matrix::<f32>::ones(size, size));
        let matrix_arc2 = Arc::new(Matrix::<f32>::ones(size, size));
        let mut res = Matrix::<f32>::ones(matrix_arc1.lines(), matrix_arc1.columns());
        let (sender, receiver) = channel();

        let lines = matrix_arc1.lines();
        for i in 0..lines {
            for j in 0..lines {
                let matrix_arc1_cloned = matrix_arc1.clone();
                let matrix_arc2_cloned = matrix_arc2.clone();
                let runnable = move || {
                    let mut res = 0f32;
                    for k in 0..lines {
                        res += matrix_arc1_cloned[i][k] * matrix_arc2_cloned[j][k];
                    }
                    (res, i, j)
                };
                thread_pool.add_task(Box::new(runnable), Some(sender.clone()));
            }
        }

        drop(sender);

        for result in receiver {
            res[result.1][result.2] = result.0
        }
        res.display();
        thread_pool.join();
    }
    let elapsed = now.elapsed();
    println!("Elapsed for multi threaded: {:.2?}", elapsed);

    let now = Instant::now();

    // Code block to measure.
    {
        let matrix1 = Matrix::<f32>::ones(size, size);
        let matrix2 = Matrix::<f32>::ones(size, size);
        let res = matrix1 * matrix2;
        //res.display();
    }

    let elapsed = now.elapsed();
    println!("Elapsed for single threaded: {:.2?}", elapsed);

}
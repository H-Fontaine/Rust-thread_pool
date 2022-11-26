extern crate core;

use std::sync::{Arc, Mutex};
use thread_pool::ThreadPool;
use std::sync::mpsc::channel;
use matrix::Matrix;
use std::time::Instant;


fn main() {
    let size = 2000;

    let now = Instant::now();
    // Code block to measure.
    {
        let thread_pool = ThreadPool::new(19);
        let matrix_arc1 = Arc::new(Matrix::<f32>::ones(size, size));
        let matrix_arc2 = Arc::new(Matrix::<f32>::ones(size, size));
        let matrix_result = Arc::new(Mutex::new(Matrix::<f32>::ones(matrix_arc1.lines(), matrix_arc1.columns())));

        let lines = matrix_arc1.lines();
        for i in 0..lines {
            for j in 0..lines {
                let matrix_arc1_cloned = matrix_arc1.clone();
                let matrix_arc2_cloned = matrix_arc2.clone();
                let matrix_result_cloned = matrix_result.clone();
                let runnable = move || {
                    let mut res = 0f32;
                    for k in 0..lines {
                        res += matrix_arc1_cloned[i][k] * matrix_arc2_cloned[j][k];
                    }
                    matrix_result_cloned.lock().unwrap()[i][j] = res;
                };
                thread_pool.add_task(Box::new(runnable), None);
            }
        }
        thread_pool.join();
        //matrix_result.lock().unwrap().display();
    }
    let elapsed = now.elapsed();
    println!("Elapsed for multi threaded: {:.2?}", elapsed);



    let now = Instant::now();
    // Code block to measure.
    {
        let matrix1 = Matrix::<f32>::ones(size, size);
        let matrix2 = Matrix::<f32>::ones(size, size);
        let _res = matrix1 * matrix2;
        //res.display();
    }
    let elapsed = now.elapsed();
    println!("Elapsed for single threaded: {:.2?}", elapsed);
}
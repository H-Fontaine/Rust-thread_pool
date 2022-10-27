extern crate core;

use std::sync::Arc;
use threadPool::ThreadPool;
use std::sync::mpsc::channel;
use matrix::Matrix;

/*
fn multiply(matrix1 : Matrix<f32>, matrix2 : Matrix<f32>, thread_pool : &ThreadPool) -> Matrix<f32> {
    let mut res = Matrix::<f32>::ones(matrix1.lines(), matrix2.columns());
    let (sender, receiver) = channel();
    for i in 0..matrix1.lines() {
        let test = matrix2.lines();
        for j in 0..matrix2.columns() {
            thread_pool.add_task().send((Box::new(|| {
                let mut res = 0f32;
                for k in 0..test {
                    res += matrix1[i][k] + matrix2[k][j];
                }
                Box::new((res, i, j))
            }), sender.clone())).unwrap();
        }
    }

    drop(sender);

    for result in receiver {
        match result.downcast_ref::<(f32, usize, usize)>() {
            Some((calcul, i, j)) => {res[*i][*j] = *calcul}
            None => println!("Error")
        }
    }
    res
}
*/

use std::time::Instant;
fn main() {
    let size = 2000;

    let now = Instant::now();
    // Code block to measure.
    {
        let thread_pool = ThreadPool::new(20);
        let matrix_arc1 = Arc::new(Matrix::<f32>::ones(size, size));
        let matrix_arc2 = Arc::new(Matrix::<f32>::ones(size, size));
        let mut res = Matrix::<f32>::ones(matrix_arc1.lines(), matrix_arc1.columns());
        let (sender, receiver) = channel();

        for i in 0..matrix_arc1.lines() {
            for j in 0..matrix_arc1.lines() {
                let matrix1 = matrix_arc1.clone();
                let matrix2 = matrix_arc2.clone();
                thread_pool.add_task().send((Box::new(move || {
                    let mut res = 0f32;
                    for k in 0..matrix1.lines() {
                        res += matrix1[i][k] * matrix2[k][j];
                    }
                    Box::new((res, i, j))
                }), sender.clone())).unwrap();
            }
        }

        drop(sender);

        for result in receiver {
            match result.downcast_ref::<(f32, usize, usize)>() {
                Some((calc, i, j)) => { res[*i][*j] = *calc }
                None => println!("Error")
            }
        }

        thread_pool.end();
    }
    let elapsed = now.elapsed();
    println!("Elapsed for multi threaded: {:.2?}", elapsed);

    let now = Instant::now();

    // Code block to measure.
    {
        let matrix1 = Matrix::<f32>::ones(size, size);
        let matrix2 = Matrix::<f32>::ones(size, size);
        let res = matrix1 * matrix2;
    }

    let elapsed = now.elapsed();
    println!("Elapsed for single threaded: {:.2?}", elapsed);

}
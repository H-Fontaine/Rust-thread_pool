extern crate core;

use std::time::Instant;
fn main() {

    // Code block to measure.
    let now = Instant::now();
    {

    }
    let elapsed = now.elapsed();
    println!("Elapsed for multi threaded: {:.2?}", elapsed);


    // Code block to measure.
    let now = Instant::now();
    {

    }
    let elapsed = now.elapsed();
    println!("Elapsed for single threaded: {:.2?}", elapsed);

}
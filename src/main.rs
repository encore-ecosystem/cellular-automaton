use std::sync::{Arc, Mutex};
use std::thread;
mod field;
mod rules;
mod consumer;
mod producer;

use std::collections::VecDeque;


fn main() {
    let num_of_consumers = 2;
    let width            = 64;
    let height           = 64;
    let iters            = 16;

    let shared_buffer = Arc::new(Mutex::new(VecDeque::new()));

    let input_buffer = Arc::clone(&shared_buffer);
    let mut prod = producer::Producer::new(input_buffer);

    
    let mut consumers_threads = vec![];
    for id in 0..num_of_consumers
    {
        let output_buffer = Arc::clone(&shared_buffer);
        let mut consumer = consumer::Consumer::new(output_buffer, id + 1);
        let thread = thread::spawn(move || {consumer.run(height, width, iters)});
        
        consumers_threads.push(thread);
    }
    prod.run();
}
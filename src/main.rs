// use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use spmc;

mod field;
mod rules;
mod consumer;
mod producer;

fn main() {
    let num_of_consumers = 2;
    let width            = 64;
    let height           = 64;
    let iters         = 16;

    let (sender, reciever) = spmc::channel();
    
    let mut consumers = vec![];
    for i in 0..num_of_consumers
    {
        let receiver = reciever.clone();
        consumers.push(thread::spawn(move || {
            let mut consumer = consumer::Consumer::new(receiver, i + 1);
            consumer.run(height, width, iters)
        }));
    }

    let mut prod = producer::Producer::new(sender);
    prod.run();
}
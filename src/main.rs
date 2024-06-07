// use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
mod field;
mod rules;
use crate::rules::{Neighborhood, CellularAutomatonParams};
use crate::field::Field;

mod blockingq;
use blockingq::BlockingQueue;

mod producer;

fn main() {
    // let survive_array = vec![3, 4, 5];
    // let birth_array   = vec![1, 2, 3];
    // let width         = 16;
    // let height        = 16;

    // let rule = CellularAutomatonParams {
    //     neighborhood_type   : Neighborhood::VonNeumann,
    //     neighborhood_radius : 1,   
    //     initial_density     : 0.9_f64,
    //     survive_array       : survive_array,
    //     birth_array         : birth_array,
    // };
    // let field = Field::new(rule, width, height);

    let prod = producer::Producer::new();
    prod.run_tasks_generation(10);
}

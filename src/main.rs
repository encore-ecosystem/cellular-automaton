// use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

mod field;
mod rules;
use crate::rules::{Neighborhood, CellularAutomatonParams};
use crate::field::Field;

/*
mod is used to declare (mod module;), or define (mod module {}) a module.
use is then used to import (and optionally rename with as) a module, or its items from elsewhere.
*/

fn main() {
    let survive_array = vec![3, 4, 5];
    let birth_array   = vec![1, 2, 3];
    let width         = 8;
    let height        = 8;

    let rule = CellularAutomatonParams {
        neighborhood_type   : Neighborhood::Moore,
        neighborhood_radius : 1,   
        initial_density     : 0.1_f64,
        survive_array       : survive_array,
        birth_array         : birth_array,
    };
    let mut field = Field::new(rule, width, height);

    for _ in 0..=3 {
        for x in 0..height {
            for y in 0..width {
                print!(" {}", if field.data[x as usize][y as usize] {1} else {0});
            }
            println!();
        }
        println!();
        println!();

        field.update();
        
        thread::sleep(Duration::from_millis(1000));
    }
    
}


// trait Producer{}

// fn producer(tx: mpsc::Sender<CellularAutomatonParams>) {
//     for _ in 0..5 {
//         let rule = "Rule".to_string();
//         let neighborhood_type = "Moore".to_string();
//         let initial_density = 0.5;
//         let neighborhood_radius = 1;

//         let params = CellularAutomatonParams {
//             rule,
//             neighborhood_type,
//             initial_density,
//             neighborhood_radius,
//         };

//         tx.send(params).unwrap();
//     }
// }

// fn consumer(id: usize, rx: mpsc::Receiver<CellularAutomatonParams>) {
//     for params in rx {
//         println!("Consumer {} received parameters: {:?}", id, params);
//     }
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx_clone = tx.clone();
//     let producer_handle = thread::spawn(move  {
//         producer(tx_clone);
//     });

//     let num_consumers = 3;
//     let mut consumer_handles = vec![];
//     for i in 0..num_consumers {
//         let rx = rx.clone();
//         let handle = thread::spawn(move  {
//             consumer(i, rx);
//         });
//         consumer_handles.push(handle);
//     }

//     producer_handle.join().unwrap();
//     for handle in consumer_handles {
//         handle.join().unwrap();
//     }
// }

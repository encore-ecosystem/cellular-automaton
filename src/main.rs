use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Структура для хранения параметров клеточного автомата
struct CellularAutomatonParams {
    rule: String,
    neighborhoodtype: String,
    initial_density: f64,
    neighborhood_radius: u32,
}

fn producer(tx: mpsc::Sender<CellularAutomatonParams>) {
    for _ in 0..5 {
        // Генерируем случайные параметры для клеточного автомата
        let rule = "Rule".to_string();
        let neighborhood_type = "Moore".to_string(); // или "von Neumann"
        let initial_density = 0.5;
        let neighborhood_radius = 1;

        let params = CellularAutomatonParams {
            rule,
            neighborhood_type,
            initial_density,
            neighborhood_radius,
        };

        tx.send(params).unwrap();
    }
}

fn consumer(id: usize, rx: mpsc::Receiver<CellularAutomatonParams>) {
    for params in rx {
        println!("Consumer {} received parameters: {:?}", id, params);
        // Запуск клеточного автомата с переданными параметрами и k итерациями
        // Здесь должен быть код запуска итераций клеточного автомата
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx_clone = tx.clone();
    let producer_handle = thread::spawn(move  {
        producer(tx_clone);
    });

    let num_consumers = 3;
    let mut consumer_handles = vec![];
    for i in 0..num_consumers {
        let rx = rx.clone();
        let handle = thread::spawn(move  {
            consumer(i, rx);
        });
        consumer_handles.push(handle);
    }

    producer_handle.join().unwrap();
    for handle in consumer_handles {
        handle.join().unwrap();
    }
}

use crate::rules::CellularAutomatonParams;
use crate::field::Field;
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Consumer
{
    buffer : Arc<Mutex<VecDeque<CellularAutomatonParams>>>,
    id     : usize,
}


impl Consumer
{
    pub fn new(buffer: Arc<Mutex<VecDeque<CellularAutomatonParams>>>, id: usize) -> Self
    {
        Self {buffer, id}
    }

    pub fn run(&mut self, height: u8, width: u8, iters: usize) -> Field
    {
        loop {
            if self.buffer.lock().unwrap().is_empty()
            {
                thread::sleep(Duration::from_millis(200));
            }
            else
            {
                let rule = self.buffer
                                    .lock()
                                    .unwrap()
                                    .pop_back()
                                    .unwrap();
                let mut field = Field::new(rule.clone(), width, height);
                for _ in 0..iters {
                    field.update();
                    std::thread::sleep(Duration::from_millis(200));
                }
                println!("Consumer[{}]: with rule: {}", self.id, rule);
                println!("{}", field);
                std::thread::sleep(Duration::from_millis(200));
            }
        }
    }
}

use crate::rules::CellularAutomatonParams;
use crate::field::Field;
use std::time::Duration;
use spmc::Receiver;


pub struct Consumer
{
    pub buffer : Vec<CellularAutomatonParams>,
    receiver   : Receiver<CellularAutomatonParams>,
    name       : usize,
}

impl Consumer
{
    pub fn new(receiver: spmc::Receiver<CellularAutomatonParams>, name: usize) -> Self
    {
        Self {buffer: vec![], receiver, name}
    }

    pub fn run(&mut self, height: u8, width: u8, iters: usize) -> Field
    {
        loop {
            self.buffer.push(self.receiver.recv().unwrap());
            match self.buffer.len() {   
                0 => {
                    std::thread::sleep(Duration::from_millis(1000));
                }
                _ => {
                    let rule = self.buffer.pop().unwrap();
                    let mut field = Field::new(rule.clone(), width, height);
                    for _ in 0..iters {
                        field.update();
                        std::thread::sleep(Duration::from_millis(200));
                    }
                    println!("Consumer: {} with rule: {}", self.name, rule);
                    println!("{}", field);
                    std::thread::sleep(Duration::from_millis(200));
                }
            }
        }
    }
}

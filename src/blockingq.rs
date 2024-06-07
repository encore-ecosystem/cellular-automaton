use std::sync::{Condvar, Mutex};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct BlockingQueue<T> {
    queue_data : Mutex<VecDeque<T>>,
    cond_var   : Condvar,
}


impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            queue_data : Mutex::new(VecDeque::new()),
            cond_var   : Condvar::new(),
        }
    }

    pub fn push(&self, variable: T) {
        let mut lock = self.queue_data.lock().unwrap();
        lock.push_back(variable);
        self.cond_var.notify_one();
    }

    pub fn pop(&self) -> T {
        let mut lock = self.queue_data.lock().unwrap();
        while lock.len() == 0 {
            lock = self.cond_var.wait(lock).unwrap();
        }
        lock.pop_front().unwrap()
    }

    pub fn len(&self) -> usize {
        self.queue_data.lock().unwrap().len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_of_len() {
        let bq = BlockingQueue::<f64>::new();
        assert_eq!(bq.len(), 0);
    }

    #[test]
    fn test_push() {
        let bq = BlockingQueue::<f64>::new();
        bq.push(3.5);
        assert_eq!(bq.len(), 1);
    }
    #[test]
    fn test_pop() {
        let bq = BlockingQueue::<f64>::new();
        bq.push(3.5);
        assert_eq!(bq.pop(), 3.5);
        assert_eq!(bq.len(), 0);
    }
}
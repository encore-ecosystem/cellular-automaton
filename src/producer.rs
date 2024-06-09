use rand::{thread_rng, Rng};
use std::collections::HashSet;
use crate::rules::CellularAutomatonParams;
use crate::rules::Neighborhood;
use spmc::Sender;



pub struct Producer
{
    sender    : Sender<CellularAutomatonParams>,
}


impl Producer
{
    pub fn new(sender : Sender<CellularAutomatonParams>) -> Self
    {
        Self {sender}
    }

    pub fn generate_task(&self) -> CellularAutomatonParams
    {
        let mut rng = thread_rng();

        let neighborhood_type: Neighborhood = rand::random();
        let neighborhood_radius: u8 = rng.gen_range(1..6);
        let initial_density = rng.gen();

        let max_neighbor = neighborhood_type.get_max_neighbors(neighborhood_radius as usize);

        let mut survive_candidates: HashSet<usize> = HashSet::from_iter(1..max_neighbor);
        let mut survive_array = vec![];
        for _ in 0..rng.gen_range(0..max_neighbor) {
            let candidate_index = rng.gen_range(0..survive_candidates.len());
            let candidate = survive_candidates.iter().nth(candidate_index).unwrap().clone();
            survive_array.push(candidate);
            survive_candidates.remove(&candidate);
        }

        let mut birth_candidates: HashSet<usize> = HashSet::from_iter(1..max_neighbor);
        let mut birth_array = vec![];
        for _ in 0..rng.gen_range(0..max_neighbor) {
            let candidate_index = rng.gen_range(0..birth_candidates.len());
            let candidate = birth_candidates.iter().nth(candidate_index).unwrap().clone();
            birth_array.push(candidate);
            birth_candidates.remove(&candidate);
        }
        
        let task = CellularAutomatonParams{
            neighborhood_type,
            neighborhood_radius,
            initial_density,
            survive_array,
            birth_array,
        };

        task
    }

    pub fn run(&mut self)
    {
        loop {
            let task = self.generate_task();
            self.sender.send(task).unwrap();
        }
    }
}

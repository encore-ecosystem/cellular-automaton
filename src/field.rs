use crate::rules::{Neighborhood, CellularAutomatonParams};
use rand::{thread_rng, Rng};



#[derive(Debug)]
pub struct Field {
    pub rule : CellularAutomatonParams,
    pub data : Vec<Vec<bool>>,
}


impl Field
{
    pub fn new(rule: CellularAutomatonParams, width: u8, height: u8) -> Self {
        let mut data = vec![vec![false; width as usize]; height as usize];
        for x in 0..(height as usize) {
            for y in 0..(width as usize) {
                let mut rng = thread_rng();
                data[x][y] = rng.gen_bool(rule.initial_density)
            }

        }
        Self{rule, data}
    }

    fn get_num_of_alive_neighbors(&self, x: i16, y: i16) -> u8 {

        let height = self.data.len() as i16;
        let width = self.data[0].len() as i16;

        let mut alive_neighbors: u8 = 0;

        let offset: Vec<i16> = (-(self.rule.neighborhood_radius as i16)..=(self.rule.neighborhood_radius as i16)).collect();
        
        for dx in offset.iter(){
            for dy in offset.iter(){
                if *dx == 0 && *dy == 0 { continue; }

                let nx = x + *dx;
                let ny = y + *dy;

                if (nx >= 0 && nx < height) && (ny >= 0 && ny < width){
                    alive_neighbors += if self.data[nx as usize][ny as usize] {1} else {0};
                }
            }
        }
        alive_neighbors
    }

    pub fn update(&mut self) {

        let mut data = self.data.clone();
        let height = data.len();
        let width = data[0].len();

        if self.rule.neighborhood_type == Neighborhood::Moore {
            for x in 0..height{
                for y in 0..width{
                    let alive_neighbors = self.get_num_of_alive_neighbors(x as i16, y as i16);
                    if data[x][y] && !self.rule.survive_array.contains(&alive_neighbors) {
                        data[x][y] = false;
                    }
                    else if !data[x][y] && self.rule.birth_array.contains(&alive_neighbors){
                        data[x][y] = true;
                    }
                }
            }
        }
        
        self.data = data;
    }
    
}
use crate::rules::CellularAutomatonParams;
use rand::{thread_rng, Rng};
use std::fmt;


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

    pub fn update(&mut self) {

        let mut data = self.data.clone();
        
        let height = data.len();
        let width = data[0].len();
        
        let mut num_of_neighbors;
        for x in 0..height {
            for y in 0..width {
                num_of_neighbors = self.rule.neighborhood_type.get_num_of_neighbors(&self, x as isize, y as isize);

                if self.data[x][y] {
                    data[x][y] = self.rule.survive_array.contains(&num_of_neighbors);
                }
                else {
                    data[x][y] = self.rule.birth_array.contains(&num_of_neighbors);
                }
            }
        }

        self.data = data;
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width  = self.data.len();
        let height = self.data[0].len();

        write!(f, "╔").unwrap();
        for _ in 0..(2 * height) {
            write!(f, "═").unwrap();
        }
        writeln!(f, "╗").unwrap();

        for x in 0..height {
            write!(f, "║").unwrap();
            for y in 0..width {
                write!(f, "{} ", if self.data[x as usize][y as usize] {"x"} else {" "}).unwrap();
            }
            writeln!(f, "║").unwrap();
        }

        write!(f, "╚").unwrap();
        for _ in 0..(2 * height) {
            write!(f, "═").unwrap();
        }
        write!(f, "╝").unwrap();

        write!(f, "")
    }
}
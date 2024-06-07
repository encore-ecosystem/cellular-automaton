use crate::field::Field;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Neighborhood {
    Moore,
    VonNeumann,
}

impl Distribution<Neighborhood> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Neighborhood {
        match rng.gen_range(0..=1) {
            0 => Neighborhood::Moore,
            _ => Neighborhood::VonNeumann,
        }
    }
}


impl Neighborhood
{
    pub fn get_num_of_neighbors(&self, field: &Field, x: isize, y: isize) -> usize {
        
        let height = field.data.len() as isize;
        let width = field.data[0].len() as isize;

        let mut num_of_neighbors: usize = 0;
        match *self
        {
            Self::Moore => {
                for n_x in (x - field.rule.neighborhood_radius as isize)..=(x + field.rule.neighborhood_radius as isize) {
                    for n_y in (y - field.rule.neighborhood_radius as isize)..=(y + field.rule.neighborhood_radius as isize) {
                        let n_x_on_tor = n_x.rem_euclid(height);
                        let n_y_on_tor = n_y.rem_euclid(width) ;

                        if (n_x == x) && (n_y == y) {
                            continue;
                        }
                        
                        num_of_neighbors += if field.data[n_x_on_tor as usize][n_y_on_tor as usize] {1} else {0};
                    }
                }
            }
            Self::VonNeumann => {
                for dh in (-(field.rule.neighborhood_radius as isize))..=(field.rule.neighborhood_radius as isize) {
                    for dw in (dh.abs() - field.rule.neighborhood_radius as isize)..=((dh.abs() - field.rule.neighborhood_radius as isize + 1).abs()) {
                        if dh != 0 && dw != 0
                        {
                            let n_x_on_tor = (x + dh).rem_euclid(height);
                            let n_y_on_tor = (y + dw).rem_euclid(width);

                            num_of_neighbors += if field.data[n_x_on_tor as usize][n_y_on_tor as usize] {1} else {0};
                        }
                    }
                }
            }
        }
        num_of_neighbors
    }

    pub fn get_max_neighbors(&self, r: usize) -> usize
    {
        match *self {
            Self::VonNeumann => {
                2*r*r + 2*r
            }
            Self::Moore      => {
                4*r*r + 4*r
            }
        }
    }
}

#[derive(Debug)]
pub struct CellularAutomatonParams {
    pub neighborhood_type   : Neighborhood,
    pub neighborhood_radius : u8,   
    pub initial_density     : f64,
    pub survive_array       : Vec<usize>,
    pub birth_array         : Vec<usize>,
}

#[derive(Debug)]

#[derive(PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum Neighborhood {
    Moore,            //square
    VonNeumann,       //rhombus
}

#[derive(Debug, Clone)]
pub struct CellularAutomatonParams {
    pub neighborhood_type   : Neighborhood,
    pub neighborhood_radius : u8,   
    pub initial_density     : f64,
    pub survive_array       : Vec<u8>,
    pub birth_array         : Vec<u8>,
}

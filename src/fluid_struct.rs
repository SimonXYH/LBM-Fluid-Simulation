// use crate::lattice_state::{Cell, FluidState};
// use crate::tup2::Tup2;
// use ndarray::Array2;
// use crate::obstacle_board::ObstacleBoard;
// 
// pub struct LBFluidSim {
//     pub states_array: Array2<Cell>,
//     pub obstacle_board: ObstacleBoard,
//     pub shape: (usize, usize),
// }
// 
// impl LBFluidSim {
//     pub fn new(
//         obstacle_board: ObstacleBoard,
//         f_density: fn(Tup2<f32>) -> f32,
//         f_velocity: fn(Tup2<f32>) -> Tup2<f32>,
//     ) -> Self {
//         let shape = obstacle_board.grid_stats.shape;
//         let density_field = obstacle_board.grid_stats.compute_scalar_field(f_density);
//         let velocity_field = obstacle_board.grid_stats.compute_vector_field(f_velocity);
//         let mut states_array = Array2::default(shape);
//         for i in 0..shape.0 {
//             for j in 0..shape.1 {
//                 let linear_index = obstacle_board.get_index(i, j);
//                 if obstacle_board.bit_vec.get(linear_index).unwrap() {
//                     states_array[[i, j]] = Cell::Obstacle
//                 } else {
//                     states_array[[i, j]] = Cell::Fluid(FluidState::new_equilibrium(
//                         density_field[[i, j]],
//                         velocity_field[[i, j]],
//                     ))
//                 }
//             }
//         }
//         LBFluidSim {
//             states_array,
//             obstacle_board,
//             shape,
//         }
//     }
//     pub fn get_state(states_array: &Array2<Cell>, i: isize, j: isize, shape: (usize, usize)) -> Cell {
//         let i = i.rem_euclid(shape.0 as isize) as usize;
//         let j = j.rem_euclid(shape.1 as isize) as usize;
//         states_array[[i, j]]
//     }
// 
//     pub fn get_neighbor_states(states_array: &Array2<Cell>, i: isize, j: isize, shape: (usize, usize)) -> [Cell; 9] {
//         let center = Self::get_state(states_array, i, j, shape);
//         let east = Self::get_state(states_array, i + 1, j, shape);
//         let north = Self::get_state(states_array, i, j + 1, shape);
//         let west = Self::get_state(states_array, i - 1, j, shape);
//         let south = Self::get_state(states_array, i, j - 1, shape);
//         let ne = Self::get_state(states_array, i + 1, j + 1, shape);
//         let nw = Self::get_state(states_array,i - 1, j + 1, shape);
//         let sw = Self::get_state(states_array, i - 1, j - 1, shape);
//         let se = Self::get_state(states_array,i + 1, j - 1, shape);
//         [center, east, north, west, south, ne, nw, sw, se]
//         // [west, south, east, north, sw, se, ne, nw]
//     }
//     pub fn collision(&mut self) {
//         for i in 0..self.shape.0 {
//             for j in 0..self.shape.1 {
//                 match Self::get_state(&self.states_array, i as isize, j as isize, self.shape) {
//                     Cell::Fluid(ref mut fluid_state) => {
//                         fluid_state.collide();
//                     }
//                     Cell::Obstacle => {}
//                 }
//             }
//         }
//     }
// 
//     pub fn advection(&mut self){
//         let states_clone = self.states_array.clone();
//         for i in 0..self.shape.0{
//             for j in 0..self.shape.1{
//                 let neighbor_states = Self::get_neighbor_states(&states_clone, i as isize, j as isize, self.shape);
//                 match &mut self.states_array[[i, j]] {
//                     Cell::Obstacle => {},
//                     Cell::Fluid(fluid_state) => {
//                         fluid_state.advect(neighbor_states);
//                         // fluid_state.update_distribution();
//                     }
//                 }
//                 if (i == 5 && j == 5){
//                     println!("{:?}", self.states_array[[i, j]])
//                 }
//             }
//         }
//     }
//     pub fn update_distribution(&mut self){
//         for i in 0..self.shape.0{
//             for j in 0..self.shape.1{
//                 match &mut self.states_array[[i, j]]{
//                         Cell::Obstacle => {},
//                     Cell::Fluid(fluid_state) => {
//                         fluid_state.update_distribution()
//                     }
//                     
//                 }
//             }
//         }
//     }
// 
// 
//     pub fn update(&mut self) {
//         // self.collision();
//         self.advection();
//         self.update_distribution()
//     }
// }
use crate::lattice_state::{Cell, FluidState, E_I, OPPOSITE};
use crate::tup2::Tup2;
use ndarray::Array2;
use crate::obstacle_board::ObstacleBoard;

pub struct LBFluidSim {
    pub states_curr: Array2<Cell>, // distribution at time t
    pub states_next: Array2<Cell>, // distribution at time t+1
    pub obstacle_board: ObstacleBoard,
    pub shape: (usize, usize),
}

impl LBFluidSim {
    pub fn new(
        obstacle_board: ObstacleBoard,
        f_density: fn(Tup2<f32>) -> f32,
        f_velocity: fn(Tup2<f32>) -> Tup2<f32>,
    ) -> Self {
        let shape = obstacle_board.grid_stats.shape;
        let density_field = obstacle_board.grid_stats.compute_scalar_field(f_density);
        let velocity_field = obstacle_board.grid_stats.compute_vector_field(f_velocity);

        let mut states_curr = Array2::default(shape);
        let mut states_next = Array2::default(shape);

        for i in 0..shape.0 {
            for j in 0..shape.1 {
                let linear_index = obstacle_board.get_index(i, j);
                if obstacle_board.bit_vec.get(linear_index).unwrap() {
                    states_curr[[i, j]] = Cell::Obstacle;
                    states_next[[i, j]] = Cell::Obstacle;
                } else {
                    let fs = FluidState::new_equilibrium(
                        density_field[[i, j]],
                        velocity_field[[i, j]],
                    );
                    states_curr[[i, j]] = Cell::Fluid(fs);
                    states_next[[i, j]] = Cell::Fluid(FluidState::default());
                }
            }
        }

        LBFluidSim {
            states_curr,
            states_next,
            obstacle_board,
            shape,
        }
    }

    #[inline]
    fn idx_wrap(i: isize, n: usize) -> usize {
        i.rem_euclid(n as isize) as usize
    }

    #[inline]
    fn neighbor_coord((i, j): (usize, usize), dir: usize, shape: (usize, usize)) -> (usize, usize) {
        let di = E_I[dir].0 as isize;
        let dj = E_I[dir].1 as isize;
        let ni = Self::idx_wrap(i as isize + di, shape.0);
        let nj = Self::idx_wrap(j as isize + dj, shape.1);
        (ni, nj)
    }

    pub fn collide(&mut self) {
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                if let Cell::Fluid(ref mut fs) = self.states_curr[[i, j]] {
                    fs.density = fs.density_sum();
                    fs.velocity = fs.velocity_sum();

                    fs.collide();
                }
            }
        }
    }

    pub fn stream(&mut self) {
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                match self.states_next[[i, j]] {
                    Cell::Obstacle => {
                    }
                    Cell::Fluid(ref mut fs_next) => {
                        fs_next.d_i = [0.0; 9];
                        fs_next.density = 0.0;
                        fs_next.velocity = Tup2(0.0, 0.0);
                    }
                }
            }
        }

        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                match self.states_curr[[i, j]] {
                    Cell::Obstacle => {
                    }
                    Cell::Fluid(ref fs_curr) => {
                        for q in 0..9 {
                            let fi = fs_curr.d_i[q];
                            if fi == 0.0 {
                                continue;
                            }
                            let (ni, nj) = Self::neighbor_coord((i, j), q, self.shape);
                            match self.states_curr[[ni, nj]] {
                                Cell::Obstacle => {
                                    if let Cell::Fluid(ref mut dst) = self.states_next[[i, j]] {
                                        let opp = OPPOSITE[q];
                                        dst.d_i[opp] += fi;
                                    }
                                }
                                Cell::Fluid(_) => {
                                    if let Cell::Fluid(ref mut dst) = self.states_next[[ni, nj]] {
                                        dst.d_i[q] += fi;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut self.states_curr, &mut self.states_next);
    }

    pub fn update_macros(&mut self) {
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                if let Cell::Fluid(ref mut fs) = self.states_curr[[i, j]] {
                    fs.density = fs.density_sum();
                    fs.velocity = fs.velocity_sum();
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.collide();
        self.stream();
        self.update_macros();
    }
}
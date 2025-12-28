use bit_vec::BitVec;
use ndarray::Array2;
use crate::tup2::Tup2;

#[derive(Clone, Debug)]
pub struct GridStats {
    pub shape: (usize, usize),
    pub length: f32,
    pub grid_pos: Array2<Tup2<f32>>,
    pub max_dim: f32,
    pub spacing: f32,
    pub spacing_inverse: f32,
    pub mid_point: Tup2<f32>,
}
impl GridStats {
    pub fn new(shape: (usize, usize), length: f32) -> Self {
        let max_dim = ((shape.0 - 1).max(shape.1 - 1)) as f32;
        let spacing = length / max_dim;
        let spacing_inverse = spacing.recip();
        let mid_point = Tup2(shape.0 - 1, shape.1 - 1).to_f32() / 2.;
        let grid_pos = Array2::default(shape);
        let mut grid_stats = GridStats {
            shape,
            length,
            grid_pos,
            max_dim,
            spacing,
            spacing_inverse,
            mid_point,
        };
        grid_stats.grid_pos = grid_stats.compute_grid();
        grid_stats
    }

    pub fn compute_grid(&self) -> Array2<Tup2<f32>> {
        let mut grid_pos = Array2::default(self.shape);
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                grid_pos[[i, j]] = (Tup2(i, j).to_f32() - self.mid_point) * self.spacing
            }
        }
        grid_pos
    }

    pub fn compute_scalar_field(&self, function: fn(Tup2<f32>) -> f32) -> Array2<f32> {
        Array2::from_shape_fn(self.shape, |(i, j)| function(self.grid_pos[[i, j]]))
    }
    pub fn compute_vector_field(&self, function: fn(Tup2<f32>) -> Tup2<f32>) -> Array2<Tup2<f32>> {
        Array2::from_shape_fn(self.shape, |(i, j)| function(self.grid_pos[[i, j]]))
    }
}

pub struct ObstacleBoard {
    pub grid_stats: GridStats,
    pub bit_vec: BitVec,
}

impl ObstacleBoard {
    pub fn new_empty(grid_stats: GridStats) -> Self {
        let bit_vec = BitVec::from_elem(grid_stats.shape.0 * grid_stats.shape.1, false);
        ObstacleBoard {
            grid_stats,
            bit_vec,
        }
    }
    pub fn get_index(&self, i: usize, j: usize) -> usize {
        let index = i + j * self.grid_stats.shape.0;
        index
    }

    pub fn set_obstacle(&mut self, i: usize, j: usize) {
        let index = self.get_index(i, j);
        self.bit_vec.set(index, true)
    }

    pub fn new(grid_stats: GridStats, filled_index: Vec<(usize, usize)>) -> Self {
        let mut obstacle_grid = ObstacleBoard::new_empty(grid_stats);
        for indices in filled_index {
            obstacle_grid.set_obstacle(indices.0, indices.1)
        }
        obstacle_grid
    }

    pub fn exist_obstacle(&self, i: usize, j: usize) -> bool {
        let index = self.get_index(i, j);
        self.bit_vec.get(index).unwrap()
    }

    pub fn pos_to_grid_index(&self, pos: Tup2<f32>) -> Tup2<usize> {
        let index = ((pos * self.grid_stats.spacing_inverse) + self.grid_stats.mid_point).floor();
        index
    }
    pub fn pos_to_linear_index(&self, pos: Tup2<f32>) -> usize {
        let grid_index = self.pos_to_grid_index(pos);
        let index = self.get_index(grid_index.0, grid_index.1);
        // println!("linear index: {}", index);
        index
    }

    pub fn set_line(&mut self, p1: Tup2<f32>, p2: Tup2<f32>) {
        let grid_i1 = self.pos_to_grid_index(p1);
        let grid_i2 = self.pos_to_grid_index(p2);
        // println!("grid_i1: {:?}", grid_i1.1);
        // println!("grid_i2: {:?}", grid_i2.1);
        // print!("y displacement: {}", grid_i2.1 - grid_i1.1);
        let (ix_min, ix_max) = (grid_i1.0.min(grid_i2.0), grid_i1.0.max(grid_i2.0));
        let (iy_min, iy_max) = (grid_i1.1.min(grid_i2.1), grid_i1.1.max(grid_i2.1));
        if ix_max - ix_min > iy_max - iy_min {
            let gradient = (grid_i2.1 as isize - grid_i1.1 as isize) as f32 / (grid_i2.0 as isize- grid_i1.0 as isize) as f32;
            // print!("y displacement: {}", grid_i2.1 - grid_i1.1);
            // print!("x displacement: {}", grid_i2.0 - grid_i1.0);
            // println!("gradient 1: {}", gradient);
            for x in ix_min..ix_max + 1 {
                let y = ((gradient * (x as f32 - grid_i1.0 as f32)) + grid_i1.1 as f32).round()
                    as usize;
                self.set_obstacle(x, y)
            }
        } else {
            let gradient = (grid_i2.0 as isize- grid_i1.0 as isize) as f32 / (grid_i2.1 as isize - grid_i1.1 as isize) as f32;
            // println!("gradient: {}", gradient);
            for y in iy_min..iy_max + 1 {
                let x = ((gradient * (y as f32 - grid_i1.1 as f32)) + grid_i1.0 as f32).round()
                    as usize;
                self.set_obstacle(x, y)
            }
        }
    }
    pub fn set_piecewise_curve(&mut self, vertices: Vec<Tup2<f32>>) {
        for i in 0..vertices.len() - 1 {
            self.set_line(vertices[i], vertices[i + 1])
        }
    }
}

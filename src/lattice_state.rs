use crate::tup2::Tup2;
const W_REST: f32 = 4. / 9.;
const W_CARDINAL: f32 = 1. / 9.;
const W_DIAGONAL: f32 = 1. / 36.;
const WEIGHTS: [f32; 9] = [
    W_REST, W_CARDINAL, W_CARDINAL, W_CARDINAL, W_CARDINAL, W_DIAGONAL, W_DIAGONAL, W_DIAGONAL,
    W_DIAGONAL,
];
const C: f32 = 1. / 1.732_050;
const C2: f32 = C * C;
const C4: f32 = C2 * C2;
pub(crate) const E_I: [Tup2<f32>; 9] = [
    Tup2(0., 0.),
    Tup2(1., 0.),
    Tup2(0., 1.),
    Tup2(-1., 0.),
    Tup2(0., -1.),
    Tup2(1., 1.),
    Tup2(-1., 1.),
    Tup2(-1., -1.),
    Tup2(1., -1.),
];

pub(crate) const DISCRETE_VEL: [Tup2<f32>; 9] = [
    Tup2(0., 0.),
    Tup2(C, 0.),
    Tup2(0., C),
    Tup2(-C, 0.),
    Tup2(0., -C),
    Tup2(C, C),
    Tup2(-C, C),
    Tup2(-C, -C),
    Tup2(C, -C),
];
const TAU: f32 = 0.6;
const TAU_INVERSE: f32 = 1. / TAU;
pub const OPPOSITE: [usize; 9] = [0, 3, 4, 1, 2, 7, 8, 5, 6];

#[derive(Clone, Copy, Debug)]
pub struct FluidState {
    pub d_i: [f32; 9],
    pub velocity: Tup2<f32>,
    pub density: f32,
}
#[derive(Default, Copy, Clone, Debug)]
pub enum Cell {
    Fluid(FluidState),
    #[default]
    Obstacle,
}
pub fn mul_array_k(a1: [f32; 9], k: f32) -> [f32; 9] {
    let mut array = [0.; 9];
    for i in 0..9 {
        array[i] = a1[i] * k
    }
    array
}

pub fn sub_array(a1: [f32; 9], a2: [f32; 9]) -> [f32; 9] {
    let mut array = [0.; 9];
    for i in 0..9 {
        array[i] = a1[i] - a2[i]
    }
    array
}

impl Default for FluidState {
    fn default() -> Self {
        FluidState {
            d_i: [0.; 9],
            velocity: Tup2::default(),
            density: 0.,
        }
    }
}

impl FluidState {
    pub fn new(d_i: [f32; 9], velocity: Tup2<f32>, density: f32) -> Self {
        FluidState {
            d_i,
            velocity,
            density,
        }
    }
    pub fn get_equilibrium_density(density: f32, velocity: Tup2<f32>) -> [f32; 9] {
        let mut density_array = [0.; 9];
        let v_mag = velocity.mag_sq();
        for i in 0..9 {
            let product = E_I[i].dot(velocity);
            density_array[i] = WEIGHTS[i]
                * density
                * (1. + product / C2 + product.powi(2) / (2. * C4) - v_mag / (2. * C2))
        }
        density_array
    }
    pub fn new_equilibrium(density: f32, velocity: Tup2<f32>) -> Self {
        FluidState {
            d_i: Self::get_equilibrium_density(density, velocity),
            velocity,
            density,
        }
    }


    pub fn density_sum(&self) -> f32 {
        self.d_i.iter().sum()
    }

    pub fn velocity_sum(&self) -> Tup2<f32> {
        let mut momentum = Tup2(0., 0.);
        for i in 0..9 {
            momentum = momentum + E_I[i] * self.d_i[i]
        }
        momentum / self.density
    }

    pub fn collide(&mut self) {
        self.d_i = sub_array(
            self.d_i,
            mul_array_k(
                sub_array(
                    self.d_i,
                    Self::get_equilibrium_density(self.density, self.velocity),
                ),
                TAU_INVERSE,
            ),
        );
    }
    
    pub fn advect(&mut self, neighbors_state: [Cell; 9]) {
        for i in 0..9 {
            match neighbors_state[i] {
                Cell::Obstacle => self.d_i[OPPOSITE[i]] = self.d_i[i],
                Cell::Fluid(fluid_state) => self.d_i[i] = fluid_state.d_i[OPPOSITE[i]],
            }
        }
    }
    
    pub fn update_distribution(&mut self){
        self.density = self.density_sum();
        self.velocity = self.velocity_sum()
    }
    
}

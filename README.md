# Lattice Boltzmann Fluid Simulation (Rust)

This is a 2D fluid simulation written in **Rust**, using the
**Lattice Boltzmann Method (LBM)** with a **D2Q9 lattice** and the
**BGK collision model**.

This is my first serious attempt at building a fluid simulation from scratch, and my aims are:
- understanding how the LBM algorithm work exactly
- exploring how fast I can implement it in Rust
- explore the power of this model by testing the simulation in different geometery

---

## What’s implemented
The goal was to build a fluid simulation from scratch, so the only two important crates used are a sdl2 crate for drawing, and the ndarray for array calculations
- vec2 struct (2d vector calculations)
- tup2 struct (2d tuple calculations)
- obstacle_board struct (fast bitboard manipulation to set the geometry in which fluid is simulated)
- fluid_element struct
- 2D D2Q9 lattice struct (a grid of fluid_elements, contains update method such as collision, advection)
- BGK (single relaxation time) collision method

---

## Lattice and weights
The code uses the standard **D2Q9** velocity set:
- one rest direction
- four cardinal directions
- four diagonal directions

Weights:
- Rest: `4/9`
- Cardinal: `1/9`
- Diagonal: `1/36`



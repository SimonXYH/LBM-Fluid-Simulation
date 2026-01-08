# Lattice Boltzmann Fluid Simulation 

<img width="1563" height="848" alt="image" src="https://github.com/user-attachments/assets/b6ebbd4b-eec7-4fee-98d7-f9604aed2bdb" />

This is a 2D fluid simulation written in Rust, using the
Lattice Boltzmann Method with a D2Q9 lattice and the
BGK collision model.

This is my first serious attempt at building a fluid simulation from scratch:)
The main goals of this project are:
- understanding how the LBM algorithm works
- exploring how efficiently it can be implemented in Rust
- playing with different geometries and observe behaviours of fluid

This is a learning / exploration project rather than a efficient CFD solver. 

---

## What’s implemented

This project is pretty much built from scratch.
The only major external crates used are:
- **SDL2** for visualization
- **ndarray** for storing the simulation grid and for doing computations on it

Core components:
- `vec2` – struct for 2D vector math
- `tup2` – lightweight 2D tuple type, sometimes easier to use than the vec2 struct
- `GridStats` – handles grid geometry (shape, length), and allows the computation of 'scalar field' and 'vector field' on it
- `ObstacleBoard` – fast bitboard-based obstacle representation, able to map lines drawn in position space to flipped bit in the bitboard, generalized so any piece-wise curve can be drawn (such as an entire maze)
- `FluidState` – stores density and velcocities in a single fluid type cell
- `Cell` – enum distinguishing fluid cells from obstacles,useful for boundary condition handling later
- `LBFluidSim` – the main struct for setting up a simulation  (collision + streaming)
- 2D **D2Q9 lattice**
- **BGK (single relaxation time)** collision model
- Bounce-back boundary conditions at obstacles

---

## Lattice and weights

The simulation uses the standard **D2Q9** velocity set:
- one rest direction
- four cardinal directions
- four diagonal directions

Weights:
- Rest: `4/9`
- Cardinal: `1/9`
- Diagonal: `1/36`

## Abstraction layers

The code is organized in a few clear layers of abstraction, going from
low-level math up to the full simulation loop. The idea is
that each layer does a specific thing, and the lower level structs are passed up the layers only when necessary. 

---

### 1. Math primitives

At the lowest level are small math types like `Tup2<f32>`.

These are simple 2D vector types that is used all over the place, they could represent:
- positions in space
- velocities
- discrete lattice directions
- 'index vectors' for computing stuff on the grid 

They provide basic operations (addition, dot product, norms, etc.) and
are intentionally lightweight, since they’re used everywhere.

---

### 2. Constants 

This layer defines everything that is demanded by the D2Q9 LBM model:
- discrete velocity directions (`E_I`)
- physical velocities (`DISCRETE_VEL`)
- lattice weights
- opposite-direction lookup table (`OPPOSITE`)
- relaxation time `τ`

---

### 3. `FluidState`

`FluidState` represents the full physical state of **one fluid cell**.

It stores:
- the distribution functions `f_i`
- density `ρ`
- velocity `u`

This layer is responsible for:
- computing equilibrium distributions
- calculating density and velocity from `f_i`

`FluidState` knows nothing about its neighbors. 

---

### 4. `Cell`

Each grid location is represented by a `Cell`, which is either:
- `Fluid(FluidState)`
- `Obstacle`

This makes boundary condition handling simple to keep track of and avoids adding too much special cases
throughout the code. Obstacle logic is handled at a higher level,
not inside `FluidState`.

---

### 5. Geometry of simulation domain

This layer describes the geometry of the simulation domain.

- `GridStats` handles grid size, spacing, and conversions between grid
  indices and physical coordinates.
- `ObstacleBoard` stores solid geometry efficiently using a bit vector
  and provides method for drawing lines, which is naturally extended to a function that allows drawing any piecewise curve. 


---

### 6. `LBFluidSim`: Simulation setup

`LBFluidSim` is the top-level simulation object and ties everything
together. It implements a double buffering update system.

It owns:
- the current lattice state (`states_curr`)
- the next lattice state (`states_next`)
- the obstacle board and grid shape

Its responsibilities are:
- applying the collision step to all fluid cells
- streaming distributions between neighboring cells
- handling bounce-back at obstacles
- updating density and velocities

Questions and possible improvements:
1. Limited understanding of the Boltzmann equation itself
2. Simulation failed to simulate KH instability (what went wrong? Is it just a matter of fine-tuning the parameters?)



use crate::scene2d::Scene2D;
use crate::vec2::Vec2;
use ndarray::{Array1, Array2};
use palette::{FromColor, Hsl, Srgb};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::fluid_struct::LBFluidSim;
use crate::lattice_state::{Cell, FluidState};
use crate::obstacle_board::GridStats;

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let hsl = Hsl::new(h, s, l);
    let rgb: Srgb = Srgb::from_color(hsl);
    let (r, g, b) = rgb.into_components();
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}
pub struct Canvas2D {
    pub canvas: WindowCanvas,
    pub size: Vec2<f32>,
    pub canvas_zoom: f32,
    pub canvas_center: Vec2<f32>,
    pub bg_color: (u8, u8, u8),
    pub scene2d_list: Vec<Scene2D>,
}

impl Canvas2D{
    pub fn new(
        canvas: WindowCanvas,
        size: Vec2<f32>,
        canvas_zoom: f32,
        canvas_center: Vec2<f32>,
        bg_color: (u8, u8, u8),
        scene2d_list: Vec<Scene2D>,
    ) -> Self {
        Canvas2D {
            canvas,
            size,
            canvas_zoom,
            canvas_center,
            bg_color,
            scene2d_list,
        }
    }

    pub fn correct_coord_to_point(&self, point: Vec2<f32>) -> Point {
        let point = (point.negate_y() * self.canvas_zoom + self.size / 2.).to_i32();
        Point::new(point.x, point.y)
    }

    pub fn corrected_coord_to_vec(&self, point: Vec2<f32>) -> Vec2<f32> {
        point.negate_y() * self.canvas_zoom + self.size / 2.
    }

}

impl Canvas2D{
    pub fn draw_point(&mut self, point: Vec2<f32>) {
        self.canvas
            .draw_point(self.correct_coord_to_point(point))
            .unwrap()
    }

    pub fn draw_line(&mut self, point_1: Vec2<f32>, point_2: Vec2<f32>) {
        self.canvas
            .draw_line(
                self.correct_coord_to_point(point_1),
                self.correct_coord_to_point(point_2),
            )
            .unwrap()
    }
    pub fn draw_point_s2(&mut self, point_1: Vec2<f32>, scene_num: usize) {
        let scene = self.scene2d_list[scene_num];
        let projected_point = scene.project(point_1);
        self.draw_point(projected_point)
    }

    pub fn draw_line_s2(&mut self, point_1: Vec2<f32>, point_2: Vec2<f32>, scene_num: usize) {
        let scene = self.scene2d_list[scene_num];
        let point_1 = scene.project(point_1);
        let point_2 = scene.project(point_2);
        self.draw_line(point_1, point_2)
    }
    pub fn draw_axis_s2(&mut self, ratio: f32, scene_num: usize) {
        self.draw_line_s2(
            Vec2::new(-self.size.x, 0.) * ratio * 0.5,
            Vec2::new(self.size.x, 0.) * ratio * 0.5,
            scene_num,
        );
        self.draw_line_s2(
            Vec2::new(0., -self.size.y) * ratio * 0.5,
            Vec2::new(0., self.size.y) * ratio * 0.5,
            scene_num,
        );
    }



}
impl Canvas2D {
    pub fn draw_rect(
        &mut self,
        center: Vec2<f32>,
        mut width: f32,
        mut height: f32,
        color: (u8, u8, u8),
    ) {
        let center = self.corrected_coord_to_vec(center);
        width *= self.canvas_zoom;
        height *= self.canvas_zoom;
        self.canvas
            .set_draw_color(Color::RGB(color.0, color.1, color.2));
        self.canvas
            .fill_rect(Rect::new(
                (center.x - width / 2.) as i32,
                (center.y - height / 2.) as i32,
                width.ceil() as u32,
                height.ceil() as u32,
            ))
            .unwrap()
    }

    pub fn draw_rect_s2(
        &mut self,
        center: Vec2<f32>,
        mut width: f32,
        mut height: f32,
        color: (u8, u8, u8),
        scene_num: usize,
    ) {
        let scene = self.scene2d_list[scene_num];
        let projected_center = scene.project(center);
        width *= scene.scene_zoom;
        height *= scene.scene_zoom;
        self.draw_rect(projected_center, width, height, color)
    }


    pub fn draw_square(&mut self, center: Vec2<f32>, mut side_length: f32, color: (u8, u8, u8)) {
        let center = self.corrected_coord_to_vec(center);
        side_length *= self.canvas_zoom;
        self.canvas
            .set_draw_color(Color::RGB(color.0, color.1, color.2));
        self.canvas
            .fill_rect(Rect::new(
                (center.x - side_length / 2.) as i32,
                (center.y - side_length / 2.) as i32,
                side_length.ceil() as u32,
                side_length.ceil() as u32,
            ))
            .unwrap()
    }

    pub fn draw_square_s2(
        &mut self,
        center: Vec2<f32>,
        mut side_length: f32,
        color: (u8, u8, u8),
        scene_num: usize,
    ) {
        let scene = self.scene2d_list[scene_num];
        let projected_center = scene.project(center);
        side_length *= scene.scene_zoom;
        self.draw_square(projected_center, side_length, color)
    }

}

impl Canvas2D{
    pub fn draw_scattered_array(
        &mut self,
        x_array: &Array1<f32>,
        y_array: &Array1<f32>,
        scene_num: usize,
        connect: bool,
    ) {
        let num = x_array.len();
        for i in 0..num {
            let point = Vec2::new(x_array[i], y_array[i]);
            self.draw_point_s2(point, scene_num)
        }
        if connect {
            for i in 0..num - 1 {
                let start = Vec2::new(x_array[i], y_array[i]);
                let end = Vec2::new(x_array[i + 1], y_array[i + 1]);
                self.draw_line_s2(start, end, scene_num)
            }
        }
    }
    pub fn draw_fluid_density(
        &mut self,
        grid_stats: &GridStats,
        fluid: &LBFluidSim,
        max_density: f32,
        scene_num: usize,
    ) {
        let grid_pos = &grid_stats.grid_pos;
        for i in 0..fluid.shape.0 {
            for j in 0..fluid.shape.1 {
                let density = match fluid.states_curr[[i, j]] {
                    Cell::Fluid(fluid_state) => fluid_state.density,
                    Cell::Obstacle => -1.,
                };
                let color = if density < 0. {
                    (255, 0, 0)
                } else {
                    let density = (density / max_density).clamp(0., 1.);
                    hsl_to_rgb(240., 1., 0.5 * density)
                };

                self.draw_square_s2(
                    grid_pos[[i, j]].to_vec2(),
                    grid_stats.spacing,
                    color,
                    scene_num,
                )
            }
        }
    }
    pub fn draw_fluid_velocity(&mut self, grid_stats: &GridStats, fluid: &LBFluidSim, max_len: f32, jump_index: usize, scene_num: usize){
        self.canvas
            .set_draw_color(Color::RGB(255, 255, 255));
        for i in (0..grid_stats.shape.0).step_by(jump_index){
            for j in (0..grid_stats.shape.1).step_by(jump_index){
                let pos = grid_stats.grid_pos[[i, j]];
                let state = fluid.states_curr[[i, j]];
                match state {
                    Cell::Obstacle => {},
                    Cell::Fluid(fluid_state) => {
                        let vel = fluid_state.velocity;
                        let normed_vel = fluid_state.velocity.normalize() * max_len;
                        
                        self.draw_line_s2(
                            pos.to_vec2(),
                            (pos + normed_vel * max_len).to_vec2(),
                            scene_num
                        )
                    }
                }
            }
        }

    }
}
use crate::canvas::Canvas2D;
use crate::scene2d::Scene2D;
use crate::tup2::Tup2;
use crate::vec2::Vec2;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::pixels::Color;
use std::time::Duration;
use crate::fluid_struct::LBFluidSim;
use crate::obstacle_board::{GridStats, ObstacleBoard};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let size = Vec2::new(1200., 900.);

    let window = video_subsystem
        .window("rust-sdl2 demo", size.x as u32, size.y as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let scene2d_list = vec![Scene2D::new(800., Vec2::new(0., 0.))];
    let mut canvas2d = Canvas2D::new(canvas, size, 1., Vec2::new(0., 0.), (0, 0, 0), scene2d_list);

    const LENGTH: f32 = 1.;
    let grid_stats = GridStats::new((200, 100), LENGTH);
    let mut obstacle_board = ObstacleBoard::new_empty(grid_stats.clone());
    // obstacle_board.set_piecewise_curve(
    //     vec![Tup2(-0.3, 0.03), Tup2(0.3, 0.03)]
    // );
    // obstacle_board.set_piecewise_curve(
    //     vec![Tup2(-0.3, -0.03), Tup2(0.3, -0.03)]
    // );
    // obstacle_board.set_piecewise_curve(
    //     vec![Tup2(0., 0.2), Tup2(0.3, 0.), Tup2(0., -0.3), Tup2(-0.3, 0.), Tup2(0., 0.3)]
    // );
    obstacle_board.set_piecewise_curve(
        vec![Tup2(0., 0.1), Tup2(0., -0.1)]
    );
    const MAX_DENSITY: f32= 1.;
    fn f_density(pos: Tup2<f32>) -> f32{
        // 1.
        // // (0.55 * LENGTH + 0.7 * pos.0) * MAX_DENSITY
        1.1- (0.1 * pos.0 + 0.5)
        // if pos.0.abs() > 0.1{
        //     0.1
        // }
        // else{
        //     0.5
        // }
        // if pos.1 < 0.07 * (5. * pos.0).sin(){
        //     1.
        // }
        // else{
        //     0.6
        // }
    }

    fn f_velocity(pos: Tup2<f32>) -> Tup2<f32>{
        Tup2(0.1, 0.)
        // // Tup2(0., 0.)
        // Tup2(pos.1, -pos.0).normalize() * 0.1
          // Tup2(pos.0.clamp(-0.2, 0.2), 0.)
        // if pos.1 < 0.{
        //     Tup2(0.5, 0.)
        // }
        // else{
        //     Tup2(0., 0.)
        // }
    }

    let mut fluid_sim = LBFluidSim::new(obstacle_board, f_density, f_velocity);

    let mut T = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas2d.canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas2d.canvas.clear();
        canvas2d
            .canvas
            .set_draw_color(Color::RGBA(255, 255, 255, 150));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    keymod,
                    ..
                } => {
                    if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                        canvas2d.canvas_zoom *= 2.;
                    }
                    canvas2d.canvas_zoom *= 0.8
                }
                _ => {}
            }
        }

        canvas2d.draw_axis_s2(0.5, 0);

        fluid_sim.update();


        T+= 1;
        // println!("{}", T);


        canvas2d.draw_fluid_density(&grid_stats, &fluid_sim, MAX_DENSITY, 0);
        canvas2d.draw_fluid_velocity(&grid_stats, &fluid_sim, 0.1, 4, 0);

        canvas2d
            .canvas
            .set_draw_color(Color::RGBA(70, 234, 255, 255));

        canvas2d.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

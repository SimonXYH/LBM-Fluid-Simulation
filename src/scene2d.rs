use crate::vec2::Vec2;
#[derive(Copy, Clone)]
pub struct Scene2D{
    pub scene_zoom: f32,
    pub scene_center: Vec2<f32>
}

impl Scene2D{
    pub fn new(scene_zoom: f32, scene_center: Vec2<f32>) -> Self{
        Scene2D{
            scene_zoom,
            scene_center
        }
    }
    
    pub fn project(&self, point: Vec2<f32>) -> Vec2<f32>{
        point * self.scene_zoom + self.scene_center
    }
}
use macroquad::prelude::*;

const ZOOM: f32 = 8.0;

pub fn default_cam() -> Camera2D {
    Camera2D {
        rotation: 0.0,
        target: vec2(0.0, 0.0),
        zoom: vec2(1.0, -screen_width() / screen_height()) / ZOOM,
        ..Default::default()
    }
}

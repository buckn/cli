use common::pos::Pos;
use hecs::Entity;
use hecs::World;
use macroquad::prelude::*;

const ZOOM: f32 = 100.0;
const CAMERA_LERP_CONSTANT: f32 = 0.9;

pub fn default_cam() -> Camera2D {
    Camera2D {
        rotation: 0.0,
        target: vec2(0.0, 0.0),
        zoom: vec2(1.0, -screen_width() / screen_height()) / ZOOM,
        ..Default::default()
    }
}

pub fn player_cam(world: &World, player: Entity) -> Camera2D {
    Camera2D {
        rotation: 0.0,
        target: world.get::<Pos>(player).unwrap().vec.clone(),
        zoom: vec2(1.0, -screen_width() / screen_height()) / ZOOM,
        ..Default::default()
    }
}

pub fn update_player_cam(world: &World, player: Entity, camera: &mut Camera2D) {
    camera.target = world.get::<Pos>(player).unwrap().vec.clone().lerp(camera.target, CAMERA_LERP_CONSTANT);
}

use hecs::Entity;
use macroquad::prelude::draw_text;
use macroquad::prelude::get_fps;
use macroquad::prelude::Camera2D;
use macroquad::prelude::Color;

pub enum CurrentScreen {
    MAIN,
    SETTINGS,
    GAME,
    LOADING,
    CONNECTING,
}

pub struct GlobalState {
    pub current_screen: CurrentScreen,
    pub fps_counter: bool,
    pub camera: Camera2D,
    pub my_player: Entity,
}

impl GlobalState {
    pub fn fps_counter(&self) {
        if self.fps_counter {
            draw_text(
                &("fps: ".to_owned() + get_fps().to_string().as_str()),
                self.camera.target.x,
                self.camera.target.y,
                24.0,
                Color::new(0.0, 0.0, 0.0, 255.0),
            );
        }
    }
}

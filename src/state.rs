enum CurrentScreen {
    MAIN,
    SETTINGS,
    GAME,
    LOADING,
    CONNECTING,
}

struct GlobalState {
    current_screen: CurrentScreen,
    fps_counter: bool,
    camera: Camera2D,
    current_game: GameInfo,
}

impl GlobalState {
	pub fn fps_counter(&self) {
		if self.fps_counter {
			draw_text(
                &("fps: ".to_owned() + get_fps().to_string().as_str()),
                0.0,
                0.0,
                24.0,
                Color::new(0.0, 0.0, 0.0, 255.0),
            );
		}
	}
}
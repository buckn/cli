mod net;

mod cam;

mod menu;

mod state;

use crate::cam::player_cam;
use crate::cam::update_player_cam;
use crate::state::CurrentScreen;
use crate::state::GlobalState;
use common::inv::item::knife::Knife;
use common::map::player_spawn::PlayerSpawn;
use common::player::*;
use common::pos::Pos;
use common::sys::control::Cntrl;
use common::sys::*;
use hecs::*;
use macroquad::prelude::*;
use std::{sync::mpsc, thread};

#[macroquad::main("FFA")]
async fn main() {
    let mut world = World::new();

    let (sender, _reciever) = mpsc::channel();

    thread::spawn(|| net::start(sender));

    let mut state = GlobalState {
        current_screen: CurrentScreen::GAME,
        fps_counter: true,
        camera: cam::default_cam(),
        my_player: ecs_spawn_default(&mut world),
    };

    PlayerSpawn::ecs_spawn_default(&mut world);

    Knife::ecs_spawn_default(&mut world);

    state.camera = player_cam(&world, state.my_player);

    loop {
        clear_background(WHITE);

        systems(
            &mut world,
            Cntrl::new(),
            state.my_player.id(),
            Pos::mouse_pos(&state.camera),
        );

        update_player_cam(&world, state.my_player, &mut state.camera);

        state.fps_counter();

        //update_player_cam(&world, state.my_player, &mut state.camera);

        set_camera(state.camera);

        next_frame().await
    }
}

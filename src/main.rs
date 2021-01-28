mod net;

mod cam;

mod menu;

use common::inv::item::player_spawn::PlayerSpawn;
use common::inv::item::knife::Knife;
use common::inv::item::ItemType;
use common::game::*;
use common::inv::*;
use common::player::Player;
use common::systems::control::Cntrl;
use common::systems::*;
use common::*;
use hecs::*;
use macroquad::prelude::*;
use net::*;
use std::{sync::mpsc, thread};

#[macroquad::main("FFA")]
async fn main() {
    let mut world = World::new();

    let (sender, reciever) = mpsc::channel();

    thread::spawn(|| net::start(sender));

    let mut state = GlobalState {
        current_screen: CurrentScreen::GAME,
        fps_counter: true,
        camera: cam::default_cam(),
        current_game: GameInfo::default_sandbox(),
    };

    //make player spawn

    world.spawn((common::inv::item::Item::default(ItemType::PLAYERSPAWN(PlayerSpawn::default())), common::module::Mod::blank()));

    //spawn player
    world.spawn((
        state.current_game.my_player,
        Player::new(&mut state.current_game.player_ct),
        Inv::new(10),
    ));

    //spawn test item
    world.spawn((common::inv::item::Item::default(ItemType::KNIFE(Knife::default())), common::module::Mod::blank()));

    loop {
        clear_background(WHITE);

        systems(&mut world, Cntrl::new(), &state.current_game.my_player);

        state.fps_counter();

        set_camera(state.camera);

        next_frame().await
    }
}
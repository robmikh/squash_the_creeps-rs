mod main_scene;
mod mob;
mod player;

use gdnative::prelude::*;
use main_scene::Main;
use mob::Mob;
use player::Player;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Mob>();
    handle.add_class::<Main>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);

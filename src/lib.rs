mod player;

use gdnative::prelude::*;
use player::Player;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
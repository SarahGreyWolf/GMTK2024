use godot::prelude::*;

mod classes;

const GRAVITY: f32 = 1000.0;

struct GMTKExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GMTKExtension {}

use godot::classes::INode;
use godot::classes::Node;
use godot::prelude::*;

struct GMTKExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GMTKExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct Player {
    base: Base<Node>,
}

#[godot_api]
impl INode for Player {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Hello, World!");

        Self { base }
    }
}
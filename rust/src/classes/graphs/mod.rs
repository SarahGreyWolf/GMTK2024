use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

pub mod bar;
mod point;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Graph {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Graph {
    fn init(base: Base<Node2D>) -> Self { Self { base } }

    fn ready(&mut self) {}
}

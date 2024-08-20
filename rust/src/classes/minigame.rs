use std::u32;

use godot::classes::{
    Area2D, IArea2D, INode2D, InputEvent, InputEventMouseButton, Node2D,
    SceneTree,
};
use godot::global::MouseButton;
use godot::prelude::*;

use super::graphs::bar::BarGraph;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct MiniGame {
    base: Base<Node2D>,
    #[export]
    limited: bool,
    #[export]
    available: u32,
}

#[godot_api]
impl INode2D for MiniGame {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            limited: false,
            available: 0,
        }
    }

    fn ready(&mut self) {}
}

#[derive(GodotClass)]
#[class(base=Area2D)]
struct BarController {
    base: Base<Area2D>,
    #[export]
    bar_index: i32,
    #[export]
    count: u32,
    #[export]
    minimum: u32,
    #[export]
    maximum: u32,
}

#[godot_api]
impl IArea2D for BarController {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            bar_index: -1,
            count: 0,
            minimum: 0,
            maximum: u32::MAX,
        }
    }

    fn ready(&mut self) {}

    fn process(&mut self, _delta: f64) { self.set_bar_height(self.count); }
}

#[godot_api]
impl BarController {
    #[func]
    fn input_event(
        &mut self,
        _viewport: Gd<Node>,
        event: Gd<InputEvent>,
        _idx: u32,
    ) {
        let Some(root) = self.base().get_parent() else {
            godot_error!("Could not get parent");
            return;
        };
        let Ok(mut minigame) = root.try_cast::<MiniGame>() else {
            godot_error!("Could not cast parent to MiniGame");
            return;
        };
        let limited: bool = minigame.call("get_limited".into(), &[]).to();
        let available: u32 = minigame.call("get_available".into(), &[]).to();

        if let Ok(mouse) = event.try_cast::<InputEventMouseButton>() {
            if mouse.is_pressed() {
                if mouse.get_button_index() == MouseButton::LEFT {
                    if self.count >= self.maximum {
                        self.count = self.maximum;
                        return;
                    }
                    if limited {
                        if available > 0 {
                            minigame.call(
                                "set_available".into(),
                                &[Variant::from(available - 1)],
                            );
                            self.count += 1;
                        }
                    } else {
                        self.count += 1;
                    }
                } else {
                    if self.count <= self.minimum {
                        self.count = self.minimum;
                        return;
                    }
                    if limited {
                        minigame.call(
                            "set_available".into(),
                            &[Variant::from(available - 1)],
                        );
                        self.count -= 1;
                    } else {
                        self.count -= 1;
                    }
                }
            }
        }
    }

    #[func]
    fn set_bar_height(&mut self, height: u32) {
        if self.bar_index < 0 {
            godot_error!("No Bar was set!");
            return;
        }
        let Some(tree) = self.base().get_tree() else {
            godot_error!("Failed to get Tree");
            return;
        };

        let Some(root) = tree.get_root() else {
            godot_error!("Failed to get Root");
            return;
        };

        let mut bar_graph =
            root.get_node_as::<BarGraph>("Node2D/Graph/BarGraph");

        bar_graph.call(
            "set_bar_height".into(),
            &[Variant::from(self.bar_index), Variant::from(height)],
        );
    }
}

use godot::{
    classes::{Area2D, IArea2D, PackedScene},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Door {
    base: Base<Area2D>,
    #[export]
    dest_scene: Option<Gd<PackedScene>>,
    #[export]
    locked: bool,
}

#[godot_api]
impl IArea2D for Door {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            dest_scene: None,
            locked: false,
        }
    }
}

#[godot_api]
impl Door {
    #[func]
    fn enter_door(&mut self) {
        let Some(mut tree) = self.base_mut().get_tree() else {
            godot_error!("Could not get scene tree!");
            return;
        };

        let Some(ref scene) = self.dest_scene else {
            godot_error!("No scene was set for this door!");
            return;
        };

        tree.change_scene_to_packed(scene);
    }
}

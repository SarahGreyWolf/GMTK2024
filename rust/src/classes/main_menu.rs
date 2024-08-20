use godot::classes::{
    Control, IControl, InputEvent, InputEventMouseButton, ResourceLoader,
};
use godot::global::MouseButton;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
struct MainMenu {
    base: Base<Control>,
    first_level: Option<Gd<PackedScene>>,
    #[export]
    credits: Option<Gd<PackedScene>>,
}

#[godot_api]
impl IControl for MainMenu {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            first_level: None,
            credits: None,
        }
    }

    fn ready(&mut self) {
        let mut resource_loader = ResourceLoader::singleton();
        let Some(level1) =
            resource_loader.load("res://levels/level1.tscn".into())
        else {
            godot_error!("Failed to load level1");
            return;
        };
        let Ok(level1) = level1.try_cast::<PackedScene>() else {
            godot_error!("Failed to cast level1 as a PackedScene");
            return;
        };
        self.first_level = Some(level1);

        let Some(credits) =
            resource_loader.load("res://scenes/credits.tscn".into())
        else {
            godot_error!("Failed to load credits");
            return;
        };
        let Ok(credits) = credits.try_cast::<PackedScene>() else {
            godot_error!("Failed to cast credits as a PackedScene");
            return;
        };
        self.credits = Some(credits);
    }
}

#[godot_api]
impl MainMenu {
    #[func]
    fn start_game(&mut self) {
        let Some(mut tree) = self.base_mut().get_tree() else {
            godot_error!("Could not get scene tree!");
            return;
        };

        let Some(ref scene) = self.first_level else {
            godot_error!("No scene was set for this button!");
            return;
        };

        tree.change_scene_to_packed(scene);
    }

    #[func]
    fn credits(&mut self) {
        let Some(mut tree) = self.base_mut().get_tree() else {
            godot_error!("Could not get scene tree!");
            return;
        };

        let Some(ref scene) = self.credits else {
            godot_error!("No scene was set for this button!");
            return;
        };

        tree.change_scene_to_packed(scene);
    }

    #[func]
    fn exit(&mut self) { self.base().get_tree().unwrap().quit(); }
}

#[derive(GodotClass)]
#[class(base=Control)]
struct Credits {
    base: Base<Control>,
    main_menu: Option<Gd<PackedScene>>,
}

#[godot_api]
impl IControl for Credits {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            main_menu: None,
        }
    }

    fn ready(&mut self) {
        let mut resource_loader = ResourceLoader::singleton();
        let Some(main_menu) =
            resource_loader.load("res://scenes/MainMenu.tscn".into())
        else {
            godot_error!("Failed to load credits");
            return;
        };
        let Ok(main_menu) = main_menu.try_cast::<PackedScene>() else {
            godot_error!("Failed to cast credits as a PackedScene");
            return;
        };
        self.main_menu = Some(main_menu);
    }
}

#[godot_api]
impl Credits {
    #[func]
    fn main_menu(&mut self) {
        let Some(mut tree) = self.base_mut().get_tree() else {
            godot_error!("Could not get scene tree!");
            return;
        };

        let Some(ref scene) = self.main_menu else {
            godot_error!("No scene was set for this button!");
            return;
        };

        tree.change_scene_to_packed(scene);
    }
}

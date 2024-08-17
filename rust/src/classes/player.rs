use godot::classes::{
    self, AnimationPlayer, CharacterBody2D, ICharacterBody2D, InputEvent,
};
use godot::prelude::*;

use crate::GRAVITY;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,
    grounded: bool,
    #[export]
    jump_speed: f32,
    #[export]
    move_speed: f32,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, World!");

        Self {
            base,
            grounded: false,
            jump_speed: -500.,
            move_speed: 500.,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut animation_player =
            self.base_mut().get_node_as::<AnimationPlayer>(
                "CollisionShape2D/Sprite2D/AnimationPlayer",
            );

        godot_print!("{:?}", self.grounded);

        if !self.grounded {
            if animation_player.get_current_animation() != "Jump".into() {
                animation_player.set_current_animation("Jump".into());
            }
        } else if animation_player.get_current_animation() == "Jump".into() {
            animation_player.set_current_animation("Idle".into());
        }
        // Failsafe: Sometimes it sets the animation to "" for some reason
        if animation_player.get_current_animation().is_empty() {
            animation_player.set_current_animation("Idle".into());
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let velocity = self.base().get_velocity();
        if !self.grounded {
            self.base_mut().set_velocity(
                velocity + Vector2::new(0., GRAVITY * real::from_f64(delta)),
            );
        }

        self.handle_input();

        let collided = self.base_mut().move_and_slide();

        self.grounded = collided;
    }

    fn input(&mut self, event: Gd<InputEvent>) {}
}

impl Player {
    fn handle_input(&mut self) {
        let input = classes::Input::singleton();

        let transform = self.base().get_transform();
        let scale = self.base().get_scale();

        let mut velocity = self.base().get_velocity();

        velocity.x = input.get_axis("MoveLeft".into(), "MoveRight".into())
            * self.move_speed;

        let left = input.is_action_pressed("MoveLeft".into());
        let right = input.is_action_pressed("MoveRight".into());
        let jump = input.is_action_pressed("Jump".into());

        if left && transform.scale().y > 0. {
            self.base_mut()
                .set_scale(Vector2::new(scale.x * -1., scale.y));
        }

        if right && transform.scale().y < 0. {
            self.base_mut()
                .set_scale(Vector2::new(scale.x * -1., scale.y));
        }

        if jump && self.grounded {
            velocity.y = self.jump_speed;
        }

        self.base_mut().set_velocity(velocity);
    }
}

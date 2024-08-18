use godot::classes::{
    AnimationPlayer, CharacterBody2D, ICharacterBody2D, Input, InputEvent,
};
use godot::prelude::*;

use crate::GRAVITY;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,
    #[export]
    jump_speed: f32,
    #[export]
    move_speed: f32,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            jump_speed: -500.,
            move_speed: 500.,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut animation_player =
            self.base_mut().get_node_as::<AnimationPlayer>(
                "CollisionShape2D/Sprite2D/AnimationPlayer",
            );

        if !self.base_mut().is_on_floor() {
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
        if !self.base_mut().is_on_floor() {
            self.base_mut().set_velocity(
                velocity + Vector2::new(0., GRAVITY * real::from_f64(delta)),
            );
        }

        self.handle_input();

        let collided = self.base_mut().move_and_slide();
    }

    fn ready(&mut self) {}
}

#[godot_api]
impl Player {
    #[signal]
    fn game_over();

    #[func]
    fn kill(mut player: Gd<Node>) {
        player.emit_signal("game_over".into(), &[]);
        player.queue_free();
    }

    fn handle_input(&mut self) {
        let input = Input::singleton();

        let transform = self.base().get_transform();
        let scale = self.base().get_scale();

        let mut velocity = self.base().get_velocity();

        velocity.x = input.get_axis("MoveLeft".into(), "MoveRight".into())
            * self.move_speed;

        let left = input.is_action_pressed("MoveLeft".into());
        let right = input.is_action_pressed("MoveRight".into());
        let jump = input.is_action_pressed("Jump".into());

        if (left && transform.scale().y > 0.) && !right {
            self.base_mut()
                .set_scale(Vector2::new(scale.x * -1., scale.y));
        }

        if (right && transform.scale().y < 0.) && !left {
            self.base_mut()
                .set_scale(Vector2::new(scale.x * -1., scale.y));
        }

        if jump && self.base_mut().is_on_floor() {
            velocity.y = self.jump_speed;
        }

        self.base_mut().set_velocity(velocity);
    }
}

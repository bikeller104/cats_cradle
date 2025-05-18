use godot::prelude::*;
use godot::global::*;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f32,
    angular_speed: f64,
    base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, World!");

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut velocity = Vector2{x:0.0, y:0.0};
        if Input::is_action_pressed(&Input::singleton(), "move_left") {
            velocity.x = -1.0 *self.speed; 
            godot_print!("I am moving left");
            godot_print!("my position is {}", self.base_mut().get_position())
        }
        else if Input::is_action_pressed(&Input::singleton(), "move_right"){
            velocity.x = self.speed;
            godot_print!("I am moving right");
            godot_print!("my position is {}", self.base_mut().get_position())
        } else {
            velocity.x = 0.0;
        }
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}


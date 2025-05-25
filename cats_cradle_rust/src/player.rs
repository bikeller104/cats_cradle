
use godot::classes::AnimatedSprite2D;
use godot::prelude::*;
use godot::global::*;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::Timer;


use godot::classes::Label;

pub const GRAVITY: f32 = 1000.0;
pub const FALL_GRAVITY: f32 = 1500.0;
pub const FAST_FALL_GRAVITY: f32 = 4000.0;
pub const WALL_GRAVITY : f32 = 10.0;

pub const JUMP_VELOCITY: f32 = -300.0;
pub const WALL_JUMP_VELOCITY: f32 = -300.0;
pub const WALL_JUMP_PUSHBACK: f32 = 300.0;

pub const INPUT_BUFFER_TIME: f64 = 0.1;
pub const JUMP_TIME:f64 = 0.05;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f32,
    gravity: f32,
    input_buffer: Gd<Timer>,
    jump_timer: Gd<Timer>,
    jump_avaliable: bool,
    base: Base<CharacterBody2D>
}


/*
    if i use self.base_mut(), I can retreive the godot player object, 
    but if I use self.base.to_gd() I can retreive the godot player 
    object and intellisense works with it.
*/


#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        //godot_print!("Hello, World!");

        let mut inst = Self {
            speed: 200.0,
            gravity: GRAVITY,
            input_buffer: Timer::new_alloc(),
            jump_timer: Timer::new_alloc(),
            jump_avaliable: true,
            base,
        };

        inst.addTimers();

        inst

    }

    fn ready(&mut self) {


    }

    fn process(&mut self, delta: f64) {


        let mut velocity = Vector2{x:0.0, y:0.0};
        let mut sprite = self.base.to_gd().get_node_as::<AnimatedSprite2D>("Sprite2D");
        velocity = self.base_mut().get_velocity();

        let input = Input::singleton();
        //let direction: f32 = Input::get_axis(&input, "move_left", "move_right");
        let moving_left: bool = Input::is_action_pressed(&Input::singleton(), "move_left") ;
        let moving_right:bool = Input::is_action_pressed(&Input::singleton(), "move_right") ;
        
        if moving_left{
            sprite.set_flip_h(true);
        }
        if moving_right{
            sprite.set_flip_h(false);
        }

        
        if moving_left {
            velocity.x = -1.0 *self.speed; 
            sprite.set_animation("walk");
            //godot_print!("I am moving left");
            //godot_print!("my position is {}", self.base_mut().get_position())
        }
        else if moving_right {
            velocity.x = self.speed;
            sprite.set_animation("walk");
            
            //godot_print!("I am moving right");
            //godot_print!("my position is {}", self.base_mut().get_position())
        } else {
            velocity.x = 0.0;
            sprite.set_animation("rest");
        }


        let jump_attempted: bool = Input::is_action_just_pressed(&input, "jump");
        
            // Jump handling
            if jump_attempted || self.input_buffer.get_time_left() > 0.0 {
                // jump if jump is avaliable it was either pressed or pressed in the last 0.1 seconds (INPUT_BUFFER_TIME)
                if self.jump_avaliable {
                    velocity.y = JUMP_VELOCITY;
                    self.jump_avaliable = false;
                    sprite.set_animation("jump");
                }
                //if the character is on a wall, regardless of if jump is avaliable, jump 
                //unless the charecter does not have directional inputs
                else if self.base_mut().is_on_wall() && (moving_left || moving_right) {
                    velocity.y = WALL_JUMP_VELOCITY;
                    //TO DO: do we want the character to jump away from the wall or in 
                    //the direction of the input which could be into the wall
                    velocity.x = WALL_JUMP_PUSHBACK; //* (direction / direction.abs());
                }
                //if jump was attempted but is not avaliable start the timer
                //and if it becomes avaliable, then it will jump on a later loop
                else if jump_attempted {
                    
                self.input_buffer.start();
                }
                else{
                    //do nothing
                }
            }
            //if the player releases the jump key, reduce upward velocity to 
            //jump lower than if they held the key.
            if input.is_action_just_released("jump"){
                velocity.y = JUMP_VELOCITY / 4.0
                }
                
            if self.base.to_gd().is_on_floor() {
                self.jump_avaliable = true;
                self.jump_timer.stop();
                }
            //if the player character is not on the floor, but a jump action
            //is avaliable, start the timer and if the player presses jump
            //while the timer is going, the character will jump even if 
            //not on the ground until 0.05 Seconds (JUMP_TIMER) has passed
            else {
                if self.jump_avaliable{
                    if self.jump_timer.is_stopped(){
                        self.jump_timer.start();
                    }
                }
                velocity.y += self.gravity * (delta as f32);
            }





        sprite.play();
        self.base_mut().set_velocity(velocity);
        //godot_print!("x:{0}\ny:{1}", velocity.x, velocity.y);
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Player{
    pub fn addTimers(&mut self)
    {
        self.input_buffer.set_wait_time(INPUT_BUFFER_TIME);
        self.input_buffer.set_one_shot(true);
        //self.base_mut().add_child(self.input_buffer);
        let mut timer = Timer::new_alloc();
        let mut label = Label::new_alloc();
        self.base.to_gd().add_child(&timer.upcast::<Node>());
        self.base.to_gd().add_child(&label.upcast::<Node>());
    }
}

pub fn print_test()
{
    println!("hello from player.rs")
}

use godot::prelude::*;
use crate::game::Game;

use godot::classes::Control;
use godot::classes::IControl;


#[derive(GodotClass)]
#[class(base=Control)]
struct MainMenu {
    base: Base<Control>
}

#[godot_api]
impl IControl for MainMenu {
    fn init(base: Base<Control>) -> Self {
        godot_print!("Initializing");
        
        Self{
            base,
        }
    }
}

#[godot_api]
impl MainMenu {
    #[func]
    pub fn start_game(&mut self) {
        let game = match self.base().get_parent() {
            Some(p) => p,
            None => {
                godot_error!("Unable to find parent game Node");
                return;
            },
        };
        let mut node:Gd<Game> = match game.try_cast::<Game>(){
            Err(_) => {
                godot_error!("Unable to cast node to Game during Start Game");
                return;
            }
            Ok(n) => n,
        };

        node.bind_mut().load_level();
    }
}
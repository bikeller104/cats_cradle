use godot::prelude::*;
use godot::{classes::{INode2D, Node2D}, global::godot_error};
use crate::game_constants::MAX_LEVEL;
use crate::player::Player;


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Game {
    base: Base<Node2D>,
    main_scene: Gd<PackedScene>,
    scene_list: Vec<Gd<PackedScene>>,
    current_level: usize,
    //player_character: Gd<Player>,
}

#[godot_api]
impl INode2D for Game {
    fn init(base: Base<Node2D>) -> Self {
        let main_scene = load("res://MainMenu.tscn");
        //let player_node = load("res://Character.tscn");
        
        Self{
            base,
            main_scene,
            scene_list: Vec::new(),
            current_level: 0,
            //player_character: player_node,
        }
    }

    fn ready(&mut self){
        let mut levels: Vec<Gd<PackedScene>> = Vec::new();
        for i in 1..=MAX_LEVEL {
            let level_path = format!("res://level_{i}.tscn");
            let level_scene = load(&level_path);
            
            levels.push(level_scene);
        }
        self.scene_list = levels;

        self.main_menu();


    }
}

#[godot_api]
impl Game {
    #[func]
    pub fn main_menu(&mut self){
        self.load_scene(self.main_scene.clone());
    }

    pub fn load_level(&mut self) {
        self.load_scene(self.scene_list[self.current_level].clone());
        self.current_level += 1;
    }

    fn load_scene(&mut self, scene: Gd<PackedScene>){
        let instance: Gd<Node> = match scene.instantiate(){
            Some(instance) => instance,
            None => {
                godot_error!("Unable to find the main menu resource");
                return;
            }
        };
        let mut entry = self.base_mut();
        entry.get_children().iter_shared().for_each(|c| {
            entry.remove_child(&c);
        });
        entry.add_child(&instance);
    }

}


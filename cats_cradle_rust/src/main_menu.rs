use godot::classes::MarginContainer;
use godot::classes::VBoxContainer;
use godot::prelude::*;
use godot::global::*;

use godot::classes::Control;
use godot::classes::IControl;
use godot::classes::Button;

#[derive(GodotClass)]
#[class(base=Control)]
struct MainMenu_rust {
    base: Base<Control>
}

#[godot_api]
impl IControl for MainMenu_rust {
    fn init(base: Base<Control>) -> Self {
        godot_print!("Initializing");
        
        Self{
            base,
        }
    }

    fn ready(&mut self){
        godot_print!("ready");

        let callable = &self.base().callable("print_start");
        godot_print!("obtained Event Handler");

        let margin_container_1 = self.base.to_gd().get_node_as::<MarginContainer>("MarginContainer");
        godot_print!("found margin container");
        let button_vbox = margin_container_1.get_node_as::<VBoxContainer>("VBoxContainer");
        godot_print!("found vbox");
        //let mut button = button_vbox.get_node_as::<Button>("StartButton");
        let mut button = self.base.to_gd().get_node_as::<Button>("StartButton");
        godot_print!("found button and connecting event handler for pressed event");
        //let mut options = button_vbox.get_node_as::<Button>("OptionsButton");
        //button_vbox.remove_child(options.);
        button.connect("pressed", callable);
        godot_print!("print_start added to StartButton");
/*
let children = self.base.to_gd().get_children();
for child in children.iter_shared() {
    let name = child.get_name();
    godot_print!("{:?}", name);
}
*/
        /*
        let mut button = self.base.to_gd().get_node_as::<Button>("StartButton").ok(){
            
    }else{
        godot_print!("Button not found");
    }
    */

    }
}

#[godot_api]
impl MainMenu_rust {
    #[func]
    fn print_start(&mut self) {
        godot_print!("start button was pressed")
    }
}
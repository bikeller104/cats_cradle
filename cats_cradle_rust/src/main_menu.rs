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
        let children = self.base.to_gd().get_children();
        for child in children.iter_shared() {
            let name = child.get_name();
            godot_print!("{:?}", name);
        }
        /*
        let callable = &self.base().callable("print_start");
        godot_print!("obtained Event Handler");
        let mut button = self.base.to_gd().get_node_as::<Button>("StartButton").ok(){
            
        godot_print!("found button and connecting event handler for pressed event");
        button.connect("pressed", callable);
        godot_print!("print_start added to StartButton");
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
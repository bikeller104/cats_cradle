use godot::prelude::*;

mod player;
pub mod game_constants;

mod main_menu;

struct CatsCradleExtension;


#[gdextension]
unsafe impl ExtensionLibrary for CatsCradleExtension {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player(){
        player::print_test();
    }
}
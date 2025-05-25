use godot::prelude::*;

mod player;

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
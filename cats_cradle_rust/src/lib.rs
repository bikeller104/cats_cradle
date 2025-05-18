use godot::prelude::*;

mod player;

struct CatsCradleExtension;


#[gdextension]
unsafe impl ExtensionLibrary for CatsCradleExtension {}
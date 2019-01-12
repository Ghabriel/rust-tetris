use sfml::graphics::{Sprite, Texture};

pub struct GameAssets {
    tile_texture: Texture,
}

impl GameAssets {
    pub fn new() -> GameAssets {
        let tile_texture = Texture::from_file("resources/tiles.png").unwrap();

        GameAssets {
            tile_texture
        }
    }

    pub fn make_tile_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.tile_texture)
    }
}

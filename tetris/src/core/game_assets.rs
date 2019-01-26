use sfml::graphics::{Sprite, Texture};

pub struct GameAssets {
    block_texture: Texture,
}

impl GameAssets {
    pub fn new() -> GameAssets {
        let block_texture = Texture::from_file("resources/blocks.png").unwrap();

        GameAssets {
            block_texture
        }
    }

    pub fn make_block_sprite(&self) -> Sprite {
        Sprite::with_texture(&self.block_texture)
    }
}

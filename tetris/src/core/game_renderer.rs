use super::{GameAssets, Model, View};
use super::traits::Render;

pub struct GameRenderer {
    assets: GameAssets,
    view: View,
}

impl Render for GameRenderer {
    type Target = Model;

    fn render(&mut self, model: &Model) -> bool {
        self.view.render(model, &mut self.assets)
    }
}

impl GameRenderer {
    pub fn new(
        width: u32,
        height: u32,
        title: &str
    ) -> GameRenderer {
        GameRenderer {
            assets: GameAssets::new(),
            view: View::new(width, height, title),
        }
    }
}

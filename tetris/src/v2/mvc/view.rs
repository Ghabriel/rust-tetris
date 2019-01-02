use sfml::graphics::{IntRect, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::window::{Event, Style};
use super::Model;
use super::traits::Render;

pub struct View {
    window: RenderWindow,
    tiles_texture: Texture,
}

impl Render for View {
    type Target = Model;

    fn render(&mut self, model: &Model) -> bool {
        if self.handle_events() {
            return true;
        }

        self.window.set_active(true);

        self.render_board(model);
        self.render_active_piece(model);

        self.window.display();

        false
    }
}

impl View {
    pub fn new(
        width: u32,
        height: u32,
        title: &str
    ) -> View {
        let window = RenderWindow::new(
            (width, height),
            title,
            Style::CLOSE,
            &Default::default()
        );

        let tiles_texture = Texture::from_file("resources/tiles.png").unwrap();

        View {
            window,
            tiles_texture,
        }
    }

    pub fn handle_events(&mut self) -> bool {
        while let Some(event) = self.window.poll_event() {
            if event == Event::Closed {
                self.window.close();
                return true;
            }
        }

        false
    }

    pub fn render_board(&mut self, model: &Model) {
        let mut tile_sprite = Sprite::with_texture(&self.tiles_texture);

        let coordinates = IntRect::new(0, 0, 18, 18);
        tile_sprite.set_texture_rect(&coordinates);

        tile_sprite.set_position((0., 0.));
        self.window.draw(&tile_sprite);

        tile_sprite.set_position((18., 0.));
        self.window.draw(&tile_sprite);

        tile_sprite.set_position((36., 0.));
        self.window.draw(&tile_sprite);

        tile_sprite.set_position((54., 0.));
        self.window.draw(&tile_sprite);

        // let mut num_rows = 0;

        // model.for_each_row(&mut |row| {
        //     num_rows += 1;

        //     row.iter().for_each(|cell| {
        //         println!("Cell: {:?}", cell);
        //     });
        // });

        // println!("Num rows: {}", num_rows);
        // println!("-----------------------------");
    }

    pub fn render_active_piece(&mut self, model: &Model) {
        // TODO
    }
}

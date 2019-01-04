use sfml::graphics::{IntRect, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::window::{Event, Style};
use super::Model;
use super::traits::Render;

const TILE_SIZE: usize = 18;
const TILE_SCALING: f32 = 1.5;

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

        self.render_board(model, (100, 20));
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

    pub fn render_board(&mut self, model: &Model, position: (i32, i32)) {
        let window = &mut self.window;
        let mut row_index = 0;

        let mut tile_sprite = Sprite::with_texture(&self.tiles_texture);
        tile_sprite.scale((TILE_SCALING, TILE_SCALING));

        model.for_each_row(&mut |row| {
            row.iter()
                .enumerate()
                .for_each(|(cell_index, cell)| {
                    // if let Some(block) = cell {
                        let tileset_coordinates = IntRect::new(0, 0, 18, 18);
                        tile_sprite.set_texture_rect(&tileset_coordinates);

                        let cell_coordinates = get_cell_coordinates(row_index, cell_index);
                        tile_sprite.set_position(
                            (
                                cell_coordinates.0 + position.0 as f32,
                                cell_coordinates.1 + position.1 as f32,
                            )
                        );

                        window.draw(&tile_sprite);
                    // }
                });

            row_index += 1;
        });
    }

    pub fn render_active_piece(&mut self, model: &Model) {
        // TODO
    }
}

fn get_cell_coordinates(row: usize, column: usize) -> (f32, f32) {
    let x = (column * TILE_SIZE) as f32 * TILE_SCALING;
    let y = (row * TILE_SIZE) as f32 * TILE_SCALING;

    (x, y)
}

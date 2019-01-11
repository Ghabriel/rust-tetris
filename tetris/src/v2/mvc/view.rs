use sfml::graphics::{IntRect, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::window::{Event, Style};
use super::super::board::helpers;
use super::super::piece::{Piece, PieceColor};
use super::super::position::{BoardPosition, PiecePosition, WindowPosition};
use super::super::rotations::RotationSystem;
use super::model::CurrentPiece;
use super::Model;
use super::traits::Render;

const TILE_SIZE: usize = 18;
const TILE_SCALING: f32 = 1.5;

pub struct View {
    window: RenderWindow,
    board_view_position: WindowPosition,
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

        let board_view_position = WindowPosition::new(100., 20.);

        let tiles_texture = Texture::from_file("resources/tiles.png").unwrap();

        View {
            window,
            board_view_position,
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
        let mut tile_sprite = make_tile_sprite(&self.tiles_texture);
        let mut row_index = 0;

        model.for_each_row(&mut |row| {
            row.iter()
                .enumerate()
                .for_each(|(cell_index, cell)| {
                    if let Some(block) = cell {
                        let tileset_coordinates = IntRect::new(0, 0, 18, 18);
                        tile_sprite.set_texture_rect(&tileset_coordinates);

                        let cell_coordinates = BoardPosition::new(row_index, cell_index);

                        self.draw_tile(&cell_coordinates, &mut tile_sprite);
                    }
                });

            row_index += 1;
        });
    }

    pub fn render_active_piece(&mut self, model: &Model) {
        if let Some(active_piece) = model.get_active_piece() {
            let CurrentPiece { piece, position } = active_piece;
            let piece_position = BoardPosition::from_index(
                *position,
                model.get_board_num_columns()
            );

            self.render_piece(model, piece, &piece_position);
        }
    }

    pub fn render_piece(
        &mut self,
        model: &Model,
        piece: &Piece,
        piece_position: &BoardPosition,
    ) {
        let piece_color = piece.get_color();
        let grid = piece.get_grid(model.get_rotation_system());
        let grid_size = grid.0.len();
        let grid_num_columns = (grid_size as f32).sqrt() as usize;
        let mut tile_sprite = make_tile_sprite(&self.tiles_texture);

        grid.0.iter()
            .enumerate()
            .filter(|(_, cell)| **cell)
            .for_each(|(cell_index, _)| {
                let block_in_piece_coordinates = PiecePosition::from_index(
                    cell_index,
                    grid_num_columns
                );

                let block_position = BoardPosition::new(
                    block_in_piece_coordinates.get_row() + piece_position.get_row(),
                    block_in_piece_coordinates.get_column() + piece_position.get_column(),
                );

                let tileset_coordinates = IntRect::new(0, 0, 18, 18);
                tile_sprite.set_texture_rect(&tileset_coordinates);

                self.draw_tile(&block_position, &mut tile_sprite);
            });
    }

    fn draw_tile(&mut self, tile_position: &BoardPosition, sprite: &mut Sprite) {
        let tile_window_position = self.to_window_coordinates(&tile_position);
        let target_position = tile_window_position + &self.board_view_position;

        sprite.set_position(target_position.as_xy());
        self.window.draw(sprite);
    }

    fn to_window_coordinates(&self, board_position: &BoardPosition) -> WindowPosition {
        let row = TILE_SCALING * (board_position.get_row() * TILE_SIZE) as f32;
        let column = TILE_SCALING * (board_position.get_column() * TILE_SIZE) as f32;

        WindowPosition::new(row, column)
    }
}

fn make_tile_sprite(tile_texture: &Texture) -> Sprite {
    let mut tile_sprite = Sprite::with_texture(tile_texture);
    tile_sprite.scale((TILE_SCALING, TILE_SCALING));

    tile_sprite
}

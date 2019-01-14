use sfml::graphics::{Color, IntRect, RenderTarget, RenderWindow, Sprite, Transformable};
use sfml::window::{Event, Style};
use super::super::piece::{Piece, PieceColor};
use super::super::position::{BoardPosition, PiecePosition, WindowPosition};
use super::model::CurrentPiece;
use super::{GameAssets, Model};

const TILE_SIZE: usize = 18;
const TILE_SCALING: f32 = 1.5;

pub struct View {
    window: RenderWindow,
    board_view_position: WindowPosition,
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

        let board_view_position = WindowPosition::new(20., 10.);

        View {
            window,
            board_view_position,
        }
    }

    pub fn render(&mut self, model: &Model, assets: &mut GameAssets) -> bool {
        if self.handle_events() {
            return true;
        }

        self.window.clear(&Color::BLACK);

        self.render_board(model, assets);
        self.render_active_piece(model, assets);

        self.window.display();

        false
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

    pub fn render_board(&mut self, model: &Model, assets: &mut GameAssets) {
        let mut tile_sprite = make_tile_sprite(assets);
        let mut row_index = 0;

        model.for_each_row(&mut |row| {
            row.iter()
                .enumerate()
                .for_each(|(tile_index, tile)| {
                    if let Some(block) = tile {
                        set_tile_color(&mut tile_sprite, &block.color);

                        let tile_coordinates = BoardPosition::new(row_index, tile_index);

                        self.draw_tile(&tile_coordinates, &mut tile_sprite);
                    }
                });

            row_index += 1;
        });
    }

    pub fn render_active_piece(&mut self, model: &Model, assets: &mut GameAssets) {
        if let Some(active_piece) = model.get_active_piece() {
            let CurrentPiece { piece, position } = active_piece;

            self.render_piece(model, piece, position, assets);
        }
    }

    pub fn render_piece(
        &mut self,
        model: &Model,
        piece: &Piece,
        piece_position: &BoardPosition,
        assets: &mut GameAssets,
    ) {
        let grid = piece.get_grid(model.get_rotation_system());
        let grid_size = grid.0.len();
        let grid_num_columns = (grid_size as f32).sqrt() as usize;
        let mut tile_sprite = make_tile_sprite(assets);

        set_tile_color(&mut tile_sprite, piece.get_color());

        grid.0.iter()
            .enumerate()
            .filter(|(_, tile)| **tile)
            .for_each(|(tile_index, _)| {
                let block_in_piece_coordinates = PiecePosition::from_index(
                    tile_index,
                    grid_num_columns
                );

                let block_position = BoardPosition::new(
                    block_in_piece_coordinates.get_row() + piece_position.get_row(),
                    block_in_piece_coordinates.get_column() + piece_position.get_column(),
                );

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

fn make_tile_sprite(assets: &GameAssets) -> Sprite {
    let mut tile_sprite = assets.make_tile_sprite();
    tile_sprite.scale((TILE_SCALING, TILE_SCALING));

    tile_sprite
}

fn set_tile_color(tile_sprite: &mut Sprite, color: &PieceColor) {
    let (color_x, color_y) = get_tile_color_coordinates(color);
    let tile_size = TILE_SIZE as i32;
    let tileset_coordinates = IntRect::new(color_x, color_y, tile_size, tile_size);

    tile_sprite.set_texture_rect(&tileset_coordinates);
}

fn get_tile_color_coordinates(color: &PieceColor) -> (i32, i32) {
    let tile_size = TILE_SIZE as i32;

    match color {
        PieceColor::Blue   => (0 * tile_size, 0),
        PieceColor::Purple => (1 * tile_size, 0),
        PieceColor::Red    => (2 * tile_size, 0),
        PieceColor::Green  => (3 * tile_size, 0),
        PieceColor::Yellow => (4 * tile_size, 0),
        PieceColor::Cyan   => (5 * tile_size, 0),
        PieceColor::Orange => (6 * tile_size, 0),
    }
}

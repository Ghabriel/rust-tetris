use sfml::graphics::{Color, IntRect, RenderTarget, RenderWindow, Sprite, Transformable};
use sfml::window::{Event, Style};
use super::super::piece::PieceColor;
use super::super::position::{BoardPosition, WindowPosition};
use super::super::settings;
use super::{ActivePiece, GameAssets, Model};

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

        let board_view_position = WindowPosition::new(
            settings::BOARD_VIEW_POSITION_Y,
            settings::BOARD_VIEW_POSITION_X,
        );

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
        let mut block_sprite = make_block_sprite(assets);
        let mut row_index = 0;

        model.for_each_row(&mut |row| {
            row.iter()
                .enumerate()
                .for_each(|(tile_index, tile)| {
                    if let Some(block) = tile {
                        set_block_color(&mut block_sprite, &block.color);

                        let tile_coordinates = BoardPosition::new(row_index, tile_index as isize);

                        self.draw_block(&tile_coordinates, &mut block_sprite);
                    }
                });

            row_index += 1;
        });
    }

    pub fn render_active_piece(&mut self, model: &Model, assets: &mut GameAssets) {
        if let Some(active_piece) = model.get_active_piece() {
            self.render_piece(active_piece, assets);
        }
    }

    pub fn render_piece(
        &mut self,
        piece: &ActivePiece,
        assets: &mut GameAssets,
    ) {
        let mut block_sprite = make_block_sprite(assets);

        set_block_color(&mut block_sprite, piece.get_color());

        piece.get_block_iterator()
            .for_each(|block_position| {
                self.draw_block(&block_position, &mut block_sprite);
            });
    }

    fn draw_block(&mut self, block_position: &BoardPosition, sprite: &mut Sprite) {
        let block_window_position = self.to_window_coordinates(&block_position);
        let target_position = block_window_position + &self.board_view_position;

        sprite.set_position(target_position.as_xy());
        self.window.draw(sprite);
    }

    fn to_window_coordinates(&self, board_position: &BoardPosition) -> WindowPosition {
        let tile_scaling = settings::TILE_SCALING;
        let tile_size = settings::TILE_SIZE;
        let row = tile_scaling * (board_position.row as usize * tile_size) as f32;
        let column = tile_scaling * (board_position.column as usize * tile_size) as f32;

        WindowPosition::new(row, column)
    }
}

fn make_block_sprite(assets: &GameAssets) -> Sprite {
    let mut block_sprite = assets.make_block_sprite();
    let tile_scaling = settings::TILE_SCALING;
    block_sprite.scale((tile_scaling, tile_scaling));

    block_sprite
}

fn set_block_color(block_sprite: &mut Sprite, color: &PieceColor) {
    let (color_x, color_y) = get_block_color_coordinates(color);
    let tile_size = settings::TILE_SIZE as i32;
    let tileset_coordinates = IntRect::new(color_x, color_y, tile_size, tile_size);

    block_sprite.set_texture_rect(&tileset_coordinates);
}

fn get_block_color_coordinates(color: &PieceColor) -> (i32, i32) {
    let tile_size = settings::TILE_SIZE as i32;

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

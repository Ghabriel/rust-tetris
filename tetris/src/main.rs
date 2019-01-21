use tetris::core::{GameLoop, GameRenderer, Model};
use tetris::gravity::Gravity;
use tetris::rotations;
use tetris::settings::Settings;

// use tetris::board::SimpleBoard;

fn main() {
    // let board = SimpleBoard::from_array(&[
    //     "00000",
    //     "00000",
    //     "11111",
    //     "11111",
    // ]);

    // board.get_filled_rows();

    let settings = Settings {
        board_size: (15, 20),
        gravity: Gravity::Naive,
        rotation_system: rotations::build_nintendo_rotation_system(),
    };

    let model = Model::new(settings);
    let renderer = GameRenderer::new(800, 600, "Tetris");

    let mut game_loop = GameLoop::new(model, renderer);

    game_loop.set_update_frequency(60);
    game_loop.start();
}

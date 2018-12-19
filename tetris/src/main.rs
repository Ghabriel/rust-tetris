use tetris::gravity::Gravity;
use tetris::mvc::{Controller, MC, Model, View};
use tetris::rotations;
use tetris::settings::Settings;
use std::cell::RefCell;

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
        board_size: (10, 20),
        gravity: Gravity::Naive,
        rotation_system: rotations::build_nintendo_rotation_system(),
    };

    let model = Model::new(settings);
    let controller = Controller::new();

    let mvc = RefCell::new(MC { model, controller });
    let mut view = View::new(mvc, 800, 600, "Tetris");

    view.init();
}

use tetris::gravity::Gravity;
use tetris::mvc::{Controller, Model, View};
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
        board_size: (10, 20),
        gravity: Gravity::Naive,
        rotation_system: rotations::build_nintendo_rotation_system(),
    };

    let mut model = Model::new(settings);
    let controller = Controller::new(model);
    let mut view = View::new(controller, 800, 600, "Tetris");

    model.add_event_listener(&mut view);

    view.init();
}

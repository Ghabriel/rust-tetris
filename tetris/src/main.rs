use tetris::gravity::Gravity;
use tetris::mvc::{Controller, Model, View};
use tetris::rotations;
use tetris::settings::Settings;
use std::cell::RefCell;
use std::rc::Rc;

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

    let mut model = Rc::new(Model::new(settings));
    let controller = Controller::new(Rc::clone(&model));
    let mut view = Rc::new(
        RefCell::new(
            View::new(controller, 800, 600, "Tetris")
        )
    );

    let view_clone = Rc::clone(&view);
    Rc::get_mut(&mut model).unwrap().add_event_listener(view_clone);

    Rc::get_mut(&mut view).unwrap().borrow_mut().init();
}

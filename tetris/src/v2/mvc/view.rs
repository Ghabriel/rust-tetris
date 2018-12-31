use sfml::graphics::RenderWindow;
use sfml::window::{Event, Style};
use super::super::board::Block;
use super::Model;
use super::traits::Render;

pub struct View {
    window: RenderWindow,
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

        View {
            window,
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
        let mut num_rows = 0;

        model.for_each_row(&mut |row| {
            num_rows += 1;

            row.iter().for_each(|cell| {
                println!("Cell: {:?}", cell);
            });
        });

        println!("Num rows: {}", num_rows);
        println!("-----------------------------");
    }

    pub fn render_active_piece(&mut self, model: &Model) {
        // TODO
    }
}

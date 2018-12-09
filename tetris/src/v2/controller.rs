struct CurrentPiece {
    piece: Piece,
    position: usize,
}

struct Controller {
    board: Box<dyn Board>,
    current_piece: Option<CurrentPiece>,
}

impl Controller {
    pub fn has_active_piece(&self) -> bool {
        self.current_piece.is_some()
    }

    // pub fn active_piece_touches_board

    pub fn tick(&mut self, settings: &Settings) {
        if !self.has_active_piece() {
            return;
        }

        if !self.active_piece_touches_board(settings) {
            self.lower_active_piece();
            return;
        }

        self.materialize_active_piece(settings);
        self.clear_filled_rows(settings);
    }
}

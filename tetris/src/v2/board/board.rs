use v2::piece::Piece;
use v2::settings::Settings;

pub trait Board {
    /**
     * Required methods
     */
    fn get_num_columns(&self) -> usize;
    fn is_occupied(&self, row: usize, column: usize) -> bool;
    fn materialize(&mut self, piece: Piece, position: usize, settings: &Settings);
    fn get_filled_rows(&self) -> Vec<usize>;
    fn clear_rows(&mut self, rows: &[usize], settings: &Settings);

    /**
     * Provided methods
     */
    fn clear_filled_rows(&mut self, settings: &Settings) {
        let filled_rows = self.get_filled_rows();

        self.clear_rows(&filled_rows, settings);
    }
}

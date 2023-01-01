use super::{Sprite, ToPaddedBitFill};

impl Sprite {
    pub fn solve(&mut self) {
        self.solve_full();
    }
    pub fn solve_full(&mut self) {
        let size = self.size.val();
        for x in 0..size {
            if self.l_constraints[x as usize].is_full(size) {
                self.fill_line_with(x as usize, self.l_constraints[x as usize].to_padded_bits(), true);
            }
            if self.c_constraints[x as usize].is_full(size) {
                self.fill_column_with(x as usize, self.l_constraints[x as usize].to_padded_bits(), true);
            }
        }
        
    }
}
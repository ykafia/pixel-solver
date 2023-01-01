use super::{Sprite, ToPaddedBitFill};

impl Sprite {
    pub fn solve(&mut self) {
        self.solve_full();
        self.mask_l_ends();
        self.mask_c_ends();
    }
    pub fn solve_full(&mut self) {
        let size = self.size.val();
        for x in 0..size {
            if self.l_constraints[x as usize].is_full(size) {
                self.fill_line_with(
                    x as usize,
                    self.l_constraints[x as usize].to_padded_bits(),
                    true,
                );
            }
            if self.c_constraints[x as usize].is_full(size) {
                self.fill_column_with(
                    x as usize,
                    self.l_constraints[x as usize].to_padded_bits(),
                    true,
                );
            }
        }
    }

    pub fn mask_l_ends(&mut self) {
        let size = self.size.val();
        for x in 0..size {
            let mut zeros = 0u32;
            let mut ones = 0;
            let mut started = false;
            for y in 0..size {
                if self.lines[x as usize] >> y & 1 == 0 && !started {
                    zeros += 1;
                } else if self.lines[x as usize] >> y & 1 == 0 && started {
                    break;
                }
                if self.lines[x as usize] >> y & 1 != 0 && !started {
                    started = true;
                    ones += 1;
                }
            }
            if &ones == self.l_constraints[x as usize].last().unwrap() && ones >= zeros {
                for y in 0..zeros {
                    self.fill_mask(x as usize, (size - y) as usize)
                }
            }
        }
    }
    pub fn mask_c_ends(&mut self) {
        let size = self.size.val();
        for y in 0..size {
            let mut zeros = 0u32;
            let mut ones = 0;
            let mut started = false;
            for x in 0..size {
                let current = self.lines[x as usize];
                let v = self.lines[x as usize] >> 15 - y;
                let tmp = 0;
                if self.lines[x as usize] >> 15 - y & 1 == 0 && !started {
                    zeros += 1;
                } else if self.lines[x as usize] >> 15 - y & 1 == 0 && started {
                    break;
                } else if self.lines[x as usize] >> 15 - y & 1 != 0 && !started {
                    started = true;
                    ones += 1;
                } else if self.lines[x as usize] >> 15 - y & 1 != 0 && started {
                    ones += 1;
                }
            }
            if &ones == self.c_constraints[y as usize].last().unwrap() && ones >= zeros {
                for x in 0..zeros {
                    self.fill_mask(x as usize, (size - y) as usize)
                }
            }
        }
    }
}

use std::{collections::btree_map::Values, fmt, process::id};

pub enum SpriteSize {
    PX5 = 5,
    PX10 = 10,
    PX15 = 15,
}

pub enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

impl SpriteSize {
    fn val(&self) -> u32 {
        match self {
            Self::PX5 => 5,
            Self::PX10 => 5,
            Self::PX15 => 15,
        }
    }
}

pub struct Sprite {
    lines: [u16; 16],
    mask: [u16; 16],
    size: SpriteSize,
    l_constraints: Vec<Vec<u32>>,
    c_constraints: Vec<Vec<u32>>,
}

impl Sprite {
    pub fn new(
        size: SpriteSize,
        l_constraints: Vec<Vec<u32>>,
        c_constraints: Vec<Vec<u32>>,
    ) -> Self {
        Sprite {
            lines: [0; 16],
            mask: [0; 16],
            size,
            l_constraints,
            c_constraints,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> u16 {
        self.lines[x] & 1 << y
    }
    pub fn fill(&mut self, x: usize, y: usize) {
        self.lines[x] |= 1 << 15 - y
    }

    pub fn solve(&mut self) {}
    pub fn solve_full(&mut self) {
        for i in 0..self.size.val() {
            let size = self.size.val() as u32;
            match self.l_constraints[i as usize].as_slice() {
                [s] if s == &size => self.fill_line(i as usize),
                _ => (),
            }
            match self.c_constraints[i as usize].as_slice() {
                [s] if s == &size => self.fill_column(i as usize),
                _ => (),
            }
        }
    }

    pub fn check_constraints(&self) -> bool {
        (0..self.size.val())
            .all(|x| self.check_l_constraint(x as usize) && self.check_c_constraint(x as usize))
    }
    pub fn check_l_constraint(&self, idx: usize) -> bool {
        let array = self.lines[idx];
        println!("Starting check");
        let mut vals = Vec::new();
        let mut count = 0u32;
        for x in 0..self.size.val() {
            match (array & 1 << 15 - x != 0, count) {
                (true, _) => count += 1,
                (false, 0) => (),
                (false, 1..) => {
                    vals.push(count);
                    count = 0;
                }
            }
        }
        if count > 0 {
            vals.push(count)
        }
        let lc = &self.l_constraints[idx];
        vals.eq(lc)
    }
    pub fn check_c_constraint(&self, idx: usize) -> bool {
        let mut vals = Vec::new();
        let mut count = 0u32;
        for x in 0..self.size.val() {
            match (self.lines[x as usize] & 1 << 15 - idx != 0, count) {
                (true, _) => count += 1,
                (false, 0) => (),
                (false, 1..) => {
                    vals.push(count);
                    count = 0;
                }
            }
        }
        if count > 0 {
            vals.push(count)
        }
        let lc = &self.c_constraints[idx];
        vals.eq(lc)
    }

    pub fn fill_line(&mut self, idx: usize) {
        self.lines[idx] = u16::MAX;
    }
    pub fn fill_line_with(&mut self, idx: usize, value: u16, align_left: bool) {
        self.lines[idx] = if align_left {
            value
        } else {
            value >> self.size.val() - value.padded_bits_len()
        };
    }

    pub fn fill_column(&mut self, idx: usize) {
        for x in 0..self.size.val() {
            self.lines[x as usize] |= 1 << 15 - idx;
        }
    }
    pub fn fill_column_with(&mut self, idx: usize, value: u16, align_top: bool) {
        if align_top {
            for x in 0..self.size.val() {
                self.lines[x as usize] |= (value >> 15 - x & 1) << 15 - idx;
            }
        } else {
            let start = self.size.val() - value.padded_bits_len();
            
            for x in start..self.size.val() {
                self.lines[x as usize] |= (value >> 16 - x & 1) << 15 - idx;
            }
        }
    }

    pub fn fill_from(&mut self, idx: usize, value: u16, direction: Direction) {
        match direction {
            Direction::TOP | Direction::BOTTOM => (),
            Direction::LEFT | Direction::RIGHT => (),
        }
    }

    pub fn new_square_5() -> Self {
        Sprite {
            lines: [0; 16],
            mask: [0; 16],
            size: SpriteSize::PX5,
            l_constraints: vec![vec![1], vec![3], vec![5], vec![5], vec![3]],
            c_constraints: vec![vec![1, 2], vec![4], vec![4], vec![4], vec![2]],
        }
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push_str("  ");
        s.push_str(
            &(0..self.size.val())
                .into_iter()
                .map(|x| format!(" {:2}", x + 1))
                .collect::<Vec<String>>()
                .join(""),
        );
        s.push('\n');
        for x in 0..self.size.val() {
            let mut line = format!("{:016b}", self.lines[x as usize])
                .split_at((self.size.val() + 1) as usize)
                .0
                .to_string()
                .replace("0", " ░░")
                .replace("1", " ██");
            line.pop();
            line.pop();
            s.push_str(format!("{:2}{}\n\n", x + 1, line).as_str());
        }
        write!(f, "{s}")
    }
}
trait DisplayBits {
    fn display_bits(&self);
    fn padded_bits_len(&self) -> u32;
}
impl DisplayBits for u16 {
    fn display_bits(&self) {
        println!("{:b}", self)
    }
    fn padded_bits_len(&self) -> u32 {
        let mut start = 0;
        let mut end = 0;
        let mut started = false;
        for x in 0..16 {
            if self & 1 << 15 - x != 0 && !started {
                start = x;
                started = true;
            }
            if self & 1 << 15 - x != 0 && started {
                end = x;
            }
        }
        end - start + 1
    }
}
pub trait ToPaddedBitFill {
    fn to_padded_bits(&self) -> u16;
    fn is_full(&self, size: u32) -> bool;
    fn padded_bits_len(&self) -> usize;
}
impl ToPaddedBitFill for Vec<u32> {
    fn to_padded_bits(&self) -> u16 {
        let mut result = 0;
        let mut bit_idx = 0;
        for x in self {
            for _ in 0..x.to_owned() {
                result |= 1 << 15 - bit_idx;
                bit_idx += 1;
            }
            bit_idx += 1
        }
        result
    }
    fn is_full(&self, size: u32) -> bool {
        self.iter().sum::<u32>() + self.len() as u32 - 1 == size
    }
    fn padded_bits_len(&self) -> usize {
        self.iter().count() + self.len() - 1
    }
}

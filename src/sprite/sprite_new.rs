use super::{Sprite, SpriteSize};


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
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
}
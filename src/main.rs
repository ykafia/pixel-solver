mod sprite;

use sprite::*;

fn main() {
    // let mut sprite = Sprite::new(
    //     SpriteSize::PX15,
    //     vec![vec![7, 7]], 
    //     vec![vec![7, 7]]
    // );
    // sprite.fill_column_with(0, vec![4,4,5].to_padded_bits());
    // sprite.fill_line_with(0, vec![9,5].to_padded_bits());
    // let val = vec![4,4,4].is_full(15);
    // println!("{}",val);

    let mut sprite = Sprite::new_square_5();
    sprite.solve();
    println!("{sprite}");
    println!("{}", sprite.check_c_constraint(0));
}


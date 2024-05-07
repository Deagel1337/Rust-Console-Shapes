pub mod line;
mod draw;
use crate::{draw::Draw, line::Line};

fn main() {
    let l1: Line = Line::new(9,3,7,4);
    let t = l1.draw_field();
    for i in 0..10 {
        for j in 0..10 {
            print!("{}",t[i][j]);
        }
        println!();
    }
}

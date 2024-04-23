mod line;
mod draw;
use crate::{draw::Draw, line::Line};

fn main() {
    let l1: Line = Line::new(10,3,7,4);
    println!("Coordinates of l1: {:?}",l1.coordinates());
    println!("{}",l1.length());
    println!("{}",l1.draw());

}

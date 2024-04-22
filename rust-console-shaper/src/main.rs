mod line;
use crate::line::Line;

fn main() {
    let l1: Line = Line::new(3,4);
    println!("Coordinates of l1: {:?}",l1.coordinates());
}

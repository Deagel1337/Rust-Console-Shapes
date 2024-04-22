#[derive(Debug)]
pub struct Line {
    x: i32,
    y: i32
}

impl Line {
    pub fn new(new_x: i32, new_y: i32) -> Line {
        Line {
            x: new_x,
            y: new_y
        }
    }

    pub fn coordinates(&self) -> (i32,i32){
        (self.x,self.y)
    }
}

// impl fmt::Display for Line {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

//         write!(f, "{}", self.0)
//     }
// }
use crate::draw::Draw;

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

    pub fn length(&self) -> f32 {
        f32::sqrt((i32::pow(self.x,2) as f32)-(i32::pow(self.y,2) as f32))
    }
}

impl Draw for Line {
    fn draw(&self) -> String {
        let str: String = String::new();
    }
}

// impl fmt::Display for Line {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

//         write!(f, "{}", self.0)
//     }
// }
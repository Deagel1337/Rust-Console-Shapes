use crate::draw::Draw;

#[derive(Debug)]
pub struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl Line {
    pub fn new(new_x1: i32, new_y1: i32, new_x2:i32, new_y2:i32) -> Line {
        Line {
            x1: new_x1,
            y1: new_y1,
            x2: new_x2,
            y2: new_y2,

        }
    }

    pub fn coordinates(&self) -> Box<[(i32,i32)]>{
        Box::new([(self.x1,self.y1),(self.x2,self.y2)])
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(((i32::pow(self.x1,2) - i32::pow(self.x2,2)) as f32 )
                    +   (i32::pow(self.y2,2)-i32::pow(self.y1,2)) as f32)
    }

    pub(crate) fn draw_field(&self) -> [[&str;10];10]{
        let mut field: [[&str;10];10] = [["-";10];10];
        for i in 0..10{
            for j in  0..10{
                if i == self.x1 && j == self.y1 {
                    field[i as usize][j as usize] = "*";
                }
                if i == self.x2 && j == self.y2 {
                    field[i as usize][j as usize] = "*";
                }
            }
        }

        return field;
    }
}

impl Draw for Line {
    fn draw(&self) -> String {
        let mut str: String = String::new();
        let length: f32 = self.length();
        for _ in 1..(length as i32) {
            str.push('#');
        }
        return str;
    }
}
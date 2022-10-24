use crate::model::point::Point;
use crate::model::line::Line;
use crate::model::behavier::figure::Figure;

#[derive(Clone)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {

    // 面積
    pub fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
  
        ((x1 - x2) * (y1 - y2)).abs()
    }

    // 外周
    pub fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
  
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

}

impl Figure for Rectangle{

    // 点リスト
    fn get_point_list(&self) -> Vec<Point> {

        let mut v = Vec::new();

        v.push(self.p1.clone());
        v.push(self.p2.clone());

        v
    }

    // 点リスト
    fn set_point_list(&mut self, point_list: Vec<Point>) {

        self.p1 = point_list[0].clone();
        self.p2 = point_list[1].clone();
    }
    
    // 線分リスト
    fn get_line_list(&self) -> Vec<Line> {

        let mut v = Vec::new();

        let l = Line {
            p1: Point::new(self.p1.x, self.p1.y),
            p2: Point::new(self.p1.x, self.p2.y),
        };
        v.push(l);

        let l = Line {
            p1: Point::new(self.p1.x, self.p2.y),
            p2: Point::new(self.p2.x, self.p2.y),
        };
        v.push(l);

        let l = Line {
            p1: Point::new(self.p2.x, self.p2.y),
            p2: Point::new(self.p2.x, self.p1.y),
        };
        v.push(l);

        let l = Line {
            p1: Point::new(self.p2.x, self.p1.y),
            p2: Point::new(self.p1.x, self.p1.y),
        };
        v.push(l);

        v
    }

}

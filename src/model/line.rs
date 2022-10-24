use crate::model::point::Point;
use crate::model::behavier::figure::Figure;

#[derive(Clone)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
 
}


impl Figure for Line{

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

        let l = self.clone();
        v.push(l);

        v
    }
}

use crate::model::point::Point;
use crate::model::line::Line;
use crate::model::behavier::figure::Figure;

#[derive(Clone)]
pub struct Polygon {
    pub point_list: Vec<Point>,
}

impl Polygon {

}

impl Figure for Polygon{

    // 点リスト
    fn get_point_list(&self) -> Vec<Point> {

        self.point_list.clone()
    }

    // 点リスト
    fn set_point_list(&mut self, point_list: Vec<Point>) {

        self.point_list = point_list;
    }
    
    // 線分リスト
    fn get_line_list(&self) -> Vec<Line> {

        // NOTE: 座標リストを右回り(or左回り)で整列してある前提

        let mut v = Vec::new();

        let mut i: u32 = 0;
        let mut first_point = Point::base();
        let mut start_point = Point::base();

        for point in self.point_list.clone() {
            if i == 0 {
                first_point = point.clone();    
                start_point = point;
            }
            else
            {
                let end_point = point.clone();
                let l = Line {
                    p1: start_point.clone(),
                    p2: end_point,
                };
                v.push(l);            
                start_point = point;        
            }
            i += 1;
        }    
        let l = Line {
            p1: start_point.clone(),
            p2: first_point,
        };
        v.push(l);            

        v
    }

}

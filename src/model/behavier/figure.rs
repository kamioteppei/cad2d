use crate::model::point::Point;
use crate::model::line::Line;

pub trait Figure {

    // 点リスト
    fn get_point_list(&self) -> Vec<Point>;

    // 点リスト
    fn set_point_list(&mut self, point_list: Vec<Point>);

    // 線分リスト
    fn get_line_list(&self) -> Vec<Line>;

}
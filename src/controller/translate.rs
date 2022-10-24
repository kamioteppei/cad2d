use crate::model::point::Point;
use crate::model::behavier::figure::Figure;
use ndarray::prelude::*;

pub struct Translate {
    pub base: Point,
}

impl Translate {
 
    // 移動
    pub fn r#move<T: Figure>(&self, figure: &mut T, x: f64, y: f64) {

        let mut new_point_list = Vec::new();

        let point_list: Vec<Point> = figure.get_point_list();
        for point in point_list {
            new_point_list.push(
                Point {
                    x: point.x + x,
                    y: point.y + y,
                }
            );
        }    

        figure.set_point_list(new_point_list);
    }

    // 回転
    pub fn rotate<T: Figure>(&self, figure: &mut T, rad: f64) {

        let mut new_point_list = Vec::new();

        let point_list: Vec<Point> = figure.get_point_list();
        for point in point_list {

            // 変換情報の行列
            let mat_rotate_info = arr2(&[
                [rad.cos(), -rad.sin()],
                [rad.sin(), rad.cos()]
            ]);
            println!("{}", mat_rotate_info);

            // 変換対象の座標の行列
            let mat_target_point = arr2(&[
                [point.x],
                [point.y]
            ]);

            // 行列の内積
            let mat_rotated_point = mat_rotate_info.dot(&mat_target_point);
            println!("{}", mat_rotated_point);

            new_point_list.push(
                Point {
                    x: mat_rotated_point[[0,0]],
                    y: mat_rotated_point[[1,0]],
                }
            );
        }    

        figure.set_point_list(new_point_list);
    }

}

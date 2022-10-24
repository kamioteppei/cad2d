extern crate image;
extern crate imageproc;
use std::f64;

use image::{Rgb, RgbImage};
use imageproc::drawing::draw_line_segment_mut;

use crate::model::{
    point::Point,
    rectangle::Rectangle,
    polygon::Polygon,
    behavier::figure::Figure,
};
use crate::controller::translate::Translate;
use crate::r#const;

pub fn run() {

    let mut img = RgbImage::new(
        r#const::VIEW_SIZE_WIDTH,
        r#const::VIEW_SIZE_HEIGHT
    );

    let translate = Translate { 
        base: Point::base(),
    };

    ////////////////////////////////////
    // 矩形
    ////////////////////////////////////
    let rectangle = Rectangle {
        p1: Point::new(300.0, 300.0),
        p2: Point::new(500.0, 500.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle are: {}", rectangle.area());

    let lines = rectangle.get_line_list();

    for line in &lines {
        draw_line_segment_mut(
            &mut img,
            convert2_view_point(&line.p1),
            convert2_view_point(&line.p2),
            Rgb([69u8, 203u8, 133u8]), // RGB colors
        );
    }

    ////////////////////////////////////
    // 矩形を移動
    ////////////////////////////////////
    let mut rectangle_move = rectangle.clone();

    translate.r#move(&mut rectangle_move, 100.0, 100.0);

    let lines = rectangle_move.get_line_list();

    for line in &lines {
        draw_line_segment_mut(
            &mut img,
            convert2_view_point(&line.p1),
            convert2_view_point(&line.p2),
            Rgb([255u8, 255u8, 255u8]), // RGB colors
        );
    }

    ////////////////////////////////////
    // 移動した矩形を回転
    ////////////////////////////////////
    let mut polygon_point_list = Vec::new();
    polygon_point_list.push(Point::new(rectangle_move.p1.x, rectangle_move.p1.y));
    polygon_point_list.push(Point::new(rectangle_move.p2.x, rectangle_move.p1.y));
    polygon_point_list.push(Point::new(rectangle_move.p2.x, rectangle_move.p2.y));
    polygon_point_list.push(Point::new(rectangle_move.p1.x, rectangle_move.p2.y));

    let mut polygon = Polygon {
        point_list: polygon_point_list,
    };

    translate.rotate(&mut polygon, 30.0 / 180.0 * f64::consts::PI );

    let lines = polygon.get_line_list();

    for line in &lines {
        draw_line_segment_mut(
            &mut img,
            convert2_view_point(&line.p1),
            convert2_view_point(&line.p2),
            Rgb([255u8, 0u8, 255u8]), // RGB colors
        );
    }

    ////////////////////////////////////
    // 出力画像保存
    //////////////////////////////////// 
    img.save("translate_figure.png").unwrap();
}

fn convert2_view_point(point: &Point) -> (f32, f32 ) {
    let x: f32 = point.x as f32;
    let y: f32 = point.y as f32;
    (x, r#const::VIEW_SIZE_HEIGHT as f32 - y)
}
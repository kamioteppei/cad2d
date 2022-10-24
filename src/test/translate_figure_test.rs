use crate::model::{
    point::Point,
    rectangle::Rectangle,
    polygon::Polygon,
    behavier::figure::Figure,
};
use crate::controller::translate::Translate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {

        let translate = Translate { 
            base: Point::base(),
        };
    
        ////////////////////////////////////
        // 矩形
        ////////////////////////////////////
        let mut rectangle = Rectangle {
            p1: Point::new(300.0, 300.0),
            p2: Point::new(500.0, 500.0),
        };
        
        ////////////////////////////////////
        // 矩形を移動
        ////////////////////////////////////    
        translate.r#move(&mut rectangle, 100.0, 100.0);
    
        let p1 = rectangle.p1;
        let p2 = rectangle.p2;
    
        assert_eq!(400.0, p1.x);
        assert_eq!(400.0, p1.y);
        assert_eq!(600.0, p2.x);
        assert_eq!(600.0, p2.y);
    }
}

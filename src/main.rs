mod ml_data;

use crate::ml_data::read_ml_json;
use crate::ml_data::MLDataContainer;
use crate::ml_data::Node;
use std::path::Path;

fn consume_s(s: String) -> usize {
    s.len()
}
enum State<T, Q = i32> {
    ON(Q),
    OFF(T),
}
mod topology {
    pub struct Point {
        x: f64,
        y: f64,
    }
    pub struct Square {
        p_tl: Point,
        p_br: Point,
    }
    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }
        pub fn x(&self) -> f64 {
            self.x
        }
        pub fn y(&self) -> f64 {
            self.y
        }

        pub fn manhattan_distance_point(&self, pnt: &Point) -> f64{
            let mut value_x:f64 = self.x-pnt.x;
            let mut value_y:f64 = self.y-pnt.y;
            return value_x.abs() + value_y.abs();
        }

    }

    impl Square {
        pub fn new(p1: Point, p2: Point) -> Self {
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);
            Self {
                p_tl: Point::new(min_x, min_y),
                p_br: Point::new(max_x, max_y),
            }
        }
        
        pub fn lower(&self) -> &Point {
            &self.p_tl
        }
        pub fn upper(&self) -> &Point {
            &self.p_br
        }
        pub fn height(&self) -> f64 {
            f64::abs(self.p_br.y - self.p_tl.y)
        }
        pub fn width(&self) -> f64 {
            f64::abs(self.p_br.x - self.p_tl.x)
        }
        pub fn area(&self) -> f64 {
            self.width() * self.height()
        }
        pub fn erosion(&mut self, d: f64) {
            self.p_tl.x = self.p_tl.x + d;
            self.p_tl.y = self.p_tl.y + d;
            self.p_br.x = self.p_br.x - d;
            self.p_br.y = self.p_br.y - d;
        }
        pub fn dilate(&mut self, d: f64) -> () {
            self.p_tl.x = self.p_tl.x - d;
            self.p_tl.y = self.p_tl.y - d;
            self.p_br.x = self.p_br.x + d;
            self.p_br.y = self.p_br.y + d;
        }
        pub fn intersection(&self, other: &Self) -> Self {
            let x1 = self.p_tl.x.max(other.p_tl.x);
            let y1 = self.p_tl.y.max(other.p_tl.y);
            let x2 = self.p_br.x.min(other.p_br.x);
            let y2 = self.p_br.y.min(other.p_br.y);
            if x1 > x2 || y1 > y2 {
                Square::new(Point::new(0.0, 0.0), Point::new(0.0, 0.0))
            } else {
                Square::new(Point::new(x1, y1), Point::new(x2, y2))
            }
        }
        pub fn union(&self, other: &Self) -> Self {
            let x1 = self.p_tl.x.min(other.p_tl.x);
            let y1 = self.p_tl.y.min(other.p_tl.y);
            let x2 = self.p_br.x.max(other.p_br.x);
            let y2 = self.p_br.y.max(other.p_br.y);
            Square::new(Point::new(x1, y1), Point::new(x2, y2))
        }
        pub fn dilate_x(&mut self, d: f64) -> () {
            let wth = self.width() * 0.5 * d;
            let mid_x = (self.p_br.x - self.p_tl.x) * 0.5;
            self.p_tl.x = mid_x - wth;
            self.p_br.x = mid_x + wth;
        }
        pub fn dilate_y(&mut self, d: f64) -> () {
            let wth = self.height() * 0.5 * d;
            let mid_y = (self.p_tl.y - self.p_tl.y) * 0.5;
            self.p_tl.y = mid_y + wth;
            self.p_br.y = mid_y - wth;
        }
        pub fn erosion_x(&mut self, d: f64) -> () {
            self.dilate_x(1.0 / d);
        }
        pub fn erosion_y(&mut self, d: f64) -> () {
            self.dilate_y(1.0 / d);
        }

        pub fn has_point(&self, pnt: &Point) -> bool {
            if self.p_tl.x<=pnt.x && pnt.x<=self.p_br.x && self.p_tl.y<=pnt.y && pnt.y<=self.p_br.y{
                return true;
            }
            return false;
        }

        pub fn has_square(&self, sqr: &Square) -> bool {
            let temp_point_1:Point = Point::new( sqr.p_tl.x , sqr.p_tl.y + sqr.height());
            let temp_point_2:Point = Point::new( sqr.p_tl.x + sqr.width(), sqr.p_tl.y );

            if self.has_point(&sqr.p_tl) || self.has_point(&sqr.p_br) ||
                self.has_point(&temp_point_1) || self.has_point(&temp_point_2) {
                return true;
            }
            return false;
        }

        pub fn manhattan_distance(&self, sqr: &Square) -> f64 {

            let bottom_sqr:&Square;
            let top_sqr:&Square;

            if self.has_square(&sqr) || sqr.has_square(&self){
                return 0.0;
            }

            if self.p_tl.y<=sqr.p_br.y{
                bottom_sqr = &self;
                top_sqr = &sqr;
            }else{
                bottom_sqr = &sqr;
                top_sqr = &self;
            }

            if bottom_sqr.p_br.x<=top_sqr.p_tl.x{
                return bottom_sqr.p_br.manhattan_distance_point( &bottom_sqr.p_tl );
            } else {
                let pnt1:Point = Point::new( bottom_sqr.p_tl.x , bottom_sqr.p_tl.y + bottom_sqr.height()) ;
                let pnt2:Point = Point::new( top_sqr.p_br.x , top_sqr.p_br.y - top_sqr.height()) ;
                return pnt1.manhattan_distance_point(&pnt2);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::topology::{Point, Square};
    #[test]
    fn point_test() {
        let p = Point::new(10.0, 10.0);
        assert_eq!(p.y(), 10.0);
        assert_eq!(p.x(), 10.0);
    }
    #[test]
    fn sq_test() {
        let p1: Point = Point::new(0.0, 0.0);
        let p2: Point = Point::new(1.0, 2.0);
        let sq: Square = Square::new(p1, p2);
        //assert_eq!(sq.area(),2.0);
        assert!(sq.lower().x() < sq.upper().x());
        assert!(sq.lower().y() < sq.upper().y());
    }
    #[test]
    fn dilate_test() {
        let p1: Point = Point::new(0.0, 2.0);
        let p2: Point = Point::new(1.0, 0.0);
        let mut sq: Square = Square::new(p1, p2);
        sq.dilate(2.0);
        assert_eq!(sq.area(), 30.0);
    }
    #[test]
    fn intersection_test_corner() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(1.0, 1.0), Point::new(4.0, 4.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.lower().x(), 1.0);
        assert_eq!(s3.lower().y(), 1.0);
        assert_eq!(s3.upper().x(), 3.0);
        assert_eq!(s3.upper().y(), 3.0);
    }
    #[test]
    fn intersection_test_cross() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(1.0, -1.0), Point::new(2.0, 4.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.lower().x(), 1.0);
        assert_eq!(s3.lower().y(), 0.0);
        assert_eq!(s3.upper().x(), 2.0);
        assert_eq!(s3.upper().y(), 3.0);
    }
    #[test]
    fn intersection_test_out() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(5.0, 5.0), Point::new(10.0, 10.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.lower().x(), 0.0);
        assert_eq!(s3.lower().y(), 0.0);
        assert_eq!(s3.upper().x(), 0.0);
        assert_eq!(s3.upper().y(), 0.0);
    }
    #[test]
    fn union_test_cross() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(1.0, -1.0), Point::new(2.0, 4.0));
        let s3 = s1.union(&s2);
        assert_eq!(s3.lower().x(), 0.0);
        assert_eq!(s3.lower().y(), -1.0);
        assert_eq!(s3.upper().x(), 3.0);
        assert_eq!(s3.upper().y(), 4.0);
    }
    fn erosion_test() {
        let p1: Point = Point::new(0.0, 4.0);
        let p2: Point = Point::new(4.0, 0.0);
        let mut sq: Square = Square::new(p1, p2);
        sq.erosion(0.5);

        assert_eq!(sq.area(), 9.0);
    }

    #[test]
    fn has_point_test(){
        let p1:Point = Point::new(0.0,0.0);
        let p2:Point = Point::new(1.0,1.0);

        let p3:Point = Point::new(2.0,2.0);
        let p4:Point = Point::new(3.0,3.0);

        let p5:Point = Point::new(0.5,0.5);

        let sqr1:Square = Square::new(p1,p2);
        let sqr2:Square = Square::new(p3,p4);

        assert!( sqr1.has_point(&p5) );
        assert!( !sqr2.has_point(&p5) );
    }

    #[test]
    fn has_square_test(){
        let p1:Point = Point::new(0.0,0.0);
        let p2:Point = Point::new(4.0,4.0);

        let p3:Point = Point::new(2.0,2.0);
        let p4:Point = Point::new(3.0,3.0);

        let sqr1:Square = Square::new(p1,p2);
        let sqr2:Square = Square::new(p3,p4);

        assert!( sqr1.has_square(&sqr2) );
        assert!( !sqr2.has_square(&sqr1) );
    }

    #[test]
    fn manhattan_distance_test(){
        let p1:Point = Point::new(0.0,0.0);
        let p2:Point = Point::new(1.0,1.0);

        let p3:Point = Point::new(2.0,2.0);
        let p4:Point = Point::new(3.0,3.0);

        let sqr1:Square = Square::new(p1,p2);
        let sqr2:Square = Square::new(p3,p4);

        assert_eq!(sqr1.manhattan_distance(&sqr2), 2.0);
    }
}

//Encontrar nodo XX
pub fn node_XX( data : &MLDataContainer ) -> Option<Node> { 

    for i in 0..data.element_statistics.nodes.len() {
        if data.element_statistics.nodes[i].a.contains_key("XX"){
            return Some(data.element_statistics.nodes[i].clone());
        }
    }
    return None;
}

// Correlación de dos nodos
pub fn correlation_two_nodes(node_1:&Node, node_2:&Node) -> f64 { 
    let mut total:f64 = 0.0;
    let mut count:f64 = 0.0;

    for key in node_1.a.keys(){
        //no se consideran esas llaves en la correlación
        if key=="XX" || key=="HT" || key=="WH" || key=="TP"{ 
            continue;
        }
        total += 1.0;

        // checa si lo contiene y es igual
        if node_2.a.contains_key( key ) && node_1.a[key]==node_2.a[key]{ 
            count += 1.0;
        }
    }
    return count/total;
}

// vector de correlación, itera un nodo con un vector de nodos
pub fn correlation_vector( node_org:&Node , nodes_vecs:&Vec<Node> ) -> Vec<f64>{ 

    let mut correlation_vec:Vec<f64> = vec![0.0; nodes_vecs.len()];
    let mut i:usize=0;

    for node in nodes_vecs{
        correlation_vec[i] = correlation_two_nodes(&node_org, &node);
        i += 1;
    }
    return correlation_vec;
}

fn main(){
    let old_path = Path::new("resources/1663154348643_8ZGUJJLLWV/ml_data/1663154348643_8ZGUJJLLWV.json");
    let old_data = read_ml_json(&old_path);
    
    let XX_node = node_XX( &old_data ).unwrap();
    let nodes_vector = read_ml_json(&old_path).element_statistics.nodes;
    let correlation_vector = correlation_vector( &XX_node, &nodes_vector );
    
    //println!("{:?}",XX_node);
    println!("{:?}",correlation_vector);
    
}
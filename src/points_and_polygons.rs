// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::{ops::Add};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    // a² + b² = c²
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(x^2 + y^2)
    }

    // d=√((x2 – x1)² + (y2 – y1)²)
    pub fn dist(&self, p2: Point) -> f64{
        let dif_x_sq = (self.x - p2.x)^2;
        let dif_y_sq = (self.y - p2.y)^2;
        f64::sqrt(dif_x_sq + dif_y_sq)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        todo!()
    }
}

pub struct Polygon (Vec<Point>);

impl Polygon {
    pub fn new() -> Self {
        Polygon(Vec::new())
    }

    pub fn add_point(&mut self, new_point: Point) {
        todo!()
    }

    pub fn left_most_point(&mut self) -> Option<Point> {
        todo!()
    }
}

pub struct Circle {
    circumference: f64
}

impl Circle {
    pub fn new(center: Point, rad: i32) -> Self {
        todo!()
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(Polygon)
    }
}
impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(Circle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.0.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        // let shapes = vec![
        //     Shape::from(poly),
        //     Shape::from(Circle::new(Point::new(10, 20), 5)),
        // ];
        // let circumferences = shapes
        //     .iter()
        //     .map(Shape::circumference)
        //     .map(round_two_digits)
        //     .collect::<Vec<_>>();
        // assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}

#![allow(dead_code)]
#![allow(unused_variables)]

use num_complex::Complex;
use std::{f64::consts::PI, f64::consts::TAU, fmt};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub id: String,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {}: ({}, {})", self.id, self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let x_close = (self.x - other.x).abs() <= 0.00001;
        let y_close = (self.y - other.y).abs() <= 0.00001;
        let id_equal = self.id == other.id;

        x_close && y_close && id_equal
    }
}
impl Eq for Point {}

impl Point {
    pub fn new(x: f64, y: f64, id: &str) -> Point {
        Point {
            x,
            y,
            id: id.to_string(),
        }
    }

    pub fn to_complex(&self) -> Complex<f64> {
        let complex_float = Complex::new(self.x, self.y);
        complex_float
    }
}

pub fn angle(a: &Point, b: &Point, c: &Point) -> f64 {
    // output range is (0, 2*PI]
    let ba_dx: f64 = a.x - b.x;
    let ba_dy: f64 = a.y - b.y;
    let ba_angle: f64 = ba_dy.atan2(ba_dx);

    let bc_dx = c.x - b.x;
    let bc_dy = c.y - b.y;
    let bc_angle = bc_dy.atan2(bc_dx);

    let mut naive_angle = bc_angle - ba_angle;
    if naive_angle <= 0.0 {
        naive_angle += TAU;
    }
    naive_angle
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    pub fn reverse(&self) -> Line {
        Line {
            start: self.end.clone(),
            end: self.start.clone(),
        }
    }

    pub fn angle(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        dy.atan2(dx)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arc {
    pub start: Point,
    pub end: Point,
    pub transit: Point,
}

impl Arc {
    pub fn new(start: Point, end: Point, transit: Point) -> Arc {
        Arc {
            start,
            end,
            transit,
        }
    }

    pub fn reverse(&self) -> Arc {
        Arc {
            start: self.end.clone(),
            end: self.start.clone(),
            transit: self.transit.clone(),
        }
    }

    pub fn to_circle(&self) -> Circle {
        // This is taken from https://math.stackexchange.com/a/3503338/816177
        let z1 = self.start.to_complex();
        let z2 = self.transit.to_complex();
        let z3 = self.end.to_complex();

        // if (z1 == z2) or (z2 == z3) or (z3 == z1){
        //     raise ValueError(f"Duplicate points: {z1}, {z2}, {z3}")
        // }

        let w = (z3 - z1) / (z2 - z1);

        // if abs(w.imag) <= 0.0000001 {
        //     // points are colinear!
        // }

        let two_j = Complex::new(0.0, 2.0);

        let c = (z2 - z1) * (w - w.norm_sqr()) / (two_j * w.im) + z1; // Simplified denominator

        let r = (z1 - c).norm();

        return Circle {
            center: Point::new(c.re, c.im, "center_point"),
            radius: r,
        };
    }

    pub fn angle(&self) -> f64 {
        let circle = self.to_circle();
        let dx = self.start.x - circle.center.x;
        let dy = self.start.y - circle.center.y;
        let center_to_start = dy.atan2(dx);
        // println!("\nAngle from center to start: {}", center_to_start);
        // the correct answer is one of these two, but we have to choose:
        let angle_0 = center_to_start + PI / 2.0;
        let angle_1 = center_to_start - PI / 2.0;
        // println!("Angle 0: {}", angle_0);
        // println!("Angle 1: {}", angle_1);

        let dx_to_transit = self.transit.x - self.start.x;
        let dy_to_transit = self.transit.y - self.start.y;
        let mut angle_to_transit = dy_to_transit.atan2(dx_to_transit);
        if angle_to_transit < 0.0 {
            angle_to_transit += TAU;
        }
        // println!("dx to transit: {}", dx_to_transit);
        // println!("dy to transit: {}", dy_to_transit);
        // println!("Angle to transit: {}", angle_to_transit);

        let angle_0_err = min_angle_diff(angle_0, angle_to_transit);
        // println!("Angle 0 err: {}", angle_0_err);
        let angle_1_err = min_angle_diff(angle_1, angle_to_transit);
        // println!("Angle 1 err: {}", angle_1_err);
        if angle_0_err.abs() < angle_1_err.abs() {
            // println!("Returning Angle 0: {}", angle_0);
            return angle_0;
        } else {
            // println!("Returning Angle 1: {}", angle_1);
            return angle_1;
        }
    }
}

pub fn min_angle_diff(a0: f64, a1: f64) -> f64 {
    let path_a = angle_difference(a0, a1);
    let path_b = angle_difference(a1, a0);
    if path_a < path_b {
        return path_a;
    } else {
        return path_b;
    }
}

pub fn angle_difference(mut a0: f64, mut a1: f64) -> f64 {
    if a0 > TAU {
        a0 -= TAU;
    }
    if a0 < 0.0 {
        a0 += TAU;
    }

    if a1 > TAU {
        a1 -= TAU;
    }
    if a1 < 0.0 {
        a1 += TAU;
    }

    let mut naive_diff = a1 - a0;
    if naive_diff > TAU {
        naive_diff -= TAU;
    }
    if naive_diff < 0.0 {
        naive_diff += TAU;
    }

    return naive_diff;
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment {
    Line(Line),
    Arc(Arc),
}

impl Segment {
    pub fn reverse(&self) -> Segment {
        match self {
            Segment::Line(l) => Segment::Line(l.reverse()),
            Segment::Arc(a) => Segment::Arc(a.reverse()),
        }
    }

    pub fn angle(&self) -> f64 {
        match self {
            Segment::Line(l) => l.angle(),
            Segment::Arc(a) => a.angle(),
        }
    }

    pub fn get_start(&self) -> Point {
        match self {
            Segment::Line(l) => l.start.clone(),
            Segment::Arc(a) => a.start.clone(),
        }
    }

    pub fn get_end(&self) -> Point {
        match self {
            Segment::Line(l) => l.end.clone(),
            Segment::Arc(a) => a.end.clone(),
        }
    }

    pub fn continues(&self, prior_segment: &Segment) -> bool {
        // determines if this segment continues the prior segment
        if prior_segment.get_end() == self.get_start() {
            true
        } else {
            false
        }
    }

    pub fn equals_or_reverse_equals(&self, other: &Self) -> bool {
        self == other || self == &other.reverse()
    }
}

type Ring<'a> = Vec<&'a mut Segment>;

#[derive(Debug, Clone)]
pub struct Sketch {
    pub segments: Vec<Segment>,
}

impl Sketch {
    pub fn new(segments: Vec<Segment>) -> Sketch {
        Sketch { segments }
    }

    pub fn find_faces(&self) -> Vec<Vec<usize>> {
        let mut segments_overall: Vec<Segment> = self.segments.iter().map(|s| s.clone()).collect();
        let segments_reversed: Vec<Segment> = self.segments.iter().map(|s| s.reverse()).collect();
        segments_overall.extend(segments_reversed);
        println!(
            "Overall: {:?} segments including reversals",
            segments_overall.len()
        );

        let mut used_indices: Vec<usize> = vec![];
        let mut new_rings: Vec<Vec<usize>> = vec![];

        for (seg_idx, s) in segments_overall.iter().enumerate() {
            println!("Starting a loop with segment: {:?}", s);
            let mut new_ring_indices: Vec<usize> = vec![];
            let starting_point = s.get_start();

            let mut next_segment_index: usize = seg_idx;
            for i in 1..segments_overall.len() {
                let next_segment = segments_overall.get(next_segment_index).unwrap();
                println!("next segment: {:?}", next_segment);
                new_ring_indices.push(next_segment_index);

                match find_next_segment_index(&segments_overall, next_segment, &used_indices) {
                    None => {
                        println!("\tno viable next segments!");
                        break;
                    }
                    Some(idx) => next_segment_index = idx,
                }
                if next_segment.get_end() == starting_point {
                    println!("\tomg finished!");
                    println!("\tring indices: {:?}", new_ring_indices);
                    new_rings.push(new_ring_indices.clone());
                    used_indices.extend(new_ring_indices);
                    break;
                }
            }
        }

        new_rings
    }
}

pub fn find_next_segment_index(
    segments: &Vec<Segment>,
    starting_segment: &Segment,
    used_indices: &Vec<usize>,
) -> Option<usize> {
    let mut matches: Vec<usize> = vec![];
    for (idx, s2) in segments.iter().enumerate() {
        if used_indices.contains(&idx) {
            continue;
        }
        if s2.continues(&starting_segment) && !s2.equals_or_reverse_equals(&starting_segment) {
            matches.push(idx);
        }
    }

    if matches.len() == 1 {
        Some(matches[0])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angles_from_points() {
        let o: Point = Point::new(0.0, 0.0, "O");
        let a = Point::new(1.0, 0.0, "A");
        let b = Point::new(1.0, 1.0, "B");
        let c = Point::new(0.0, 1.0, "C");
        let d = Point::new(-1.0, 1.0, "D");
        let e = Point::new(-1.0, 0.0, "E");
        let f = Point::new(-1.0, -1.0, "F");
        let g = Point::new(0.0, -1.0, "G");
        let h = Point::new(1.0, -1.0, "H");

        assert_eq!(angle(&a, &o, &b), PI / 4.0);
        assert_eq!(angle(&a, &o, &c), PI / 2.0);
        assert_eq!(angle(&a, &o, &d), PI / 4.0 * 3.0);
        assert_eq!(angle(&a, &o, &e), PI);
        assert_eq!(angle(&a, &o, &f), PI / 4.0 * 5.0);
        assert_eq!(angle(&a, &o, &g), PI / 2.0 * 3.0);
        assert_eq!(angle(&a, &o, &h), PI / 4.0 * 7.0);
        assert_eq!(angle(&a, &o, &a), TAU);
    }

    #[test]
    fn arc_angles() {
        let a = Point::new(-1.0, 0.0, "A");
        let b = Point::new(1.0, 0.0, "B");
        let c = Point::new(0.0, 1.0, "C");
        let d = Point::new(0.0, -1.0, "D");

        let a1 = Arc::new(a.clone(), b.clone(), c.clone());
        let c1 = a1.to_circle();
        assert_eq!(c1.center.x, 0.0);
        assert_eq!(c1.center.y, 0.0);
        assert_eq!(c1.radius, 1.0);
        assert_eq!(a1.angle(), PI / 2.0);

        let a2 = Arc::new(a.clone(), b.clone(), d.clone());
        let c2 = a2.to_circle();
        assert_eq!(c2.center.x, 0.0);
        assert_eq!(c2.center.y, 0.0);
        assert_eq!(c2.radius, 1.0);
        assert_eq!(a2.angle(), PI / 2.0 * 3.0);

        let a3 = Arc::new(a.clone(), d.clone(), b.clone());
        let c3 = a3.to_circle();
        assert_eq!(c3.center.x, 0.0);
        assert_eq!(c3.center.y, 0.0);
        assert_eq!(c3.radius, 1.0);
        assert_eq!(a3.angle(), PI / 2.0);

        let a4 = Arc::new(a.clone(), d.clone(), c.clone());
        let c4 = a4.to_circle();
        assert_eq!(c4.center.x, 0.0);
        assert_eq!(c4.center.y, 0.0);
        assert_eq!(c4.radius, 1.0);
        assert_eq!(a4.angle(), PI / 2.0);

        let a5 = Arc::new(c.clone(), a.clone(), b.clone());
        let c5 = a5.to_circle();
        assert_eq!(c5.center.x, 0.0);
        assert_eq!(c5.center.y, 0.0);
        assert_eq!(c5.radius, 1.0);
        assert_eq!(a5.angle(), 0.0);
    }

    #[test]
    fn simplest_triangle() {
        let a = Point::new(-1.0, 0.0, "A");
        let b = Point::new(1.0, 0.0, "B");
        let c = Point::new(0.0, 1.0, "C");
        let line_ab = Line::new(a.clone(), b.clone());
        let line_bc = Line::new(b.clone(), c.clone());
        let line_ca = Line::new(c.clone(), a.clone());
        let segments = vec![
            Segment::Line(line_ab),
            Segment::Line(line_bc),
            Segment::Line(line_ca),
        ];
        let sketch1 = Sketch::new(segments);

        println!("{:?}", sketch1.find_faces());
    }
}

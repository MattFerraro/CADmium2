#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::common::{CoordinateFrame, LineFace, LineRing, LineSegment};
use geo::polygon;
use geo::Area;
use geo::Contains;
use geo::LineString;
use geo::Polygon;
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

    pub fn link(points: Vec<Point>, closed: bool) -> Vec<Segment> {
        let mut segments: Vec<Segment> = vec![];
        for i in 0..points.len() - 1 {
            let start = points[i].clone();
            let end = points[i + 1].clone();
            let line = Line::new(start, end);
            segments.push(Segment::Line(line));
        }

        if closed {
            let start = points.last().unwrap().clone();
            let end = points.first().unwrap().clone();
            let line = Line::new(start, end);
            segments.push(Segment::Line(line));
        }
        segments
    }
}

type Ring = Vec<Segment>;

pub fn as_polygon(ring: &Ring) -> Polygon {
    let mut b: Vec<(f64, f64)> = vec![];
    for segment in ring.iter() {
        let start = segment.get_start();
        let start_tuple = (start.x, start.y);
        b.push(start_tuple);
    }
    let polygon = Polygon::new(LineString::from(b), vec![]);
    polygon
}

pub fn signed_area(ring: &Ring) -> f64 {
    let mut area: f64 = 0.0;
    for segment in ring {
        area += (segment.get_end().x - segment.get_start().x)
            * (segment.get_end().y + segment.get_start().y);
    }
    return area / -2.0;
}

pub fn pretty_print(ring: &Ring) {
    for segment in ring {
        print!("{} --> {}, ", segment.get_start().id, segment.get_end().id);
    }
    print!("Area: {}", signed_area(ring));
    println!();
}

pub struct Face {
    pub exterior: Ring,
    pub interiors: Vec<Ring>,
}

#[derive(Debug, Clone)]
pub struct Sketch {
    pub segments: Vec<Segment>,
}

impl Sketch {
    pub fn new() -> Sketch {
        Sketch { segments: vec![] }
    }

    pub fn add_segments(&mut self, segments: Vec<Segment>) {
        self.segments.extend(segments);
    }

    pub fn find_faces(&self, debug: bool) -> Vec<Polygon> {
        let rings = self.find_rings(debug);

        let mut polygons: Vec<Polygon> = rings
            .iter()
            .map(|r| as_polygon(r))
            .filter(|p| p.signed_area() > 0.0)
            .collect();

        // they are already sorted from smallest to largest area
        let mut what_contains_what: Vec<(usize, usize)> = vec![];
        for smaller_polygon_index in 0..polygons.len() - 1 {
            let smaller_polygon = &polygons[smaller_polygon_index];
            println!("Smaller poly area: {:?}", smaller_polygon.signed_area());

            for bigger_polygon_index in smaller_polygon_index + 1..polygons.len() {
                let bigger_polygon = &polygons[bigger_polygon_index];
                let inside = bigger_polygon.contains(smaller_polygon);
                println!(
                    "Bigger poly area: {} contains? {}",
                    bigger_polygon.signed_area(),
                    inside
                );

                if inside {
                    what_contains_what.push((bigger_polygon_index, smaller_polygon_index));
                    break;
                }
            }
        }

        for (bigger_index, smaller_index) in what_contains_what {
            let smaller = &polygons[smaller_index];
            let new_interior_coords: Vec<&geo::Coord> = smaller.exterior().coords().collect();
            let new_interior: Vec<(f64, f64)> =
                new_interior_coords.iter().map(|c| (c.x, c.y)).collect();

            let bigger = &mut polygons[bigger_index];
            bigger.interiors_push(new_interior);
        }

        polygons
    }

    pub fn find_rings(&self, debug: bool) -> Vec<Ring> {
        let mut segments_overall: Vec<Segment> = self.segments.iter().map(|s| s.clone()).collect();
        let segments_reversed: Vec<Segment> = self.segments.iter().map(|s| s.reverse()).collect();
        segments_overall.extend(segments_reversed);

        if debug {
            println!(
                "Overall: {:?} segments including reversals",
                segments_overall.len()
            );
        }

        let mut used_indices: Vec<usize> = vec![];
        let mut new_rings: Vec<Vec<usize>> = vec![];

        for (seg_idx, s) in segments_overall.iter().enumerate() {
            if debug {
                println!("Starting a loop with segment: {:?}", s);
            }
            if used_indices.contains(&seg_idx) {
                if debug {
                    println!("Skipping because it's been used");
                }
                continue;
            }
            let mut new_ring_indices: Vec<usize> = vec![];
            let starting_point = s.get_start();
            if debug {
                println!("Starting point: {:?}", starting_point);
            }

            let mut next_segment_index: usize = seg_idx;
            for i in 1..segments_overall.len() {
                let next_segment = segments_overall.get(next_segment_index).unwrap();
                if debug {
                    println!("next segment: {:?}", next_segment);
                }
                new_ring_indices.push(next_segment_index);

                match find_next_segment_index(&segments_overall, next_segment, &used_indices, debug)
                {
                    None => {
                        if debug {
                            println!("\tno viable next segments!");
                        }
                        break;
                    }
                    Some(idx) => next_segment_index = idx,
                }
                if next_segment.get_end() == starting_point {
                    if debug {
                        println!("\tomg finished!");
                        println!("\tring indices: {:?}", new_ring_indices);
                    }
                    new_rings.push(new_ring_indices.clone());
                    used_indices.extend(new_ring_indices);
                    break;
                }
            }
        }

        let mut all_rings: Vec<Ring> = vec![];
        for ring_indices in new_rings.iter() {
            let mut this_ring: Ring = vec![];
            for segment_index in ring_indices {
                let actual_segment = segments_overall.get(*segment_index).unwrap();
                this_ring.push(actual_segment.clone());
            }
            all_rings.push(this_ring);
        }

        all_rings.sort_by(|r1, r2| signed_area(r1).partial_cmp(&signed_area(r2)).unwrap());

        all_rings
    }

    pub fn create_view(&self, frame: &CoordinateFrame) -> SketchView {
        let mut sv = SketchView {
            segments: vec![],
            faces: vec![],
        };

        for segment in self.segments.iter() {
            let start = segment.get_start();
            let end = segment.get_end();
            let start_3d = frame.to_3d(start);
            let end_3d = frame.to_3d(end);
            let line_segment = LineSegment {
                start: start_3d,
                end: end_3d,
            };
            sv.segments.push(line_segment);
        }

        let faces = self.find_faces(false);
        for face in faces.iter() {
            let mut exterior: LineRing = LineRing::new();
            let mut interiors: Vec<LineRing> = vec![];
            let exterior_coords: Vec<&geo::Coord> = face.exterior().coords().collect();
            let exterior_points: Vec<Point> = exterior_coords
                .iter()
                .map(|c| Point::new(c.x, c.y, "exterior"))
                .collect();
            for i in 0..exterior_points.len() - 1 {
                let start = exterior_points[i].clone();
                let end = exterior_points[i + 1].clone();
                let start_3d = frame.to_3d(start);
                let end_3d = frame.to_3d(end);
                let line_segment = LineSegment {
                    start: start_3d,
                    end: end_3d,
                };
                exterior.add_segment(line_segment);
            }

            sv.faces.push(LineFace {
                exterior: exterior,
                interiors: vec![],
            });

            for interior in face.interiors() {
                let interior_coords: Vec<&geo::Coord> = interior.coords().collect();
                let interior_points: Vec<Point> = interior_coords
                    .iter()
                    .map(|c| Point::new(c.x, c.y, "interior"))
                    .collect();
                let mut interior_ring: LineRing = LineRing::new();
                for i in 0..interior_points.len() - 1 {
                    let start = interior_points[i].clone();
                    let end = interior_points[i + 1].clone();
                    let start_3d = frame.to_3d(start);
                    let end_3d = frame.to_3d(end);
                    let line_segment = LineSegment {
                        start: start_3d,
                        end: end_3d,
                    };
                    interior_ring.add_segment(line_segment);
                }
                interiors.push(interior_ring);
            }
            sv.faces.last_mut().unwrap().interiors = interiors;
        }

        sv
    }
}

#[derive(Debug, Clone)]
pub struct SketchView {
    pub segments: Vec<LineSegment>,
    pub faces: Vec<LineFace>,
}

pub fn find_next_segment_index(
    segments: &Vec<Segment>,
    starting_segment: &Segment,
    used_indices: &Vec<usize>,
    debug: bool,
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

    if matches.len() == 0 {
        None
    } else if matches.len() == 1 {
        Some(matches[0])
    } else {
        if debug {
            println!("\tMultiple options! Deciding which one to take...");
        }
        let point_a = starting_segment.get_start();
        let point_b = starting_segment.get_end();

        let mut best_option: usize = 0;
        let mut biggest_angle: f64 = 0.0;
        for option in matches {
            let point_c = segments[option].get_end();
            let ang = angle(&point_a, &point_b, &point_c);
            if debug {
                println!(
                    "\tAngle from {} to {} to {}: {}",
                    point_a.id,
                    point_b.id,
                    point_c.id,
                    ang * 180.0 / 3.1415926
                );
            }
            if ang >= biggest_angle {
                biggest_angle = ang;
                best_option = option;
            }
        }

        Some(best_option)
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
    fn simplest_triangle_rings() {
        /*
          C
         / \
        A---B
         */
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
        let mut sketch1 = Sketch::new();
        sketch1.add_segments(segments);

        let rings = sketch1.find_rings(false);
        assert_eq!(rings.len(), 2);
        assert_eq!(rings[0].len(), 3);
        assert_eq!(rings[1].len(), 3);
    }

    #[test]
    fn double_triangles_rings() {
        /*
          C
         / \
        A---B
         \ /
          D
         */
        let a = Point::new(-1.0, 0.0, "A");
        let b = Point::new(1.0, 0.0, "B");
        let c = Point::new(0.0, 1.0, "C");
        let d = Point::new(0.0, -1.0, "D");
        let line_ab = Line::new(a.clone(), b.clone());
        let line_bc = Line::new(b.clone(), c.clone());
        let line_ca = Line::new(c.clone(), a.clone());
        let line_ad = Line::new(a.clone(), d.clone());
        let line_bd = Line::new(b.clone(), d.clone());
        let segments = vec![
            Segment::Line(line_ab),
            Segment::Line(line_bc),
            Segment::Line(line_ca),
            Segment::Line(line_ad),
            Segment::Line(line_bd),
        ];
        let mut sketch1 = Sketch::new();
        sketch1.add_segments(segments);

        let rings = sketch1.find_rings(false);
        assert_eq!(rings.len(), 3);
        println!("\nAbout to find faces for triangles");
        let faces = sketch1.find_faces(false);

        for f in faces {
            println!("Found Face: {:?}", f);
        }
    }

    #[test]
    fn nested_squares() {
        /*
        H-----------G
        |           |
        |   D---C   |
        |   |   |   |
        |   A---B   |
        |           |
        E-----------F
         */
        let a = Point::new(0.0, 0.0, "A");
        let b = Point::new(1.0, 0.0, "B");
        let c = Point::new(1.0, 1.0, "C");
        let d = Point::new(0.0, 1.0, "D");
        let line_ab = Line::new(a.clone(), b.clone());
        let line_bc = Line::new(b.clone(), c.clone());
        let line_cd = Line::new(c.clone(), d.clone());
        let line_da = Line::new(d.clone(), a.clone());

        let e = Point::new(-1.0, -1.0, "E");
        let f = Point::new(2.0, -1.0, "F");
        let g = Point::new(2.0, 2.0, "G");
        let h = Point::new(-1.0, 2.0, "H");
        let line_ef = Line::new(e.clone(), f.clone());
        let line_fg = Line::new(f.clone(), g.clone());
        let line_gh = Line::new(g.clone(), h.clone());
        let line_he = Line::new(h.clone(), e.clone());
        let segments = vec![
            Segment::Line(line_ab),
            Segment::Line(line_bc),
            Segment::Line(line_cd),
            Segment::Line(line_da),
            Segment::Line(line_ef),
            Segment::Line(line_fg),
            Segment::Line(line_gh),
            Segment::Line(line_he),
        ];
        let mut sketch1 = Sketch::new();
        sketch1.add_segments(segments);

        let rings = sketch1.find_rings(false);
        for ring in rings.iter() {
            pretty_print(&ring);
        }
        assert_eq!(rings.len(), 4);

        println!("\nAbout to find faces for squares");
        let faces = sketch1.find_faces(false);

        for f in faces {
            println!("Found Face: {:?}", f);
        }
    }

    #[test]
    fn double_nested_squares() {
        /*
        L-------------------K
        |                   |
        |   H-----------G   |
        |   |           |   |
        |   |   D---C   |   |
        |   |   |   |   |   |
        |   |   A---B   |   |
        |   |           |   |
        |   E-----------F   |
        |                   |
        I-------------------J
         */
        let a = Point::new(0.0, 0.0, "A");
        let b = Point::new(1.0, 0.0, "B");
        let c = Point::new(1.0, 1.0, "C");
        let d = Point::new(0.0, 1.0, "D");
        let line_ab = Line::new(a.clone(), b.clone());
        let line_bc = Line::new(b.clone(), c.clone());
        let line_cd = Line::new(c.clone(), d.clone());
        let line_da = Line::new(d.clone(), a.clone());

        let e = Point::new(-1.0, -1.0, "E");
        let f = Point::new(2.0, -1.0, "F");
        let g = Point::new(2.0, 2.0, "G");
        let h = Point::new(-1.0, 2.0, "H");
        let line_ef = Line::new(e.clone(), f.clone());
        let line_fg = Line::new(f.clone(), g.clone());
        let line_gh = Line::new(g.clone(), h.clone());
        let line_he = Line::new(h.clone(), e.clone());

        let i = Point::new(-2.0, -2.0, "I");
        let j = Point::new(3.0, -2.0, "J");
        let k = Point::new(3.0, 3.0, "K");
        let l = Point::new(-2.0, 3.0, "L");
        let line_ij = Line::new(i.clone(), j.clone());
        let line_jk = Line::new(j.clone(), k.clone());
        let line_kl = Line::new(k.clone(), l.clone());
        let line_li = Line::new(l.clone(), i.clone());

        let segments = vec![
            Segment::Line(line_ab),
            Segment::Line(line_bc),
            Segment::Line(line_cd),
            Segment::Line(line_da),
            Segment::Line(line_ef),
            Segment::Line(line_fg),
            Segment::Line(line_gh),
            Segment::Line(line_he),
            Segment::Line(line_ij),
            Segment::Line(line_jk),
            Segment::Line(line_kl),
            Segment::Line(line_li),
        ];
        let mut sketch1 = Sketch::new();
        sketch1.add_segments(segments);

        let rings = sketch1.find_rings(false);
        for ring in rings.iter() {
            pretty_print(&ring);
        }
        assert_eq!(rings.len(), 6);

        println!("\nAbout to find faces for DOUBLE squares");
        let faces = sketch1.find_faces(false);

        for f in faces {
            println!("Found Face: {:?}", f);
        }
    }
}

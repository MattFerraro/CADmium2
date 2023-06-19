use cadmium::sketch as cad_sketch;
use cadmium::{self};
use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::common::{CoordinateFrame, LineFace, LineSegment};

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into())
//     }
// }

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Point2D(cadmium::sketch::Point);

#[wasm_bindgen]
impl Point2D {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D(cadmium::sketch::Point::new(x, y, "a"))
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.0.x
    }
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.0.y
    }

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Line(cadmium::sketch::Line);

#[wasm_bindgen]
impl Line {
    #[wasm_bindgen(constructor)]
    pub fn new(start: &Point2D, end: &Point2D) -> Line {
        let sketch_line = cadmium::sketch::Line::new(start.0.clone(), end.0.clone());
        Line(sketch_line)
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> Point2D {
        Point2D(self.0.start.clone())
    }
    #[wasm_bindgen(getter)]
    pub fn end(&self) -> Point2D {
        Point2D(self.0.end.clone())
    }

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Ring(cadmium::sketch::Ring);

#[wasm_bindgen]
impl Ring {
    #[wasm_bindgen(getter)]
    pub fn segments(&self) -> Array {
        let retval = Array::new();
        for segment in self.0.iter() {
            let wrapped = Segment::wrap(segment);
            retval.push(&JsValue::from(wrapped));
        }
        retval
    }

    #[wasm_bindgen]
    pub fn num_segments(&self) -> usize {
        self.0.len()
    }

    #[wasm_bindgen]
    pub fn get_segment(&self, index: u32) -> Line {
        match self.0.get(index as usize) {
            Some(seg) => match seg {
                cadmium::sketch::Segment::Line(line) => Line(line.clone()),
                cadmium::sketch::Segment::Arc(arc) => {
                    let start = Point2D(arc.start.clone());
                    let end = Point2D(arc.end.clone());
                    let line = cadmium::sketch::Line::new(start.0.clone(), end.0.clone());
                    Line(line)
                }
            },
            None => panic!("index out of bounds"),
        }
    }

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        let mut strings: Vec<String> = vec![];
        for seg in self.0.iter() {
            match seg {
                cadmium::sketch::Segment::Line(line) => {
                    strings.push(line.start.to_string());
                }
                cadmium::sketch::Segment::Arc(arc) => {
                    strings.push(arc.start.to_string());
                }
            }
        }

        strings.join(", ")
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Face(cadmium::sketch::Face);

#[wasm_bindgen]
impl Face {
    #[wasm_bindgen(getter)]
    pub fn exterior(&self) -> Ring {
        Ring(self.0.exterior.clone())
    }

    #[wasm_bindgen(getter)]
    pub fn interiors(&self) -> Array {
        let retval = Array::new();
        for ring in self.0.interiors.iter() {
            retval.push(&JsValue::from(Ring(ring.clone())));
        }
        retval
    }
}

impl Face {
    pub fn wrap(wb: &cad_sketch::Face) -> Face {
        Face(wb.to_owned())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Segment(cadmium::sketch::Segment);

impl Segment {
    pub fn wrap(wb: &cad_sketch::Segment) -> Segment {
        Segment(wb.to_owned())
    }
}

#[wasm_bindgen]
impl Segment {
    #[wasm_bindgen(getter)]
    pub fn start(&self) -> crate::common::Point {
        let start = self.0.get_start();
        crate::common::Point::new(start.x, start.y, f64::NAN)
    }
    #[wasm_bindgen(getter)]
    pub fn end(&self) -> crate::common::Point {
        let end = self.0.get_end();
        crate::common::Point::new(end.x, end.y, f64::NAN)
    }
}

#[wasm_bindgen]
pub struct Sketch(cadmium::sketch::Sketch);

#[wasm_bindgen]
impl Sketch {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Sketch {
        Sketch(cadmium::sketch::Sketch::new())
    }

    #[wasm_bindgen]
    pub fn add_segment(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let start = cadmium::sketch::Point::new(x1, y1, "a");
        let end = cadmium::sketch::Point::new(x2, y2, "a");
        let line = cadmium::sketch::Line::new(start, end);
        let as_seg = cadmium::sketch::Segment::Line(line);
        self.0.segments.push(as_seg);
    }

    #[wasm_bindgen]
    pub fn add_line(&mut self, line: Line) {
        let as_seg = cadmium::sketch::Segment::Line(line.0.clone());
        self.0.segments.push(as_seg);
    }

    #[wasm_bindgen]
    pub fn find_rings(&self) -> Array {
        let rings = self.0.find_rings(false);
        let retval = Array::new();
        for ring in rings.iter() {
            retval.push(&JsValue::from(Ring(ring.clone())));
        }
        retval
    }

    #[wasm_bindgen]
    pub fn find_faces(&self) -> Array {
        let faces = self.0.find_faces(false);
        let retval = Array::new();
        for face in faces.iter() {
            retval.push(&JsValue::from(Face(face.clone())));
        }
        retval
    }
}

#[wasm_bindgen]
pub struct SketchView(cadmium::sketch::SketchView);

impl SketchView {
    pub fn wrap(wb: &cad_sketch::SketchView) -> SketchView {
        SketchView(wb.to_owned())
    }
}

#[wasm_bindgen]
impl SketchView {
    #[wasm_bindgen(getter)]
    pub fn segments(&self) -> Array {
        let retval = Array::new();
        for segment in self.0.segments.iter() {
            let wrapped = LineSegment::wrap(*segment);
            retval.push(&JsValue::from(wrapped));
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn faces(&self) -> Array {
        let retval = Array::new();
        for face in self.0.faces.iter() {
            let wrapped = LineFace::wrap(face.clone());
            retval.push(&JsValue::from(wrapped));
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn faces_2d(&self) -> Array {
        let retval = Array::new();
        for face in self.0.faces_2d.iter() {
            let wrapped = Face::wrap(face);
            retval.push(&JsValue::from(wrapped));
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn segments_2d(&self) -> Array {
        let retval = Array::new();
        for segment in self.0.segments_2d.iter() {
            let wrapped = Segment::wrap(segment);
            retval.push(&JsValue::from(wrapped));
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn coordinate_frame(&self) -> CoordinateFrame {
        CoordinateFrame::wrap(self.0.coordinate_frame)
    }
}

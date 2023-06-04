use cadmium::sketch as cad_sketch;
use cadmium::{self};
use js_sys::Array;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

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
    #[wasm_bindgen]
    pub fn num_interiors(&self) -> u32 {
        self.0.interiors.len() as u32
    }
    #[wasm_bindgen(getter)]
    pub fn exterior(&self) -> Ring {
        Ring(self.0.exterior.clone())
    }
    #[wasm_bindgen]
    pub fn get_interior(&self, index: u32) -> Ring {
        Ring(self.0.interiors[index as usize].clone())
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
    pub fn add_segments(&mut self, segments: Array) {
        for seg in segments.iter() {
            log!("segment: {:?}", seg);
            // let line = seg.dyn_ref::<Line>();
            // let line = seg.dyn_ref::<Line>();

            // match line {
            //     Some(line) => {
            //         let as_seg = cadmium::sketch::Segment::Line(line.0.clone());
            //         self.0.segments.push(as_seg);
            //     }
            //     None => {
            //         let arc = seg.dyn_ref::<Arc>();
            //         match arc {
            //             Some(arc) => {
            //                 let as_seg = cadmium::sketch::Segment::Arc(arc.0.clone());
            //                 self.0.segments.push(as_seg);
            //             }
            //             None => panic!("unknown segment type"),
            //         }
            //     }
            // }
        }
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

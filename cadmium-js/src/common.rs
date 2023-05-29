use wasm_bindgen::prelude::*;

use cadmium::{self};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Point(cadmium::common::Point);

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point(cadmium::common::Point::new(x, y, z))
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.0.x
    }
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.0.y
    }
    #[wasm_bindgen(getter)]
    pub fn z(&self) -> f64 {
        self.0.z
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct UV(cadmium::common::UV);

#[wasm_bindgen]
impl UV {
    #[wasm_bindgen(constructor)]
    pub fn new(u: f64, v: f64) -> UV {
        UV(cadmium::common::UV::new(u, v))
    }

    #[wasm_bindgen(getter)]
    pub fn u(&self) -> f64 {
        self.0.u
    }
    #[wasm_bindgen(getter)]
    pub fn v(&self) -> f64 {
        self.0.v
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Vector(cadmium::common::Vector);

#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector(cadmium::common::Vector::new(x, y, z))
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.0.x
    }
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.0.y
    }
    #[wasm_bindgen(getter)]
    pub fn z(&self) -> f64 {
        self.0.z
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CoordinateFrame(cadmium::common::CoordinateFrame);

#[wasm_bindgen]
impl CoordinateFrame {
    #[wasm_bindgen(getter)]
    pub fn origin(&self) -> Point {
        Point(self.0.origin)
    }

    #[wasm_bindgen(getter)]
    pub fn x_axis(&self) -> Vector {
        Vector(self.0.x_axis)
    }

    #[wasm_bindgen(getter)]
    pub fn y_axis(&self) -> Vector {
        Vector(self.0.y_axis)
    }

    #[wasm_bindgen(getter)]
    pub fn normal(&self) -> Vector {
        Vector(self.0.normal)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct LineSegment(cadmium::common::LineSegment);
#[wasm_bindgen]
impl LineSegment {
    #[wasm_bindgen(getter)]
    pub fn start(&self) -> Point {
        Point(self.0.start)
    }
    #[wasm_bindgen(getter)]
    pub fn end(&self) -> Point {
        Point(self.0.end)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct LineRing(cadmium::common::LineRing);
#[wasm_bindgen]
impl LineRing {
    #[wasm_bindgen]
    pub fn num_segments(&self) -> u32 {
        self.0.segments.len() as u32
    }
    #[wasm_bindgen]
    pub fn get_segment(&self, index: u32) -> LineSegment {
        LineSegment(self.0.segments[index as usize])
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct LineFace(cadmium::common::LineFace);
#[wasm_bindgen]
impl LineFace {
    #[wasm_bindgen]
    pub fn num_interiors(&self) -> u32 {
        self.0.interiors.len() as u32
    }
    #[wasm_bindgen(getter)]
    pub fn exterior(&self) -> LineRing {
        LineRing(self.0.exterior.clone())
    }
    #[wasm_bindgen]
    pub fn get_interior(&self, index: u32) -> LineRing {
        LineRing(self.0.interiors[index as usize].clone())
    }
}

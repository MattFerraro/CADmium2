use js_sys::Array;
use wasm_bindgen::prelude::*;

use cadmium::{self};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
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

// impl From<cadmium::common::Point> for Point {
//     fn from(point: cadmium::common::Point) -> Point {
//         Point(point)
//     }
// }

impl Point {
    pub fn wrap(point: cadmium::common::Point) -> Point {
        Point(point)
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

impl UV {
    pub fn wrap(point: cadmium::common::UV) -> UV {
        UV(point)
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

impl Vector {
    pub fn wrap(point: cadmium::common::Vector) -> Vector {
        Vector(point)
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

impl CoordinateFrame {
    pub fn wrap(point: cadmium::common::CoordinateFrame) -> CoordinateFrame {
        CoordinateFrame(point)
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

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Plane(cadmium::common::Plane);

#[wasm_bindgen]
impl Plane {
    #[wasm_bindgen(constructor)]
    pub fn new(origin: Point, x: Vector, y: Vector, normal: Vector) -> Plane {
        Plane(cadmium::common::Plane::new(origin.0, x.0, y.0, normal.0))
    }

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

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f64 {
        self.0.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f64 {
        self.0.height
    }

    #[wasm_bindgen]
    pub fn get_mesh(&self) -> Mesh {
        Mesh(self.0.get_mesh())
    }

    #[wasm_bindgen]
    pub fn get_upper_left(&self) -> Point {
        Point(self.0.get_upper_left())
    }

    #[wasm_bindgen]
    pub fn get_up(&self) -> Vector {
        Vector(self.0.get_up())
    }
}

impl Plane {
    pub fn wrap(point: cadmium::common::Plane) -> Plane {
        Plane(point)
    }
}

#[wasm_bindgen]
pub struct Solid(cadmium::common::Solid);

impl Solid {
    pub fn wrap(point: cadmium::common::Solid) -> Solid {
        Solid(point)
    }
}

#[wasm_bindgen]
impl Solid {
    pub fn get_mesh(&self) -> Mesh {
        Mesh(self.0.get_mesh())
    }
}

#[wasm_bindgen]
pub struct Mesh(cadmium::common::Mesh);

#[wasm_bindgen]
impl Mesh {
    #[wasm_bindgen(getter)]
    pub fn vertices(&self) -> Array {
        let retval = Array::new();
        for vertex in self.0.vertices.iter() {
            retval.push(&JsValue::from(Point::wrap(*vertex)));
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn normals(&self) -> Array {
        let retval = Array::new();

        // let mut index = 0;
        // while index < self.0.indices.len() {
        //     let a_idx = self.0.indices[index];
        //     let b_idx = self.0.indices[index + 1];
        //     let c_idx = self.0.indices[index + 2];

        //     let a_x = self.0.vertices[a_idx].x;
        //     let a_y = self.0.vertices[a_idx].y;
        //     let a_z = self.0.vertices[a_idx].z;
        //     let a_vec = cadmium::common::Vector::new(a_x, a_y, a_z);

        //     let b_x = self.0.vertices[b_idx].x;
        //     let b_y = self.0.vertices[b_idx].y;
        //     let b_z = self.0.vertices[b_idx].z;
        //     let b_vec = cadmium::common::Vector::new(b_x, b_y, b_z);

        //     let c_x = self.0.vertices[c_idx].x;
        //     let c_y = self.0.vertices[c_idx].y;
        //     let c_z = self.0.vertices[c_idx].z;
        //     let c_vec = cadmium::common::Vector::new(c_x, c_y, c_z);

        //     let ab = b_vec.subtract(a_vec);
        //     let ac = c_vec.subtract(a_vec);

        //     let normal = ab.cross(ac).normalize();

        //     index += 3;
        // }

        for normal in self.0.normals.iter() {
            retval.push(&JsValue::from(Vector::wrap(*normal)));
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn indices(&self) -> Array {
        let retval = Array::new();
        for index in self.0.indices.iter() {
            retval.push(&JsValue::from_f64(*index as f64));
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn uvs(&self) -> Array {
        let retval = Array::new();
        for uv in self.0.uvs.iter() {
            retval.push(&JsValue::from(UV::wrap(*uv)));
        }
        retval
    }
}

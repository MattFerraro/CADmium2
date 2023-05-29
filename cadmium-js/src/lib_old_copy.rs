use cadmium::workbench;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // alert(&format!("Hello, {}!", name));
    log!("Hello from {}", name)
}

#[wasm_bindgen]
pub fn new_part_studio() {
    let proj = cadmium::project::Project::new("matt");
    log!("Project: {:?}", proj);
    log!("WB View: {:?}", proj.workbenches[0].create_view(100));
}

#[derive(Serialize, Deserialize)]
pub struct WorkbenchView {
    pub points: HashMap<String, Point>,
    pub planes: HashMap<String, Plane>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}
#[wasm_bindgen]
impl LineSegment {
    #[wasm_bindgen(constructor)]
    pub fn new(start: Point, end: Point) -> Self {
        LineSegment { start, end }
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Plane {
    pub frame: CoordinateFrame,
    name: String,
}
#[wasm_bindgen]
impl Plane {
    #[wasm_bindgen(constructor)]
    pub fn new(frame: CoordinateFrame, name: String) -> Self {
        Plane {
            frame,
            name: name.to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CoordinateFrame {
    pub origin: Point,
    pub x_axis: Vector,
    pub y_axis: Vector,
    pub normal: Vector,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
    name: String,
}

#[wasm_bindgen]
impl Color {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f64, g: f64, b: f64, a: f64, name: String) -> Color {
        Color {
            r,
            g,
            b,
            a,
            name: name.to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
    pub field4: String,
}

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));
    let example = Example {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: [1., 2., 3., 4.],
        field4: "hello".to_string(),
    };

    serde_wasm_bindgen::to_value(&example).unwrap()
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
#[wasm_bindgen]
pub struct Workbench(workbench::Workbench);

#[wasm_bindgen]
pub fn new_workbench() -> Workbench {
    let workbench = workbench::Workbench::new("matt");
    Workbench(workbench)
}

#[wasm_bindgen]
impl Workbench {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.0.name.to_owned()
    }
}

// #[wasm_bindgen]
// pub fn new_workbench() -> JsValue {
// let workbench = Workbench::new(name);

// let workbench_binding = Workbench::from_workbench(workbench);
// let workbench_binding = Workbench {
//     field4: "wbb1".to_string(),
// };
// serde_wasm_bindgen::to_value(&workbench_binding).unwrap()
// }

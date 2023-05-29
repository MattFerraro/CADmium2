use cadmium::project;
use cadmium::workbench;
use js_sys::Array;

use cadmium;
mod common;
mod sketch;

// use serde::{Deserialize, Serialize};
// use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::prelude::*;
extern crate js_sys;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into())
//     }
// }

// pub trait IntoWasm: Sized {
//     /// wasm wrapper struct
//     type WasmWrapper: From<Self>;
//     /// Into wasm wrapper
//     fn into_wasm(self) -> Self::WasmWrapper {
//         self.into()
//     }
// }

// impl From<common::Point> for Point {
//     fn from(point: common::Point) -> Point {
//         Point(point)
//     }
// }

// impl IntoWasm for common::Point {
//     type WasmWrapper = Point;
// }

#[derive(Debug)]
#[wasm_bindgen]
pub struct Workbench(workbench::Workbench);

#[wasm_bindgen]
impl Workbench {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.0.name.to_owned()
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Project(project::Project);

#[wasm_bindgen]
pub fn new_project() -> Project {
    let project = project::Project::new("project0");
    Project(project)
}

#[wasm_bindgen]
impl Project {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.0.name.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn workbench_names(&self) -> Array {
        let wbs: Vec<String> = self
            .0
            .workbenches
            .iter()
            .map(|wb| wb.name.clone())
            .collect();
        let retval = Array::new();
        for wb in wbs.iter() {
            retval.push(&JsValue::from(wb));
        }
        retval
    }

    #[wasm_bindgen]
    pub fn get_workbench(&self, name: &str) -> Option<Workbench> {
        let wb = self.0.get_workbench(name);
        match wb {
            Some(wb) => Some(Workbench(wb.clone())),
            None => None,
        }
    }
}

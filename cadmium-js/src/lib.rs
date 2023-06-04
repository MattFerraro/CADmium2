// use cadmium::project;
// use cadmium::workbench;
// use js_sys::Array;

// use cadmium;
mod common;
mod project;
mod sketch;
mod workbench;

// use serde::{Deserialize, Serialize};
// use wasm_bindgen::convert::IntoWasmAbi;
// use wasm_bindgen::prelude::*;
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

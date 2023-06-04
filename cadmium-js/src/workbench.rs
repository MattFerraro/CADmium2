use crate::common::{Plane, Point};
use cadmium::workbench as cad_workbench;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Workbench(cad_workbench::Workbench);

#[wasm_bindgen]
impl Workbench {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.0.name.to_owned()
    }

    #[wasm_bindgen]
    pub fn get_steps(&self) -> Array {
        let retval = Array::new();
        for step in self.0.steps.iter() {
            // retval.push(&JsValue::from(step));
            match step {
                cad_workbench::Step::Point { name, point } => {
                    let new_point_step = NewPointStep {
                        name: name.to_owned(),
                        point: Point::wrap(*point),
                    };
                    retval.push(&JsValue::from(new_point_step));
                }
                cad_workbench::Step::Plane { name, plane } => {
                    let new_plane_step = NewPlaneStep {
                        name: name.to_owned(),
                        plane: Plane::wrap(*plane),
                    };
                    retval.push(&JsValue::from(new_plane_step));
                }
                cad_workbench::Step::Sketch {
                    name,
                    sketch: _,
                    plane: _,
                } => {
                    let new_sketch_step = NewSketchStep {
                        name: name.to_owned(),
                    };
                    retval.push(&JsValue::from(new_sketch_step));
                }
                cad_workbench::Step::Extrusion {
                    name,
                    extrusion: _,
                    sketch: _,
                    faces: _,
                } => {
                    let new_extrude_step = NewExtrudeStep {
                        name: name.to_owned(),
                    };
                    retval.push(&JsValue::from(new_extrude_step));
                }
            }
        }
        retval
    }
}

impl Workbench {
    pub fn wrap(wb: &cad_workbench::Workbench) -> Workbench {
        Workbench(wb.to_owned())
    }
}

#[wasm_bindgen]
pub struct NewPointStep {
    name: String,
    pub point: Point,
}

#[wasm_bindgen]
impl NewPointStep {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}

#[wasm_bindgen]
pub struct NewPlaneStep {
    name: String,
    pub plane: Plane,
}
#[wasm_bindgen]
impl NewPlaneStep {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}

#[wasm_bindgen]
pub struct NewSketchStep {
    name: String,
}
#[wasm_bindgen]
impl NewSketchStep {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}

#[wasm_bindgen]
pub struct NewExtrudeStep {
    name: String,
}
#[wasm_bindgen]
impl NewExtrudeStep {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}

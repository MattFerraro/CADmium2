use crate::common::{Plane, Point, Solid};
use crate::sketch;
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
                    extrusion: ext,
                    sketch,
                    faces,
                } => {
                    let new_extrude_step = NewExtrudeStep {
                        name: name.to_owned(),
                        depth: ext.depth,
                        direction: ext.direction,
                        operation: ext.operation.to_string(),
                        faces: faces.to_vec(),
                        sketch: sketch.to_owned(),
                    };
                    retval.push(&JsValue::from(new_extrude_step));
                }
            }
        }
        retval
    }

    #[wasm_bindgen]
    pub fn create_view(&self, max_steps: usize) -> WorkbenchView {
        let wbv = self.0.create_view(max_steps);
        WorkbenchView(wbv)
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
    pub depth: f64,
    direction: cadmium::common::Vector,
    operation: String,
    faces: Vec<usize>,
    sketch: String,
}
#[wasm_bindgen]
impl NewExtrudeStep {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn direction(&self) -> crate::common::Vector {
        crate::common::Vector::wrap(self.direction)
    }

    #[wasm_bindgen(getter)]
    pub fn operation(&self) -> String {
        self.operation.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn sketch(&self) -> String {
        self.sketch.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn faces(&self) -> Array {
        let retval = Array::new();
        for face_index in self.faces.iter() {
            retval.push(&JsValue::from(*face_index));
        }
        retval
    }
}

#[wasm_bindgen]
pub struct WorkbenchView(cad_workbench::WorkbenchView);

#[wasm_bindgen]
impl WorkbenchView {
    #[wasm_bindgen(getter)]
    pub fn points(&self) -> Array {
        let retval = Array::new();
        for (name, point) in self.0.points.iter() {
            let js_map = js_sys::Map::new();
            js_map.set(&JsValue::from("name"), &JsValue::from(name.to_owned()));
            js_map.set(&JsValue::from("point"), &JsValue::from(Point::wrap(*point)));
            retval.push(&js_map);
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn planes(&self) -> Array {
        let retval = Array::new();
        for (name, plane) in self.0.planes.iter() {
            let js_map = js_sys::Map::new();
            js_map.set(&JsValue::from("name"), &JsValue::from(name.to_owned()));
            js_map.set(&JsValue::from("plane"), &JsValue::from(Plane::wrap(*plane)));
            retval.push(&js_map);
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn sketches(&self) -> Array {
        let retval = Array::new();
        for (name, sketch) in self.0.sketches.iter() {
            let js_map = js_sys::Map::new();
            js_map.set(&JsValue::from("name"), &JsValue::from(name.to_owned()));
            js_map.set(
                &JsValue::from("sketch"),
                &JsValue::from(sketch::SketchView::wrap(sketch)),
            );
            retval.push(&js_map);
        }
        retval
    }

    #[wasm_bindgen(getter)]
    pub fn solids(&self) -> Array {
        let retval = Array::new();
        for (name, solid) in self.0.solids.iter() {
            let js_map = js_sys::Map::new();
            js_map.set(&JsValue::from("name"), &JsValue::from(name.to_owned()));
            js_map.set(
                &JsValue::from("solid"),
                &JsValue::from(Solid::wrap(solid.clone())),
            );
            retval.push(&js_map);
        }
        retval
    }
}

// use crate::workbench::Workbench;
use cadmium::project as cad_project;
// use cadmium::workbench as cad_workbench;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Project(cad_project::Project);

#[wasm_bindgen]
pub fn new_project() -> Project {
    let project = cad_project::Project::new("First Project");
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

    // #[wasm_bindgen]
    // pub fn get_workbench(&self, name: &str) -> Option<cad_workbench::Workbench> {
    //     let wb = self.0.get_workbench(name);
    //     match wb {
    //         Some(wb) => Some(Workbench(wb.clone())),
    //         None => None,
    //     }
    // }
}

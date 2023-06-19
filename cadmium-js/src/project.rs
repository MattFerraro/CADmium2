// use crate::workbench::Workbench;
use cadmium::project as cad_project;
// use cadmium::workbench as cad_workbench;
use crate::workbench::Workbench;
use js_sys::Array;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Project(cad_project::Project);

#[wasm_bindgen]
pub fn new_project() -> Project {
    let project = cad_project::Project::new("First Project", true);
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
        let wb_option = self.0.get_workbench(name);
        match wb_option {
            Some(wb) => Some(Workbench::wrap(wb)),
            None => None,
        }
    }

    #[wasm_bindgen]
    pub fn add_segment_to_sketch(
        &mut self,
        workbench_name: String,
        sketch_name: String,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) {
        self.0
            .add_segment_to_sketch(&workbench_name, &sketch_name, x1, y1, x2, y2)
            .unwrap();
    }

    #[wasm_bindgen]
    pub fn set_step_parameters(
        &mut self,
        workbench_name: &str,
        step_name: &str,
        parameter_names: Array,
        parameter_values: Array,
    ) {
        log!("Param names: {:?}", parameter_names);
        log!("Param values: {:?}", parameter_values);
        let parameter_names: Vec<String> = parameter_names
            .iter()
            .map(|name| name.as_string().unwrap())
            .collect();
        let parameter_values: Vec<f64> = parameter_values
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect();

        log!("Rust: wb name: {} step: {}", workbench_name, step_name);
        log!("Param names: {:?}", parameter_names);
        log!("Param values: {:?}", parameter_values);
        // for (i, name) in parameter_names.iter().enumerate() {
        //     log!("Rust: param name: {} value: {}", name, parameter_values[i]);
        // }
        self.0
            .set_step_parameters(workbench_name, step_name, parameter_names, parameter_values)
            .unwrap();
    }
}

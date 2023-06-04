use cadmium::workbench as cad_workbench;
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

    // pub fn new(wb: &cad_workbench::Workbench) -> Workbench {
    //     Workbench(wb)
    // }
}

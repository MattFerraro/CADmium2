use crate::assembly::Assembly;
use crate::workbench::Workbench;

// A Project is the overall thing. It contains many workbenches and assemblies

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub workbenches: Vec<Workbench>,
    pub assemblies: Vec<Assembly>,
}

impl Project {
    pub fn new(name: &str) -> Project {
        let mut p = Project {
            name: name.to_owned(),
            workbenches: vec![],
            assemblies: vec![],
        };

        let mut bench0 = Workbench::new("workbench0");
        bench0.add_sketch_and_extrusion();
        p.workbenches.push(bench0);

        p
    }

    pub fn get_workbench(&self, name: &str) -> Option<&Workbench> {
        for wb in self.workbenches.iter() {
            if wb.name == name {
                return Some(wb);
            }
        }
        None
    }
}

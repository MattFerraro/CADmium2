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

        let bench0 = Workbench::new("workbench1");
        p.workbenches.push(bench0);

        p
    }
}

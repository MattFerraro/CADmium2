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
    pub fn new(name: &str, add_dummy_data: bool) -> Project {
        let mut p = Project {
            name: name.to_owned(),
            workbenches: vec![],
            assemblies: vec![],
        };

        let mut bench0 = Workbench::new("Workbench 1");

        if add_dummy_data {
            bench0.add_sketch_and_extrusion();
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        let p = Project::new("Project 1", true);
        assert_eq!(p.name, "Project 1");
        assert_eq!(p.workbenches.len(), 1);
        assert_eq!(p.assemblies.len(), 0);
    }

    #[test]
    fn test_project_makes_shapes() {
        let p = Project::new("Project 1", true);
        let wb = p.get_workbench("Workbench 1").unwrap();
        let wbv = wb.create_view(100);
        let solid = wbv.solids.get("Extrude 1_0").unwrap();
        let as_mesh = solid.get_mesh();
    }
}

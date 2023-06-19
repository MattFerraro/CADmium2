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

    pub fn set_step_parameters(
        &mut self,
        workbench_name: &str,
        step_name: &str,
        parameter_names: Vec<String>,
        parameter_values: Vec<f64>,
    ) -> Result<(), String> {
        // let wb = self
        //     .get_workbench(workbench_name)
        //     .ok_or(format!("No workbench named {}", workbench_name))?;
        for wb in self.workbenches.iter_mut() {
            if wb.name == workbench_name {
                return wb.set_step_parameters(step_name, parameter_names, parameter_values);
            }
        }

        return Ok(());
    }

    pub fn add_segment_to_sketch(
        &mut self,
        workbench_name: &str,
        sketch_name: &str,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) -> Result<(), String> {
        for wb in self.workbenches.iter_mut() {
            if wb.name == workbench_name {
                return wb.add_segment_to_sketch(sketch_name, x1, y1, x2, y2);
            }
        }

        return Ok(());

        // .ok_or(format!("No workbench named {}", workbench_name))?;
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

    #[test]
    fn test_modify_step() {
        let mut p = Project::new("Project 1", true);
        let res = p.set_step_parameters(
            "Workbench 1",
            "Extrude 1",
            vec!["depth".to_string()],
            vec![5.65],
        );

        let wb = p.get_workbench("Workbench 1").unwrap();

        let mut found = false;
        for step in wb.steps.iter() {
            match step {
                crate::workbench::Step::Extrusion {
                    name, extrusion, ..
                } => {
                    if name == "Extrude 1" {
                        assert_eq!(extrusion.depth, 5.65);
                        found = true;
                    }
                }
                _ => {}
            }
        }
        assert_eq!(res.is_ok(), true);
        assert_eq!(found, true);
    }
}

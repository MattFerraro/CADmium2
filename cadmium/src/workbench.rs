use crate::common::Plane;
use crate::common::Point;
use crate::common::Vector;
use std::collections::HashMap;

// A workbench is like a part studio in onshape
#[derive(Debug)]
pub struct Workbench {
    pub name: String,
    pub steps: Vec<Step>,
}

impl Workbench {
    pub fn new(name: &str) -> Workbench {
        let mut w = Workbench {
            name: name.to_owned(),
            steps: vec![],
        };

        let origin = Point::new(0.0, 0.0, 0.0);
        w.add_point("Origin", origin);

        let x_axis = Vector::new(1.0, 0.0, 0.0);
        let y_axis = Vector::new(0.0, 1.0, 0.0);
        let z_axis = Vector::new(0.0, 0.0, 1.0);

        let top = Plane::new(origin, x_axis, y_axis, z_axis);
        let front = Plane::new(origin, z_axis, x_axis, y_axis);
        let right = Plane::new(origin, y_axis, z_axis, x_axis);
        w.add_plane("Top", top);
        w.add_plane("Front", front);
        w.add_plane("Right", right);

        w
    }

    pub fn add_point(&mut self, name: &str, p: Point) {
        self.steps.push(Step::Point {
            name: name.to_owned(),
            point: p,
        });
    }

    pub fn add_plane(&mut self, name: &str, p: Plane) {
        self.steps.push(Step::Plane {
            name: name.to_owned(),
            plane: p,
        });
    }

    pub fn create_view(&self, max_steps: usize) -> WorkbenchView {
        let mut wbv = WorkbenchView::new();
        for step in self.steps.iter().take(max_steps) {
            match step {
                Step::Point { point: p, name } => {
                    wbv.points.insert(name.to_owned(), p.clone());
                }
                Step::Plane { plane: p, name } => {
                    wbv.planes.insert(name.to_owned(), p.clone());
                }
            }
        }

        wbv
    }
}

#[derive(Debug)]
pub enum Step {
    Point { name: String, point: Point },
    Plane { name: String, plane: Plane },
    // Sketch { name: String, sketch: Sketch },
    // Extrusion { name: String, extrusion: Extrusion },
}

#[derive(Debug)]
pub struct WorkbenchView {
    pub points: HashMap<String, Point>,
    pub planes: HashMap<String, Plane>,
}

impl WorkbenchView {
    pub fn new() -> WorkbenchView {
        WorkbenchView {
            points: HashMap::new(),
            planes: HashMap::new(),
        }
    }
}

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::common::{Plane, Point, Solid, Vector};
use crate::sketch::Point as Point2D;
use crate::sketch::{Line, Segment, Sketch, SketchView};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn add_sketch_and_extrusion(&mut self) {
        let a = Point2D::new(-1.0, 0.0, "A");
        let b = Point2D::new(1.0, 0.0, "B");
        let c = Point2D::new(0.0, 1.0, "C");
        let segments = Segment::link(vec![a, b, c], true);
        let mut sketch1 = Sketch::new();
        sketch1.add_segments(segments);
        self.add_sketch("Sketch 1", sketch1, "Front");

        self.add_extrusion("Extrude 1", "Sketch 1", 10.0, vec![0], Operation::New);
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
    pub fn add_sketch(&mut self, name: &str, s: Sketch, plane: &str) {
        self.steps.push(Step::Sketch {
            name: name.to_owned(),
            sketch: s,
            plane: plane.to_owned(),
        });
    }

    pub fn find_plane(&self, name: &str) -> Option<&Plane> {
        for step in self.steps.iter() {
            match step {
                Step::Plane { name: n, plane } => {
                    if n == name {
                        return Some(plane);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn find_sketch(&self, name: &str) -> Option<&Sketch> {
        for step in self.steps.iter() {
            match step {
                Step::Sketch {
                    name: n,
                    sketch,
                    plane,
                } => {
                    if n == name {
                        // let plane = self.find_plane(plane).unwrap();
                        return Some(sketch);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn find_sketch_plane(&self, name: &str) -> Option<&Plane> {
        for step in self.steps.iter() {
            match step {
                Step::Sketch {
                    name: n,
                    sketch,
                    plane,
                } => {
                    if n == name {
                        let plane = self.find_plane(plane).unwrap();
                        return Some(plane);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn add_extrusion(
        &mut self,
        name: &str,
        sketch: &str,
        depth: f64,
        faces: Vec<usize>,
        operation: Operation,
    ) {
        // we need the normal!
        let plane = self.find_sketch_plane(sketch).unwrap();
        let normal = plane.normal;

        self.steps.push(Step::Extrusion {
            name: name.to_owned(),
            extrusion: Extrusion {
                depth: depth,
                direction: normal,
                operation: operation,
            },
            sketch: sketch.to_owned(),
            faces: faces,
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
                Step::Sketch {
                    sketch,
                    name,
                    plane,
                } => {
                    let actual_plane = wbv.find_plane(plane).unwrap();
                    let transform = actual_plane.to_frame();
                    let sketchview = sketch.create_view(&transform);
                    wbv.sketches.insert(name.to_owned(), sketchview);
                }
                Step::Extrusion {
                    name,
                    extrusion,
                    sketch,
                    faces,
                } => {
                    let sketchview = wbv.sketches.get(sketch).unwrap();

                    let mut count = 0;
                    for face_index in faces.iter() {
                        let face = &sketchview.faces[*face_index];
                        let res = face.tsweep(extrusion.direction, extrusion.depth);
                        let solid_name = format!("{}_{}", name, count);
                        wbv.solids.insert(solid_name.to_owned(), res);
                        count += 1;
                    }
                }
            }
        }

        wbv
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Step {
    Point {
        name: String,
        point: Point,
    },
    Plane {
        name: String,
        plane: Plane,
    },
    Sketch {
        name: String,
        sketch: Sketch,
        plane: String,
    },
    Extrusion {
        name: String,
        extrusion: Extrusion,
        sketch: String,
        faces: Vec<usize>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Operation {
    New,
    Add,
    Remove,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Extrusion {
    pub depth: f64,
    pub direction: Vector,
    pub operation: Operation,
}
impl Extrusion {
    pub fn new(depth: f64, direction: Vector, operation: Operation) -> Extrusion {
        Extrusion {
            depth: depth,
            direction: direction,
            operation: operation,
        }
    }
}

#[derive(Debug)]
pub struct WorkbenchView {
    pub points: HashMap<String, Point>,
    pub planes: HashMap<String, Plane>,
    pub sketches: HashMap<String, SketchView>,
    pub solids: HashMap<String, Solid>,
}

impl WorkbenchView {
    pub fn new() -> WorkbenchView {
        WorkbenchView {
            points: HashMap::new(),
            planes: HashMap::new(),
            sketches: HashMap::new(),
            solids: HashMap::new(),
        }
    }

    pub fn find_plane(&self, name: &str) -> Option<&Plane> {
        self.planes.get(name)
    }
}

#[cfg(test)]
mod tests {
    use crate::sketch;

    use super::*;

    #[test]
    fn test_trangular_prism() {
        let mut wb = Workbench::new("wb");

        let a = Point2D::new(-1.0, 0.0, "A");
        let b = Point2D::new(1.0, 0.0, "B");
        let c = Point2D::new(0.0, 1.0, "C");
        let segments = Segment::link(vec![a, b, c], true);
        let mut sketch1 = Sketch::new();
        sketch1.add_segments(segments);
        wb.add_sketch("sketch1", sketch1, "Front");

        wb.add_extrusion("ext1", "sketch1", 10.0, vec![0], Operation::New);

        let wbv = wb.create_view(100);
        let solid = wbv.solids.get("ext1_0").unwrap();
        let as_mesh = solid.get_mesh();

        solid.save_as_obj("test0.obj");
    }

    #[test]
    fn test_square_with_hole() {
        let mut wb = Workbench::new("wb");

        let a = Point2D::new(-1.0, -1.0, "A");
        let b = Point2D::new(1.0, -1.0, "B");
        let c = Point2D::new(1.0, 1.0, "C");
        let d = Point2D::new(-1.0, 1.0, "D");
        let mut segments_0 = Segment::link(vec![a, b, c, d], true);

        let e = Point2D::new(-2.0, -2.0, "E");
        let f = Point2D::new(2.0, -2.0, "F");
        let g = Point2D::new(2.0, 2.0, "G");
        let h = Point2D::new(-2.0, 2.0, "H");
        let segments_1 = Segment::link(vec![e, f, g, h], true);

        segments_0.extend(segments_1);

        let mut sketch1 = Sketch::new();
        sketch1.add_segments(segments_0);

        wb.add_sketch("sketch1", sketch1, "Front");
        wb.add_extrusion("ext1", "sketch1", 4.0, vec![1], Operation::New);

        let wbv = wb.create_view(100);

        let solid = wbv.solids.get("ext1_0").unwrap();
        let as_mesh = solid.get_mesh();

        solid.save_as_obj("test1.obj");
    }
}

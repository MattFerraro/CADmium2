pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
    pub construction: bool,
}

#[derive(Debug)]
pub struct Sketch {
    pub plane_name: String,
    pub lines: Vec<Line2D>,
    pub rings: Vec<Vec<u32>>,
}

#[derive(Debug)]
pub struct Plane {
    pub origin: Point3D,
    pub x_axis: Point3D,
    pub y_axis: Point3D,
    pub normal: Point3D,
}

#[derive(Debug)]
pub enum ExtrusionOperation {
    New,
    Add,
    Remove,
}

#[derive(Debug)]
pub enum Step {
    NewPoint { name: String, point: Point3D },
    NewPlane { name: String, plane: Plane },
    NewSketch { name: String, sketch: Sketch },
    NewExtrusion { name: String, extrusion: Extrusion },
}

#[derive(Debug)]
pub struct Extrusion {
    pub sketch_name: String,
    pub rings: Vec<u32>,
    pub depth: f64,
    pub direction: Point3D,
    pub operation: ExtrusionOperation,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub steps: Vec<Step>,
}

impl Project {
    pub fn add_point(&mut self, name: &str, p: Point3D) {
        self.steps.push(Step::NewPoint {
            name: name.to_owned(),
            point: p,
        });
    }

    pub fn add_plane(&mut self, name: &str, p: Plane) {
        self.steps.push(Step::NewPlane {
            name: name.to_owned(),
            plane: p,
        });
    }

    pub fn add_sketch(&mut self, name: &str, s: Sketch) {
        self.steps.push(Step::NewSketch {
            name: name.to_owned(),
            sketch: s,
        });
    }

    pub fn add_extrusion(&mut self, name: &str, e: Extrusion) {
        self.steps.push(Step::NewExtrusion {
            name: name.to_owned(),
            extrusion: e,
        });
    }

    pub fn get_plane(&self, name: &str) -> Option<&Plane> {
        for step in self.steps.iter() {
            if let Step::NewPlane { plane: p, name: n } = step {
                if n == name {
                    return Some(&p);
                }
            }
        }
        return None;
    }

    pub fn get_sketch(&self, name: &str) -> Option<&Sketch> {
        for step in self.steps.iter() {
            if let Step::NewSketch { sketch: s, name: n } = step {
                if n == name {
                    return Some(&s);
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

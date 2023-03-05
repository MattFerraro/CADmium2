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
pub struct Sketch<'a> {
    pub plane_name: String,
    pub lines: Vec<Line2D>,
    pub rings: Vec<Vec<&'a Line2D>>,
}

#[derive(Debug)]
pub struct Plane {
    pub origin: Point3D,
    pub x_axis: Point3D,
    pub y_axis: Point3D,
    pub normal: Point3D,
    pub name: String,
}

#[derive(Debug)]
pub enum ExtrusionOperation {
    New,
    Add,
    Remove,
}

#[derive(Debug)]
pub enum Step<'a> {
    NewPoint { name: String, point: Point3D },
    NewPlane { name: String, plane: Plane },
    NewSketch { name: String, sketch: Sketch<'a> },
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
pub struct Project<'a> {
    pub name: String,
    pub steps: Vec<Step<'a>>,
}

impl<'a> Project<'a> {
    pub fn add_step(&mut self, step: Step<'a>) {
        self.steps.push(step);
    }

    pub fn get_plane(&self, name: &str) -> Option<&Plane> {
        for step in self.steps.iter() {
            if let Step::NewPlane { plane: p, .. } = step {
                let plane_name = &p.name;
                if plane_name == name {
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

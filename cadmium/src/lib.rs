use std::collections::HashMap;
use truck_modeling::*;

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

impl Point3D {
    pub fn scale(&self, factor: f64) -> Point3D {
        Point3D {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
    pub fn plus(&self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    pub fn to_vertex(&self) -> Vertex {
        builder::vertex(self.to_point3())
    }
    pub fn to_point3(&self) -> Point3 {
        Point3::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
    pub construction: bool,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Point3D>,
    pub normals: Vec<Point3D>,
    pub uvs: Vec<Point2D>,
    pub indices: Vec<u32>,
}

#[derive(Debug)]
pub struct Representation {
    pub points: HashMap<String, Point3D>,
    pub planes: HashMap<String, Plane>,
    pub sketches: HashMap<String, ConcreteSketch>,
    pub solids: HashMap<String, Mesh>,
}

#[derive(Debug, Clone)]
pub struct Sketch {
    pub plane_name: String,
    pub lines: Vec<Line2D>,
    pub rings: Vec<Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct ConcreteSketch {
    pub plane_name: String,
    pub edges: Vec<Edge>,
    pub wires: Vec<Wire>,
    pub faces: Vec<Face>,
}

impl ConcreteSketch {
    pub fn new(s: &Sketch, p: &Project) -> ConcreteSketch {
        let mut cs = ConcreteSketch {
            plane_name: s.plane_name.to_owned(),
            edges: vec![],
            wires: vec![],
            faces: vec![],
        };

        let plane = p
            .get_plane(&s.plane_name)
            .expect("Project has no plane by that name!");
        println!("{:?}", plane);

        let x_axis = plane.x_axis;
        let y_axis = plane.y_axis;
        let origin = plane.origin;

        for l in s.lines.iter() {
            let x1 = x_axis.scale(l.start.x).plus(origin);
            let y1 = y_axis.scale(l.start.y).plus(origin);
            let start = x1.plus(y1).to_vertex();

            let x2 = x_axis.scale(l.end.x).plus(origin);
            let y2 = y_axis.scale(l.end.y).plus(origin);
            let end = x2.plus(y2).to_vertex();

            let edge = builder::line(&start, &end);
            cs.edges.push(edge);
        }

        // To find all wires we need to:
        // x----x
        // |    |
        // x----x
        // |    |
        // x----x
        // loop through each vertex which has not yet been accessed (start will all being possible)
        // find all edges that have this vertex as start or end
        // make a wire for each one

        // start with every edge being its own Wire (7 in diagram)
        // if two Wires share a start or end vertex but no other wires
        // touch that vertex, merge them (3 left in diagram, top, bottom, middle)
        // what do we do with the shared edges?

        // https://stackoverflow.com/questions/12367801/finding-all-cycles-in-undirected-graphs

        // Let's fake it for now!
        let wire: Wire = vec![
            cs.edges[0].clone(),
            cs.edges[1].clone(),
            cs.edges[2].clone(),
            cs.edges[3].clone(),
        ]
        .into();
        cs.wires.push(wire);

        // for w in cs.wires.iter() {
        //     let f = Face::new(
        //         vec![wire],
        //         Surface::NurbsSurface(NurbsSurface::new(surface)), // this should just be a plane
        //     )
        // }

        cs
    }
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

    pub fn get_representation(&self, steps: usize) -> Option<Representation> {
        let mut repr = Representation {
            points: HashMap::new(),
            planes: HashMap::new(),
            sketches: HashMap::new(),
            solids: HashMap::new(),
        };

        let mut vertices: HashMap<String, Vertex> = HashMap::new();
        // let mut edges: HashMap<String, Edge> = HashMap::new();
        // let mut wires: HashMap<String, Wire> = HashMap::new();
        // let mut faces: HashMap<String, Wire> = HashMap::new();

        for step in self.steps.iter().take(steps) {
            match step {
                Step::NewPoint { point: p, name: n } => {
                    repr.points.insert(n.to_owned(), p.clone());
                }
                Step::NewPlane { plane: p, name: n } => {
                    repr.planes.insert(n.to_owned(), p.clone());
                }
                Step::NewSketch { sketch: s, name: n } => {
                    let concrete: ConcreteSketch = ConcreteSketch::new(s, &self);
                    repr.sketches.insert(n.to_owned(), concrete);
                }
                _ => {}
            }
        }

        Some(repr)
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

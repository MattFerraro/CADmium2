use std::collections::HashMap;
use truck_meshalgo::prelude::*;
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
    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
    pub construction: bool,
}

impl Line2D {
    pub fn new(start: Point2D, end: Point2D) -> Line2D {
        Line2D {
            start: start,
            end: end,
            construction: false,
        }
    }
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
    pub faces: Vec<usize>,
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
    pub indices: Vec<usize>,
}

#[derive(Debug)]
pub struct Representation {
    pub points: HashMap<String, Point3D>,
    pub planes: HashMap<String, Plane>,
    pub sketches: HashMap<String, ConcreteSketch>,
    pub solids: HashMap<String, Vec<Solid>>,
    pub meshes: HashMap<String, Vec<PolygonMesh>>,
}

#[derive(Debug, Clone)]
pub struct Sketch {
    pub plane_name: String,
    pub lines: Vec<Line2D>,
    pub faces: Vec<Vec<u32>>,
    pub verticies: Vec<Point2D>,
}

#[derive(Debug, Clone)]
pub struct ConcreteSketch {
    pub plane_name: String,
    pub edges: Vec<Edge>,
    pub wires: Vec<Wire>,
    pub faces: Vec<Face>,
    pub vertices: Vec<Point3D>,
}

impl ConcreteSketch {
    pub fn new(s: &Sketch, p: &Project) -> ConcreteSketch {
        let mut cs = ConcreteSketch {
            plane_name: s.plane_name.to_owned(),
            edges: vec![],
            wires: vec![],
            faces: vec![],
            vertices: vec![],
        };

        let plane = p
            .get_plane(&s.plane_name)
            .expect("Project has no plane by that name!");
        // println!("{:?}", plane);
        let plane0: Surface = truck_modeling::Plane::new(
            Point3::new(plane.origin.x, plane.origin.y, plane.origin.z),
            Point3::new(plane.x_axis.x, plane.x_axis.y, plane.x_axis.z),
            Point3::new(plane.y_axis.x, plane.y_axis.y, plane.y_axis.z),
        )
        .into();

        // println!("Here is a truck plane: {:?}", plane0);

        let x_axis = plane.x_axis;
        let y_axis = plane.y_axis;
        let origin = plane.origin;

        for v in s.verticies.iter() {
            let x = x_axis.scale(v.x);
            let y = y_axis.scale(v.y);
            let vertex = x.plus(y).plus(origin);
            cs.vertices.push(vertex);
        }

        // TODO: should edges be represented as [start_vertex_index, end_vertex_index] instead of
        // as [start_vertex, end_vertex], which is right now deep-copied? Doing so would mean
        // we don't have to do the coordinate transform twice here
        for l in s.lines.iter() {
            let x1 = x_axis.scale(l.start.x);
            let y1 = y_axis.scale(l.start.y);
            let start = x1.plus(y1).plus(origin).to_vertex();

            let x2 = x_axis.scale(l.end.x);
            let y2 = y_axis.scale(l.end.y);
            let end = x2.plus(y2).plus(origin).to_vertex();

            let edge = builder::line(&start, &end);
            // println!("\n{:?}", edge);
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
        let v0 = cs.vertices[0].to_vertex();
        let v1 = cs.vertices[1].to_vertex();
        let v2 = cs.vertices[2].to_vertex();
        let wire: Wire = vec![
            builder::line(&v0, &v1),
            builder::line(&v1, &v2),
            builder::line(&v2, &v0),
        ]
        .into();
        cs.wires.push(wire.clone());

        // working example from the reference repo!
        // let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
        // let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
        // let v2 = builder::vertex(Point3::new(0.0, 1.0, 0.0));
        // let wire: Wire = vec![builder::line(&v0, &v1), builder::line(&v1, &v2)].into();
        // let mut wires = vec![wire];
        // wires[0].push_back(builder::line(&v2, &v0));
        // let plane = builder::try_attach_plane(&wires);
        // println!("My Plane: {:?}", plane);
        // end example

        for w in cs.wires.iter() {
            let f = Face::new(vec![w.clone()], plane0.clone());
            // let b = builder::try_attach_plane(&vec![w.clone()]);
            cs.faces.push(f);
        }

        cs
    }
}

impl Project {
    pub fn new(name: &str) -> Project {
        let mut proj = Project {
            name: name.to_string(),
            steps: vec![],
        };
        proj.add_defaults();
        proj
    }
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

    pub fn add_defaults(&mut self) {
        let origin: Point3D = Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        self.add_point("Origin", origin);

        let x_axis: Point3D = Point3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let y_axis: Point3D = Point3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let z_axis: Point3D = Point3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let top_plane: Plane = Plane {
            origin: origin,
            x_axis: x_axis,
            y_axis: y_axis,
            normal: z_axis,
        };
        self.add_plane("Top", top_plane);

        let front_plane: Plane = Plane {
            origin: origin,
            x_axis: z_axis,
            y_axis: x_axis,
            normal: y_axis,
        };
        self.add_plane("Front", front_plane);

        let right_plane: Plane = Plane {
            origin: origin,
            x_axis: y_axis,
            y_axis: z_axis,
            normal: x_axis,
        };
        self.add_plane("Right", right_plane);
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

    pub fn extrude(&self, extrusion: &Extrusion, repr: &Representation) -> Vec<Solid> {
        let mut new_meshes: Vec<Solid> = vec![];
        let concrete_sketch = &repr.sketches[&extrusion.sketch_name];
        // let faces = &repr.faces[extrusion.faces]
        for face_index in extrusion.faces.iter() {
            // println!("A face index: {:?}", face_index);
            let face2 = &concrete_sketch.faces[*face_index];
            // println!("A face: {:?}", face2);
            // TODO: EXTRUDE IT! Return a mesh!
            let res = builder::tsweep(
                face2,
                extrusion.direction.scale(extrusion.depth).to_vector3(),
            );
            // println!("\n\tExtrusion result: {:?}", res);
            new_meshes.push(res);
        }

        // println!("Source Extrusion:\n{:?}", extrusion);
        // println!("Source Concrete Sketche:\n{:?}", concrete_sketch.faces[0]);
        new_meshes
    }

    pub fn get_representation(&self, steps: usize) -> Option<Representation> {
        let mut repr = Representation {
            points: HashMap::new(),
            planes: HashMap::new(),
            sketches: HashMap::new(),
            solids: HashMap::new(),
            meshes: HashMap::new(),
        };

        // let mut vertices: HashMap<String, Vertex> = HashMap::new();
        // let mut edges: HashMap<String, Edge> = HashMap::new();
        // let mut wires: HashMap<String, Wire> = HashMap::new();
        // let mut faces: HashMap<String, Wire> = HashMap::new();

        for step in self.steps.iter().take(steps) {
            match step {
                Step::NewPoint { point: p, name } => {
                    repr.points.insert(name.to_owned(), p.clone());
                }
                Step::NewPlane { plane: p, name } => {
                    repr.planes.insert(name.to_owned(), p.clone());
                }
                Step::NewSketch { sketch: s, name } => {
                    let concrete: ConcreteSketch = ConcreteSketch::new(s, &self);
                    repr.sketches.insert(name.to_owned(), concrete);
                }
                Step::NewExtrusion { extrusion, name } => {
                    let new_solids = self.extrude(extrusion, &repr);
                    repr.solids.insert(name.to_owned(), new_solids);
                }
            }
        }

        // TODO: handle other kinds of extrusions where solids might disappear or merge or split

        for (name, solid_list) in repr.solids.iter() {
            let mut meshes_for_this_solid_list: Vec<PolygonMesh> = vec![];
            for solid in solid_list.iter() {
                // let polygon = solid;
                let mut mesh = solid.triangulation(0.01).to_polygon();
                mesh.put_together_same_attrs();
                // assert!(mesh.shell_condition() == ShellCondition::Closed);
                meshes_for_this_solid_list.push(mesh);
            }
            repr.meshes
                .insert(name.to_string(), meshes_for_this_solid_list);
        }

        Some(repr)
    }
}

pub fn save_mesh_as_obj(mesh: &PolygonMesh, filename: &str) {
    let file = std::fs::File::create(filename).unwrap();
    obj::write(&mesh, file).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn triangular_prism() {
        let mut project1: Project = Project::new("First Project");

        //       c
        //     / |
        //   a---b
        let a: Point2D = Point2D { x: 0.0, y: 0.0 };
        let b: Point2D = Point2D { x: 1.0, y: 0.0 };
        let c: Point2D = Point2D { x: 1.0, y: 1.0 };

        let l1 = Line2D::new(a, b);
        let l2 = Line2D::new(b, c);
        let l3 = Line2D::new(c, a);

        let s: Sketch = Sketch {
            plane_name: "Front".to_string(),
            verticies: vec![a, b, c],
            lines: vec![l1, l2, l3],
            faces: vec![vec![0, 1, 2, 0]],
        };
        project1.add_sketch("Sketch1", s);

        let ext1: Extrusion = Extrusion {
            sketch_name: "Sketch1".to_string(),
            faces: vec![0],
            depth: 0.5,
            operation: ExtrusionOperation::New,
            direction: project1
                .get_plane(&project1.get_sketch("Sketch1").unwrap().plane_name)
                .unwrap()
                .normal,
        };
        project1.add_extrusion("Ext1", ext1);

        let repr = project1.get_representation(100).unwrap();
        let ext1_mesh = &repr.meshes["Ext1"][0];

        let local_filename = "triangular_prism.obj";
        save_mesh_as_obj(ext1_mesh, local_filename);
        let _ = std::fs::remove_file(local_filename);
    }
}

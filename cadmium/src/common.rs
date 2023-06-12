// #![allow(unused_variables)]
// #![allow(unused_imports)]

use crate::sketch::Point as SketchPoint;
use serde::{Deserialize, Serialize};
use truck_meshalgo::prelude::*;
use truck_modeling::{builder, Edge, Face, Point3, Vector3, Vertex, Wire};
use truck_stepio::out;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn add(&self, other: Point) -> Self {
        Point {
            x: other.x + self.x,
            y: other.y + self.y,
            z: other.z + self.z,
        }
    }

    pub fn add_vec(&self, other: Vector) -> Self {
        Point {
            x: other.x + self.x,
            y: other.y + self.y,
            z: other.z + self.z,
        }
    }

    pub fn scale(&self, s: f64) -> Self {
        Point {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn to_vector(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn to_vertex(&self) -> Vertex {
        builder::vertex(self.to_point3())
    }
    pub fn to_point3(&self) -> Point3 {
        Point3::new(self.x, self.y, self.z)
    }
    pub fn from_point3(p: Point3) -> Self {
        Point {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x: x, y: y, z: z }
    }

    pub fn add(&self, other: Vector) -> Self {
        Vector {
            x: other.x + self.x,
            y: other.y + self.y,
            z: other.z + self.z,
        }
    }

    pub fn negate(&self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn cross(&self, other: Vector) -> Self {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn subtract(&self, other: Vector) -> Self {
        self.add(other.negate())
    }

    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn scale(&self, s: f64) -> Self {
        Vector {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
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
    pub fn from_vector3(v: Vector3) -> Self {
        Vector {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct UV {
    pub u: f64,
    pub v: f64,
}

impl UV {
    pub fn new(u: f64, v: f64) -> Self {
        UV { u, v }
    }
}

#[derive(Clone, Copy)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quaternion {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Quaternion { x, y, z, w }
    }

    pub fn identity() -> Self {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        Quaternion {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineFace {
    pub exterior: LineRing,
    pub interiors: Vec<LineRing>,
}

impl LineFace {
    pub fn new() -> Self {
        LineFace {
            exterior: LineRing::new(),
            interiors: vec![],
        }
    }

    pub fn add_interior(&mut self, r: LineRing) {
        self.interiors.push(r);
    }

    pub fn to_face(&self) -> Face {
        let mut wires: Vec<Wire> = vec![self.exterior.to_wire()];
        for r in &self.interiors {
            wires.push(r.to_wire().inverse());
        }
        let face = builder::try_attach_plane(&wires).unwrap();
        face
    }

    pub fn tsweep(&self, direction: Vector, depth: f64) -> Solid {
        // First we need to build a truck representation of this face.
        let face = self.to_face();
        let truck_solid = builder::tsweep(&face, direction.scale(depth).to_vector3());
        let solid = Solid::new(truck_solid);
        solid
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LineRing {
    pub segments: Vec<LineSegment>,
}
impl LineRing {
    pub fn new() -> Self {
        LineRing { segments: vec![] }
    }
    pub fn add_segment(&mut self, s: LineSegment) {
        self.segments.push(s);
    }

    pub fn to_wire(&self) -> Wire {
        let truck_vertices = self
            .segments
            .iter()
            .map(|s| s.start.to_vertex())
            .collect::<Vec<Vertex>>();

        let mut truck_edges = truck_vertices
            .windows(2)
            .map(|chunk| {
                let v1 = chunk.get(0).unwrap();
                let v2 = chunk.get(1).unwrap();

                builder::line(&v1, &v2)
            })
            .collect::<Vec<Edge>>();
        truck_edges.push(builder::line(
            truck_vertices.last().unwrap(),
            truck_vertices.first().unwrap(),
        ));

        let wire = Wire::from_iter(truck_edges.into_iter());
        wire
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> Self {
        LineSegment { start, end }
    }

    pub fn to_edge(&self) -> Edge {
        builder::line(&self.start.to_vertex(), &self.end.to_vertex())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Plane {
    pub origin: Point,
    pub x_axis: Vector,
    pub y_axis: Vector,
    pub normal: Vector,
    pub frame: CoordinateFrame,
    pub width: f64,
    pub height: f64,
}

impl Plane {
    pub fn new(origin: Point, x: Vector, y: Vector, normal: Vector) -> Self {
        Plane {
            origin,
            x_axis: x,
            y_axis: y,
            normal,
            frame: CoordinateFrame {
                origin,
                x_axis: x,
                y_axis: y,
                normal,
            },
            width: 150.0,
            height: 150.0,
        }
    }

    pub fn to_frame(&self) -> CoordinateFrame {
        CoordinateFrame {
            origin: self.origin,
            x_axis: self.x_axis,
            y_axis: self.y_axis,
            normal: self.normal,
        }
    }

    pub fn get_mesh(&self) -> Mesh {
        let mut vertices: Vec<Point> = vec![];
        let mut normals: Vec<Vector> = vec![];
        let mut uvs: Vec<UV> = vec![];
        let mut indices: Vec<usize> = vec![];

        let frame = self.frame;
        let x_axis = frame.x_axis;
        let y_axis = frame.y_axis;
        let normal = frame.normal;

        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;

        let p1 = frame
            .origin
            .add_vec(x_axis.scale(-half_width))
            .add_vec(y_axis.scale(-half_height));
        let p2 = frame
            .origin
            .add_vec(x_axis.scale(half_width))
            .add_vec(y_axis.scale(-half_height));
        let p3 = frame
            .origin
            .add_vec(x_axis.scale(half_width))
            .add_vec(y_axis.scale(half_height));
        let p4 = frame
            .origin
            .add_vec(x_axis.scale(-half_width))
            .add_vec(y_axis.scale(half_height));

        vertices.push(p1);
        vertices.push(p2);
        vertices.push(p3);
        vertices.push(p4);

        normals.push(normal);
        normals.push(normal);
        normals.push(normal);
        normals.push(normal);

        uvs.push(UV::new(0.0, 0.0));
        uvs.push(UV::new(1.0, 0.0));
        uvs.push(UV::new(1.0, 1.0));
        uvs.push(UV::new(0.0, 1.0));

        indices.push(0);
        indices.push(1);
        indices.push(2);
        indices.push(0);
        indices.push(2);
        indices.push(3);

        Mesh {
            vertices,
            normals,
            uvs,
            indices,
        }
    }

    pub fn get_up(&self) -> Vector {
        self.y_axis
    }

    pub fn get_upper_left(&self) -> Point {
        self.origin
            .add_vec(self.x_axis.scale(-self.width / 2.0))
            .add_vec(self.y_axis.scale(self.height / 2.0))
    }

    pub fn get_rotation_matrix(&self) -> Vec<Vec<f64>> {
        let frame = self.frame;
        let x_axis = frame.x_axis;
        let y_axis = frame.y_axis;
        let normal = frame.normal;

        let m = vec![
            vec![x_axis.x, x_axis.y, x_axis.z],
            vec![y_axis.x, y_axis.y, y_axis.z],
            vec![normal.x, normal.y, normal.z],
        ];
        m
    }

    pub fn get_quaternion(&self) -> Quaternion {
        // the normal in the primary XYZ coordinate frame is the z axis:
        let up = Vector::new(0.0, 0.0, 1.0);

        // but the normal on this plane is given by frame.normal
        // so we need to find the quaternion that rotates the z axis to the normal
        let frame = self.frame;
        let normal = frame.normal;

        let dot = normal.dot(up);
        // If the two vectors are already parallel, then the rotation is the identity
        if dot > 0.999999 {
            // 180 degree rotation around any orthogonal vector
            return Quaternion::identity();
        } else {
            let a = normal.cross(up);
            return Quaternion::new(a.x, a.y, a.z, 1.0 + dot).normalize();
        }
        // see https://github.com/toji/gl-matrix/blob/f0583ef53e94bc7e78b78c8a24f09ed5e2f7a20c/src/gl-matrix/quat.js#L54
        // for a reference implementation
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CoordinateFrame {
    pub origin: Point,
    pub x_axis: Vector,
    pub y_axis: Vector,
    pub normal: Vector,
}

impl CoordinateFrame {
    pub fn new(origin: Point, x: Vector, y: Vector, normal: Vector) -> Self {
        CoordinateFrame {
            origin,
            x_axis: x,
            y_axis: y,
            normal,
        }
    }

    pub fn to_3d(&self, SketchPoint { x, y, id }: SketchPoint) -> Point {
        self.origin
            .add(self.x_axis.scale(x).to_point())
            .add(self.y_axis.scale(y).to_point())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Point>,
    pub normals: Vec<Vector>,
    pub uvs: Vec<UV>,
    pub indices: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Solid {
    pub name: String,
    pub mesh: Mesh,
    pub color: Color,
    pub truck_solid: truck_modeling::Solid,
}

impl Solid {
    pub fn new(truck_solid: truck_modeling::Solid) -> Self {
        Solid {
            name: "unnamed".to_owned(),
            mesh: Mesh {
                vertices: vec![],
                normals: vec![],
                uvs: vec![],
                indices: vec![],
            },
            color: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            },
            truck_solid: truck_solid,
        }
    }

    pub fn get_mesh(&self) -> Mesh {
        let mut mesh = self.truck_solid.triangulation(0.001).to_polygon();
        mesh.put_together_same_attrs();

        // the mesh is prepared for obj export, but we need to convert it
        // to a format compatible for rendering
        // We have to brute force this. Go through every single triangle
        // and emit three positions, three normals, and three uvs.
        let mut vertices: Vec<Point> = vec![];
        let mut normals: Vec<Vector> = vec![];
        let mut uvs: Vec<UV> = vec![];
        let mut indices: Vec<usize> = vec![];
        let mut index = 0 as usize;
        for face in mesh.tri_faces() {
            for v in face.iter() {
                let vertex_index = v.pos;
                let normal_index = v.nor.unwrap();
                let uv_index = v.uv.unwrap();
                let vertex = mesh.positions()[vertex_index];
                let normal = mesh.normals()[normal_index];
                let uv = mesh.uv_coords()[uv_index];

                let pt = Point::from_point3(vertex);
                vertices.push(pt);
                normals.push(Vector::from_vector3(normal));
                uvs.push(UV { u: uv.x, v: uv.y });
                indices.push(index);

                index += 1;
            }
        }

        Mesh {
            vertices,
            normals,
            uvs,
            indices,
        }

        // Mesh {
        //     vertices: mesh
        //         .positions()
        //         .iter()
        //         .map(|p| Point::from_point3(*p))
        //         .collect(),
        //     normals: mesh
        //         .normals()
        //         .iter()
        //         .map(|n| Vector::from_vector3(*n))
        //         .collect(),
        //     uvs: mesh
        //         .uv_coords()
        //         .iter()
        //         .map(|uv| UV { u: uv.x, v: uv.y })
        //         .collect(),
        //     indices: mesh
        //         .tri_faces()
        //         .iter()
        //         .flat_map(|tri| tri.iter().map(|v| v.pos))
        //         .collect(),
        // }
    }

    pub fn save_as_obj(&self, filename: &str) {
        let mut mesh = self.truck_solid.triangulation(0.001).to_polygon();
        mesh.put_together_same_attrs();
        let file = std::fs::File::create(filename).unwrap();
        obj::write(&mesh, file).unwrap();
    }

    pub fn get_obj_text(&self) -> String {
        let mut mesh = self.truck_solid.triangulation(0.001).to_polygon();
        mesh.put_together_same_attrs();
        // let mut text = String::new();
        let mut buf = Vec::new();
        obj::write(&mesh, &mut buf).unwrap();
        let string = String::from_utf8(buf).unwrap();
        string
        // fn scale_vertices(line: &str) -> String {
        //     if line.starts_with("v ") {
        //         let mut parts = line.split(" ");
        //         let _ = parts.next();
        //         let x = parts.next().unwrap().parse::<f64>().unwrap();
        //         let y = parts.next().unwrap().parse::<f64>().unwrap();
        //         let z = parts.next().unwrap().parse::<f64>().unwrap();
        //         let pt = Point::new(x, y, z);
        //         let scaled = pt.scale(100.0);
        //         return format!("v {} {} {}\n", scaled.x, scaled.y, scaled.z);
        //     } else {
        //         return format!("{}\n", line);
        //     }
        // }

        // let scaled: String = string.split("\n").map(scale_vertices).collect();

        // scaled
    }

    pub fn get_step_text(&self) -> String {
        let compressed = self.truck_solid.compress();
        let step_string = out::CompleteStepDisplay::new(
            out::StepModel::from(&compressed),
            out::StepHeaderDescriptor {
                origination_system: "shape-to-step".to_owned(),
                ..Default::default()
            },
        )
        .to_string();
        step_string
    }

    pub fn save_as_step(&self, filename: &str) {
        let step_text = self.get_step_text();
        let mut step_file = std::fs::File::create(filename).unwrap();
        std::io::Write::write_all(&mut step_file, step_text.as_ref()).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

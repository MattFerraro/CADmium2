#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::sketch::Point as SketchPoint;
use serde::{Deserialize, Serialize};
use truck_meshalgo::prelude::*;
use truck_modeling::{builder, Curve, Edge, Face, Point3, Vector3, Vertex, Wire};

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
            width: 1.0,
            height: 1.0,
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

        Mesh {
            vertices: mesh
                .positions()
                .iter()
                .map(|p| Point::from_point3(*p))
                .collect(),
            normals: mesh
                .normals()
                .iter()
                .map(|n| Vector::from_vector3(*n))
                .collect(),
            uvs: mesh
                .uv_coords()
                .iter()
                .map(|uv| UV { u: uv.x, v: uv.y })
                .collect(),
            indices: mesh
                .tri_faces()
                .iter()
                .flat_map(|tri| tri.iter().map(|v| v.pos))
                .collect(),
        }
    }

    pub fn save_as_obj(&self, filename: &str) {
        let mut mesh = self.truck_solid.triangulation(0.001).to_polygon();
        mesh.put_together_same_attrs();
        let file = std::fs::File::create(filename).unwrap();
        obj::write(&mesh, file).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

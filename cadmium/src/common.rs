use truck_modeling::*;

use crate::sketch::Point as SketchPoint;

#[derive(Debug, Clone, Copy)]
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
    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone)]
pub struct LineFace {
    pub exterior: LineRing,
    pub interiors: Vec<LineRing>,
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub origin: Point,
    pub x_axis: Vector,
    pub y_axis: Vector,
    pub normal: Vector,
}

impl Plane {
    pub fn new(origin: Point, x: Vector, y: Vector, normal: Vector) -> Self {
        Plane {
            origin,
            x_axis: x,
            y_axis: y,
            normal,
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

#[derive(Debug, Clone)]
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

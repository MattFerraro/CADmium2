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

    pub fn to_vector(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
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

    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
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
}

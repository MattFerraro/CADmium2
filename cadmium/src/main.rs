use cadmium::*;

fn main() {
    let mut project1: Project = Project {
        name: "First Project!".to_string(),
        steps: vec![],
    };
    let origin: Point3D = Point3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    project1.add_point("Origin", origin);

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
    project1.add_plane("Top", top_plane);

    let front_plane: Plane = Plane {
        origin: origin,
        x_axis: z_axis,
        y_axis: x_axis,
        normal: y_axis,
    };
    project1.add_plane("Front", front_plane);

    let right_plane: Plane = Plane {
        origin: origin,
        x_axis: y_axis,
        y_axis: z_axis,
        normal: x_axis,
    };
    project1.add_plane("Right", right_plane);

    let a: Point2D = Point2D { x: 0.0, y: 0.0 };
    let b: Point2D = Point2D { x: 1.0, y: 0.0 };
    let c: Point2D = Point2D { x: 1.0, y: 1.0 };
    let d: Point2D = Point2D { x: 0.0, y: 1.0 };

    let l1 = Line2D {
        start: a,
        end: b,
        construction: false,
    };
    let l2 = Line2D {
        start: b,
        end: c,
        construction: false,
    };
    let l3 = Line2D {
        start: c,
        end: d,
        construction: false,
    };
    let l4 = Line2D {
        start: d,
        end: a,
        construction: false,
    };

    let s: Sketch = Sketch {
        plane_name: "Top".to_string(),
        lines: vec![l1, l2, l3, l4],
        rings: vec![vec![0, 1, 2, 3]],
    };
    project1.add_sketch("Sketch1", s);

    let ext1: Extrusion = Extrusion {
        sketch_name: "Sketch1".to_string(),
        rings: vec![0],
        depth: 0.5,
        operation: ExtrusionOperation::New,
        direction: project1
            .get_plane(&project1.get_sketch("Sketch1").unwrap().plane_name)
            .unwrap()
            .normal,
    };
    project1.add_extrusion("Ext1", ext1);

    let repr = project1.get_representation(10);

    println!("{:?}", repr);
}

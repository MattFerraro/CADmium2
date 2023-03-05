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
    let step1: Step = Step::NewPoint {
        name: String::from("Set Origin"),
        point: origin,
    };
    project1.add_step(step1);

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
        name: String::from("Top"),
    };
    let step2 = Step::NewPlane {
        name: String::from("Top Plane"),
        plane: top_plane,
    };
    project1.add_step(step2);

    let front_plane: Plane = Plane {
        origin: origin,
        x_axis: z_axis,
        y_axis: x_axis,
        normal: y_axis,
        name: String::from("Front"),
    };
    let step3 = Step::NewPlane {
        name: String::from("Front Plane"),
        plane: front_plane,
    };
    project1.add_step(step3);

    let right_plane: Plane = Plane {
        origin: origin,
        x_axis: y_axis,
        y_axis: z_axis,
        normal: x_axis,
        name: String::from("Right"),
    };
    let step4 = Step::NewPlane {
        name: String::from("Right Plane"),
        plane: right_plane,
    };
    project1.add_step(step4);

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
        rings: vec![vec![&l1, &l2, &l3, &l4]],
    };
    let step5 = Step::NewSketch {
        name: String::from("Sketch1"),
        sketch: s,
    };
    project1.add_step(step5);

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
    let step6 = Step::NewExtrusion {
        name: "Extrusion1".to_string(),
        extrusion: ext1,
    };
    project1.add_step(step6);

    println!("{:?}", project1);
}

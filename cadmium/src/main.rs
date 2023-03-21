use cadmium::*;

fn main() {
    let mut project1: Project = Project::new("First Project");

    //   d------c
    //   |      |
    //   a------b
    let a: Point2D = Point2D { x: 0.0, y: 0.0 };
    let b: Point2D = Point2D { x: 2.0, y: 0.0 };
    let c: Point2D = Point2D { x: 2.0, y: 1.0 };
    let d: Point2D = Point2D { x: 0.0, y: 1.0 };

    let l1 = Line2D::new(a, b);
    let l2 = Line2D::new(b, c);
    let l3 = Line2D::new(c, d);
    let l4 = Line2D::new(d, a);

    let s: Sketch = Sketch {
        plane_name: "Front".to_string(),
        verticies: vec![a, b, c, d],
        lines: vec![l1, l2, l3, l4],
        faces: vec![vec![0, 1, 2, 3, 0]],
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
    // println!("ext1_mesh: {:?}", ext1_mesh);

    let local_filename = "rectangular_prism.obj";
    save_mesh_as_obj(ext1_mesh, local_filename);
    // let _ = std::fs::remove_file(local_filename);
}

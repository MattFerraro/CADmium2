use cadmium::*;

fn main() {
    let mut project1: Project = Project::new("First Project");

    //       c
    //     / |
    //   a---b
    let a: Point2D = Point2D { x: 0.0, y: 0.0 };
    let b: Point2D = Point2D { x: 1.0, y: 0.0 };
    let c: Point2D = Point2D { x: 1.0, y: 1.0 };
    // let d: Point2D = Point2D { x: 0.0, y: 1.0 };

    let l1 = Line2D::new(a, b);
    let l2 = Line2D::new(b, c);
    let l3 = Line2D::new(c, a);
    // let l4 = Line2D::new(d, a);

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

    // println!("Points: {:?}", repr.points);
    // println!("Planes: {:?}", repr.planes);
    // println!("Sketches:\n{:?}", repr.sketches["Sketch1"].faces);
    // println!("Solids:\n{:?}", repr.solids["Ext1"]);
    let ext1_mesh = &repr.meshes["Ext1"][0];
    // println!("Meshes:\n{:?}", ext1_mesh.uv_coords());

    //TODO: make it write to .obj and .stl formats!
    save_mesh_as_obj(ext1_mesh, "Ext1.obj");
}

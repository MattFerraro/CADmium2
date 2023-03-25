use cadmium::*;
use truck_meshalgo::prelude::*;
use truck_modeling::*;

fn main() {
    println!("Main!");
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(3.5, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(0.0, 3.5, 0.0));

    let v3 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v4 = builder::vertex(Point3::new(2.0, 1.0, 0.0));
    let v5 = builder::vertex(Point3::new(1.0, 2.0, 0.0));

    let up = Point3D {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    }
    .to_vector3();

    let outer_wire: Wire = vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v0),
    ]
    .into();

    let inner_wire: Wire = vec![
        builder::line(&v3, &v5),
        builder::line(&v5, &v4),
        builder::line(&v4, &v3),
    ]
    .into();
    // println!("My outer wire: {:?}", outer_wire.inverse());
    let wires = vec![outer_wire, inner_wire];
    let face = builder::try_attach_plane(&wires).unwrap();

    let solid = builder::tsweep(&face, up);

    let mut mesh = solid.triangulation(0.01).to_polygon();
    mesh.put_together_same_attrs();

    save_mesh_as_obj(&mesh, "complex.obj");
    // println!("My Solid: {:?}", res);
}

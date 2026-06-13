mod geometry;

use geometry::Mesh;

fn main() {
    println!("Rust 3D Modeler");
    println!("MVP: vertex, edge, triangle, polygon");

    let mut mesh = Mesh::new();

    let vertex_0_id = mesh.add_vertex(0.0, 0.0, 0.0);
    let vertex_1_id = mesh.add_vertex(1.0, 0.0, 0.0);
    let vertex_2_id = mesh.add_vertex(1.0, 1.0, 0.0);
    let vertex_3_id = mesh.add_vertex(0.0, 1.0, 0.0);

    let triangle_0_id = mesh
        .add_triangle(vertex_0_id, vertex_1_id, vertex_2_id)
        .expect("Impossible de créer le triangle 0");

    let triangle_1_id = mesh
        .add_triangle(vertex_0_id, vertex_2_id, vertex_3_id)
        .expect("Impossible de créer le triangle 1");

    let polygon_0_id = mesh
        .add_polygon(vec![triangle_0_id, triangle_1_id])
        .expect("Impossible de créer le polygon 0");

    mesh.print_summary();
    mesh.print_details();

    println!("Triangles créés : {}, {}", triangle_0_id, triangle_1_id);
    println!("Polygon créé : {}", polygon_0_id);
}
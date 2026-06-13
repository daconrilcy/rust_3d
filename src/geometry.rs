use std::fmt;

#[derive(Debug)]
struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Vertex {
    id: usize,
    position: Point3,
}

#[derive(Debug)]
struct Edge {
    id: usize,
    start_vertex_id: usize,
    end_vertex_id: usize,
}

#[derive(Debug)]
struct Triangle {
    id: usize,
    vertex_ids: [usize; 3],
}

#[derive(Debug)]
struct Polygon {
    id: usize,
    triangle_ids: Vec<usize>,
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    triangles: Vec<Triangle>,
    polygons: Vec<Polygon>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MeshError {
    VertexNotFound(usize),
    EdgeNotFound(usize),
    TriangleNotFound(usize),
    PolygonNotFound(usize),
    EdgeNeedsTwoDistinctVertices,
    TriangleNeedsThreeDistinctVertices,
    PolygonNeedsAtLeastOneTriangle,
}

impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeshError::VertexNotFound(id) => write!(f, "Vertex #{} not found", id),
            MeshError::EdgeNotFound(id) => write!(f, "Edge #{} not found", id),
            MeshError::TriangleNotFound(id) => write!(f, "Triangle #{} not found", id),
            MeshError::PolygonNotFound(id) => write!(f, "Polygon #{} not found", id),
            MeshError::EdgeNeedsTwoDistinctVertices => write!(f, "Edge needs two distinct vertices"),
            MeshError::TriangleNeedsThreeDistinctVertices => write!(f, "Triangle needs three distinct vertices"),
            MeshError::PolygonNeedsAtLeastOneTriangle => write!(f, "Polygon needs at least one triangle"),
        }
    }
}


impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            triangles: Vec::new(),
            polygons: Vec::new(),
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    pub fn polygon_count(&self) -> usize {
        self.polygons.len()
    }
    
    pub fn get_vertex_positions(&self) -> Vec<[f32; 3]> {
        let mut positions = Vec::new();
        for vertex in &self.vertices {
            positions.push([vertex.position.x, vertex.position.y, vertex.position.z]);
        }
        positions
    }

    pub fn get_triangle_indices(&self) -> Vec<[usize; 3]> {
        let mut indices = Vec::new();
        for triangle in &self.triangles {
            indices.push(triangle.vertex_ids);
        }
        indices
    }

    pub fn get_edge_indices(&self) -> Vec<[usize; 2]> {
        let mut indices = Vec::new();
        for edge in &self.edges {
            indices.push([edge.start_vertex_id, edge.end_vertex_id]);
        }
        indices
    }

    pub fn add_vertex(&mut self, x: f32, y: f32, z: f32) -> usize {
        let id = self.vertices.len();

        let vertex = Vertex {
            id,
            position: Point3 { x, y, z },
        };

        self.vertices.push(vertex);

        id
    }

    pub fn add_edge(
        &mut self,
        start_vertex_id: usize,
        end_vertex_id: usize,
    ) -> Result<usize, MeshError> {
        if !self.has_vertex(start_vertex_id) {
            return Err(MeshError::VertexNotFound(start_vertex_id));
        }

        if !self.has_vertex(end_vertex_id) {
            return Err(MeshError::VertexNotFound(end_vertex_id));
        }

        if start_vertex_id == end_vertex_id {
            return Err(MeshError::EdgeNeedsTwoDistinctVertices);
        }

        let id = self.edges.len();

        let edge = Edge {
            id,
            start_vertex_id,
            end_vertex_id,
        };

        self.edges.push(edge);

        Ok(id)
    }

    fn add_edge_if_missing(
        &mut self,
        start_vertex_id: usize,
        end_vertex_id: usize,
    ) -> Result<usize, MeshError> {
        for edge in &self.edges {
            let same_direction =
                edge.start_vertex_id == start_vertex_id && edge.end_vertex_id == end_vertex_id;

            let opposite_direction =
                edge.start_vertex_id == end_vertex_id && edge.end_vertex_id == start_vertex_id;

            if same_direction || opposite_direction {
                return Ok(edge.id);
            }
        }

        self.add_edge(start_vertex_id, end_vertex_id)
    }

    pub fn add_triangle(
        &mut self,
        vertex_0_id: usize,
        vertex_1_id: usize,
        vertex_2_id: usize,
    ) -> Result<usize, MeshError> {
        if !self.has_vertex(vertex_0_id) {
            return Err(MeshError::VertexNotFound(vertex_0_id));
        }

        if !self.has_vertex(vertex_1_id) {
            return Err(MeshError::VertexNotFound(vertex_1_id));
        }

        if !self.has_vertex(vertex_2_id) {
            return Err(MeshError::VertexNotFound(vertex_2_id));
        }

        if vertex_0_id == vertex_1_id || vertex_1_id == vertex_2_id || vertex_2_id == vertex_0_id {
            return Err(MeshError::TriangleNeedsThreeDistinctVertices);
        }

        let id = self.triangles.len();

        self.add_edge_if_missing(vertex_0_id, vertex_1_id)?;
        self.add_edge_if_missing(vertex_1_id, vertex_2_id)?;
        self.add_edge_if_missing(vertex_2_id, vertex_0_id)?;

        let triangle = Triangle {
            id,
            vertex_ids: [vertex_0_id, vertex_1_id, vertex_2_id],
        };

        self.triangles.push(triangle);

        Ok(id)
    }

    pub fn add_polygon(&mut self, triangle_ids: Vec<usize>) -> Result<usize, MeshError> {
        if triangle_ids.is_empty() {
            return Err(MeshError::PolygonNeedsAtLeastOneTriangle);
        }

        for triangle_id in &triangle_ids {
            if !self.has_triangle(*triangle_id) {
                return Err(MeshError::TriangleNotFound(*triangle_id as usize));
            }
        }

        let id = self.polygons.len();

        let polygon = Polygon {
            id,
            triangle_ids,
        };

        self.polygons.push(polygon);

        Ok(id)
    }

    fn has_vertex(&self, vertex_id: usize) -> bool {
        vertex_id < self.vertices.len()
    }

    fn has_triangle(&self, triangle_id: usize) -> bool {
        triangle_id < self.triangles.len()
    }

    pub fn print_summary(&self) {
        println!("Résumé du mesh :");
        println!("Nombre de vertices : {}", self.vertex_count());
        println!("Nombre d'edges : {}", self.edge_count());
        println!("Nombre de triangles : {}", self.triangle_count());
        println!("Nombre de polygons : {}", self.polygon_count());
    }

    pub fn print_details(&self) {
        println!("Détails du mesh :");

        self.print_vertices();
        self.print_edges();
        self.print_triangles();
        self.print_polygons();
    }

    fn print_vertices(&self) {
        println!("Vertices :");

        for vertex in &self.vertices {
            println!(
                "  Vertex #{} => x: {}, y: {}, z: {}",
                vertex.id,
                vertex.position.x,
                vertex.position.y,
                vertex.position.z
            );
        }
    }

    fn print_edges(&self) {
        println!("Edges :");

        for edge in &self.edges {
            println!(
                "  Edge #{} => Vertex #{} -> Vertex #{}",
                edge.id,
                edge.start_vertex_id,
                edge.end_vertex_id
            );
        }
    }

    fn print_triangles(&self) {
        println!("Triangles :");

        for triangle in &self.triangles {
            println!(
                "  Triangle #{} => vertices {:?}",
                triangle.id,
                triangle.vertex_ids
            );
        }
    }

    fn print_polygons(&self) {
        println!("Polygons :");

        for polygon in &self.polygons {
            println!(
                "  Polygon #{} => triangles {:?}",
                polygon.id,
                polygon.triangle_ids
            );
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_triangle_creates_three_edges() {
        let mut mesh = Mesh::new();

        let vertex_0_id = mesh.add_vertex(0.0, 0.0, 0.0);
        let vertex_1_id = mesh.add_vertex(1.0, 0.0, 0.0);
        let vertex_2_id = mesh.add_vertex(1.0, 1.0, 0.0);

        let triangle_result = mesh.add_triangle(vertex_0_id, vertex_1_id, vertex_2_id);

        assert!(triangle_result.is_ok());

        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.edge_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn two_triangles_sharing_an_edge_creates_only_five_edges() {
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

        assert_eq!(triangle_0_id, 0);
        assert_eq!(triangle_1_id, 1);
        assert_eq!(polygon_0_id, 0);

        assert_eq!(mesh.vertex_count(), 4);
        assert_eq!(mesh.edge_count(), 5);
        assert_eq!(mesh.triangle_count(), 2);
        assert_eq!(mesh.polygon_count(), 1);

    }

    #[test]
    fn triangle_with_missing_vertex_returns_an_error() {
        let mut mesh = Mesh::new();

        let vertex_0_id = mesh.add_vertex(0.0, 0.0, 0.0);
        let vertex_1_id = mesh.add_vertex(1.0, 0.0, 0.0);
        
        let result = mesh.add_triangle(vertex_0_id, vertex_1_id, 999);

        assert_eq!(result, Err(MeshError::VertexNotFound(999)));
    }

    #[test]
    fn polygon_with_missing_triangle_returns_an_error() {
        let mut mesh = Mesh::new();

        let result = mesh.add_polygon(vec![999]);

        assert_eq!(result, Err(MeshError::TriangleNotFound(999)));
    }

    #[test]
    fn get_vertex_positions_returns_the_correct_positions() {
        let mut mesh = Mesh::new();

        mesh.add_vertex(0.0, 0.0, 0.0);
        mesh.add_vertex(1.0, 0.0, 0.0);
        mesh.add_vertex(1.0, 1.0, 0.0);

        let positions = mesh.get_vertex_positions();

        assert_eq!(positions, vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
    }
    
    #[test]
    fn get_triangle_indices_returns_the_correct_indices() {
        let mut mesh = Mesh::new();

        let vertex_0_id = mesh.add_vertex(0.0, 0.0, 0.0);
        let vertex_1_id = mesh.add_vertex(1.0, 0.0, 0.0);
        let vertex_2_id = mesh.add_vertex(1.0, 1.0, 0.0);

        mesh.add_triangle(vertex_0_id, vertex_1_id, vertex_2_id).expect("Impossible de créer le triangle");

        let indices = mesh.get_triangle_indices();

        assert_eq!(indices, vec![[vertex_0_id, vertex_1_id, vertex_2_id]]);
    }

    #[test]
    fn get_edge_indices_returns_the_correct_indices() {
        let mut mesh = Mesh::new();

        let vertex_0_id = mesh.add_vertex(0.0, 0.0, 0.0);
        let vertex_1_id = mesh.add_vertex(1.0, 0.0, 0.0);
        let vertex_2_id = mesh.add_vertex(1.0, 1.0, 0.0);

        mesh.add_edge(vertex_0_id, vertex_1_id).expect("Impossible de créer l'edge");
        mesh.add_edge(vertex_1_id, vertex_2_id).expect("Impossible de créer l'edge");
        mesh.add_edge(vertex_2_id, vertex_0_id).expect("Impossible de créer l'edge");

        let indices = mesh.get_edge_indices();

        assert_eq!(indices, vec![[vertex_0_id, vertex_1_id], [vertex_1_id, vertex_2_id], [vertex_2_id, vertex_0_id]]);
    }

    #[test]
    fn edge_with_same_vertex_returns_an_error() {
        let mut mesh = Mesh::new();

        let vertex_0_id = mesh.add_vertex(0.0, 0.0, 0.0);

        let result = mesh.add_edge(vertex_0_id, vertex_0_id);

        assert_eq!(result, Err(MeshError::EdgeNeedsTwoDistinctVertices));
    }

    #[test]
    fn edge_with_missing_vertex_returns_an_error() {
        let mut mesh = Mesh::new();

        let result = mesh.add_edge(999, 0);

        assert_eq!(result, Err(MeshError::VertexNotFound(999)));
    }

    #[test]
    fn triangle_with_same_vertex_returns_an_error() {
        let mut mesh = Mesh::new();

        let vertex_0_id = mesh.add_vertex(0.0, 0.0, 0.0);
        let vertex_1_id = mesh.add_vertex(1.0, 0.0, 0.0);

        let result = mesh.add_triangle(vertex_0_id, vertex_0_id, vertex_1_id);

        assert_eq!(result, Err(MeshError::TriangleNeedsThreeDistinctVertices));
    }

    #[test]
    fn triangle_with_duplicate_vertices_returns_error() {
        let mut mesh = Mesh::new();

        let vertex_0_id = mesh.add_vertex(0.0, 0.0, 0.0);
        let vertex_1_id = mesh.add_vertex(1.0, 0.0, 0.0);


        let result = mesh.add_triangle(vertex_0_id, vertex_1_id, vertex_1_id);

        assert_eq!(result, Err(MeshError::TriangleNeedsThreeDistinctVertices));
    }
    
    #[test]
    fn mesh_error_display_returns_readable_message() {
        let error = MeshError::VertexNotFound(999);

        assert_eq!(
            error.to_string(),
            "Vertex #999 not found"
        );
    }
    
}
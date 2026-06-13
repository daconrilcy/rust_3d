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

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            triangles: Vec::new(),
            polygons: Vec::new(),
        }
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
    ) -> Result<usize, String> {
        if !self.has_vertex(start_vertex_id) {
            return Err(format!("Start vertex #{} does not exist", start_vertex_id));
        }

        if !self.has_vertex(end_vertex_id) {
            return Err(format!("End vertex #{} does not exist", end_vertex_id));
        }

        if start_vertex_id == end_vertex_id {
            return Err(format!(
                "Cannot create an edge from Vertex #{} to itself",
                start_vertex_id
            ));
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
    ) -> Result<usize, String> {
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
    ) -> Result<usize, String> {
        if !self.has_vertex(vertex_0_id) {
            return Err(format!("Vertex #{} does not exist", vertex_0_id));
        }

        if !self.has_vertex(vertex_1_id) {
            return Err(format!("Vertex #{} does not exist", vertex_1_id));
        }

        if !self.has_vertex(vertex_2_id) {
            return Err(format!("Vertex #{} does not exist", vertex_2_id));
        }

        if vertex_0_id == vertex_1_id || vertex_1_id == vertex_2_id || vertex_2_id == vertex_0_id {
            return Err("A triangle needs three distinct vertices".to_string());
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

    pub fn add_polygon(&mut self, triangle_ids: Vec<usize>) -> Result<usize, String> {
        if triangle_ids.is_empty() {
            return Err("A polygon needs at least one triangle".to_string());
        }

        for triangle_id in &triangle_ids {
            if !self.has_triangle(*triangle_id) {
                return Err(format!("Triangle #{} does not exist", triangle_id));
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
        println!("Nombre de vertices : {}", self.vertices.len());
        println!("Nombre d'edges : {}", self.edges.len());
        println!("Nombre de triangles : {}", self.triangles.len());
        println!("Nombre de polygons : {}", self.polygons.len());
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
mod geometry;

use eframe::egui;
use geometry::Mesh;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([800.0, 600.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "Rust 3D Modeler",
        options,
        Box::new(|_creation_context| Ok(Box::new(ModelerApp::default()))),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tool {
    Select,
    DrawTriangle,
}

impl Tool {
    fn label(&self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::DrawTriangle => "Draw Triangle",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewKind {
    Front,
    Left,
    Top,
    Perspective3D,
}

impl ViewKind {
    fn label(&self) -> &'static str {
        match self {
            ViewKind::Front => "FRONT",
            ViewKind::Left => "LEFT",
            ViewKind::Top => "TOP",
            ViewKind::Perspective3D => "3D",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            ViewKind::Front => "Vue de face : axes X / Y",
            ViewKind::Left => "Vue de gauche : axes Z / Y",
            ViewKind::Top => "Vue de dessus : axes X / Z",
            ViewKind::Perspective3D => "Vue 3D perspective",
        }
    }
}

struct ModelerApp {
    mesh: Mesh,
    active_tool: Tool,
    last_message: String,
}

impl Default for ModelerApp {
    fn default() -> Self {
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

        mesh.add_polygon(vec![triangle_0_id, triangle_1_id])
            .expect("Impossible de créer le polygon 0");

        Self { 
            mesh, 
            active_tool: Tool::DrawTriangle, 
            last_message: "Premier affichage graphique du Mesh".to_string() 
        }
    }
}

impl eframe::App for ModelerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show_top_menu(ui);
        self.show_right_panel(ui);
        self.show_central_area(ui);
    }
}

impl ModelerApp {
    fn show_top_menu(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top_menu").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New scene").clicked() {
                        self.last_message = "New scene requested.".to_string();
                        ui.close();
                    }
        
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
        
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        self.last_message = "Undo is not implemented yet.".to_string();
                        ui.close();
                    }
        
                    if ui.button("Redo").clicked() {
                        self.last_message = "Redo is not implemented yet.".to_string();
                        ui.close();
                    }
                });
        
                ui.menu_button("View", |ui| {
                    if ui.button("Reset view").clicked() {
                        self.last_message = "Reset view is not implemented yet.".to_string();
                        ui.close();
                    }
                });
        
                ui.menu_button("Tools", |ui| {
                    if ui.button("Select").clicked() {
                        self.active_tool = Tool::Select;
                        self.last_message = "Select tool activated.".to_string();
                        ui.close();
                    }
        
                    if ui.button("Draw Triangle").clicked() {
                        self.active_tool = Tool::DrawTriangle;
                        self.last_message = "Draw Triangle tool activated.".to_string();
                        ui.close();
                    }
                });
        
                ui.separator();
                ui.label(format!("Active tool: {}", self.active_tool.label()));
            });
        });
    }

    fn show_right_panel(&mut self, ui: &mut egui::Ui) {
        egui::Panel::right("right_panel")
        .size_range(100.0..=300.0)
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.heading("Tools");
            ui.separator();
            ui.label("Active tool");

            ui.radio_value(&mut self.active_tool, Tool::Select, Tool::Select.label());
        
            ui.radio_value(&mut self.active_tool, Tool::DrawTriangle, Tool::DrawTriangle.label());

            ui.separator();

            ui.heading("Mesh");

            ui.label(format!("Vertices: {}", self.mesh.vertex_count()));
            ui.label(format!("Edges: {}", self.mesh.edge_count()));
            ui.label(format!("Triangles: {}", self.mesh.triangle_count()));
            ui.label(format!("Polygons: {}", self.mesh.polygon_count()));

            ui.separator();

            ui.heading("Status");
            ui.label(&self.last_message);

        });
    }

    fn show_central_area(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let available_size = ui.available_size();
    
            let spacing = ui.spacing().item_spacing;
    
            let viewport_width = (available_size.x - spacing.x) / 2.0;
            let viewport_height = (available_size.y - spacing.y) / 2.0;
    
            let viewport_size = egui::vec2(viewport_width, viewport_height);
    
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    self.show_viewport(ui, ViewKind::Front, viewport_size);
                    self.show_viewport(ui, ViewKind::Left, viewport_size);
                });
    
                ui.horizontal(|ui| {
                    self.show_viewport(ui, ViewKind::Top, viewport_size);
                    self.show_viewport(ui, ViewKind::Perspective3D, viewport_size);
                });
            });
        });
    }

    fn show_viewport(&mut self, ui: &mut egui::Ui, view_kind: ViewKind, size: egui::Vec2) {
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    
        let painter = ui.painter();
    
        painter.rect_filled(
            rect,
            0.0,
            egui::Color32::from_gray(25),
        );
    
        painter.rect_stroke(
            rect,
            0.0,
            egui::Stroke::new(1.0, egui::Color32::from_gray(80)),
            egui::StrokeKind::Inside,
        );
    
        painter.text(
            rect.left_top() + egui::vec2(10.0, 10.0),
            egui::Align2::LEFT_TOP,
            view_kind.label(),
            egui::FontId::monospace(16.0),
            egui::Color32::WHITE,
        );
    
        painter.text(
            rect.left_top() + egui::vec2(10.0, 34.0),
            egui::Align2::LEFT_TOP,
            view_kind.description(),
            egui::FontId::monospace(12.0),
            egui::Color32::from_gray(170),
        );
    
        if response.clicked() {
            self.last_message = format!("Clicked in {} view.", view_kind.label());
        }
    }
}

mod geometry;

use std::fmt::format;

use eframe::egui::{self, CursorIcon::ZoomOut, Key::Z, SliderOrientation::{Horizontal, Vertical}};
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

    fn is_drawable(&self) -> bool {
        match self {
            ViewKind::Front => true,
            ViewKind::Left => true,
            ViewKind::Top => true,
            ViewKind::Perspective3D => false,
        }
    }
}

struct ModelerApp {
    mesh: Mesh,
    active_tool: Tool,
    last_message: String,
    zoom: f32,
    pending_triangle_vertex_ids: Vec<usize>,
}

impl Default for ModelerApp {
    fn default() -> Self {
        let mut mesh = Mesh::new();

        Self { 
            mesh, 
            active_tool: Tool::DrawTriangle, 
            last_message: "Premier affichage graphique du Mesh".to_string(),
            zoom: 50.0,
            pending_triangle_vertex_ids: Vec::new(), 
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

            ui.heading("View settings");

            ui.add(
                egui::Slider::new(&mut self.zoom, 10.0..=200.0)
                .text("Zoom"),
            );

            ui.separator();

            ui.heading("Current Triangle");

            ui.label(format!(
                "Pending Vertices: {}/3",
                self.pending_triangle_vertex_ids.len()
            ));

            if ui.button("Cancel current triangle").clicked(){
                self.pending_triangle_vertex_ids.clear();
                self.last_message = "Current triangle cancelled.".to_string();
            }

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

        self.draw_vertices_in_view(painter, rect, view_kind);
    
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
            if let Some(pointer_position) = response.interact_pointer_pos(){

                let world_position = Self::pointer_to_world_position(
                    pointer_position, 
                    rect, 
                    view_kind, 
                    self.zoom,
                );

                match self.active_tool {
                    Tool::Select => {
                        self.last_message = format!(
                             "Select tool clicked in {} view at x={:.2}, y={:.2}, z={:.2}",
                            view_kind.label(),
                            world_position[0],
                            world_position[1],
                            world_position[2],    
                        );
                    }

                    Tool::DrawTriangle => {
                        self.handle_draw_triangle_click(view_kind, world_position);
                    }
                }
        
            }
        }
    }

    fn pointer_to_local_position(pointer_position: egui::Pos2, rect: egui::Rect) -> egui::Vec2{
        egui::vec2 (
            pointer_position.x - rect.left(),
            pointer_position.y - rect.top(),
        )
    }

    fn pointer_to_centered_position(pointer_position: egui::Pos2, rect: egui::Rect) -> egui::Vec2{
        egui::vec2 (
            pointer_position.x - rect.center().x,
            rect.center().y - pointer_position.y,
        )
    }

    fn pointer_to_world_position(
        pointer_position: egui::Pos2,
        rect: egui::Rect,
        view_kind: ViewKind,
        zoom: f32,
    ) -> [f32; 3] {
        
        let centered_position = Self::pointer_to_centered_position(pointer_position, rect);
        
        let world_horizontal = centered_position.x / zoom;
        let world_vertical = centered_position.y / zoom;

        match view_kind {
            ViewKind::Front => [
                world_horizontal,
                world_vertical,
                0.0,
            ],

            ViewKind::Left => [
                0.0,
                world_vertical,
                world_horizontal,
            ],

            ViewKind::Top => [
                world_horizontal,
                world_vertical,
                0.0,
            ],

            ViewKind::Perspective3D => [
                world_horizontal,
                world_vertical,
                0.0,
            ],
        }
    }

    fn handle_draw_triangle_click(&mut self, view_kind: ViewKind, world_position: [f32; 3]){
        if !view_kind.is_drawable(){
            self.last_message = "For now, draw triangles only in FRONT, LEFT or TOP view".to_string();
            return;
        }

        let vertex_id = self.mesh.add_vertex(
            world_position[0],
            world_position[1], 
            world_position[2]
        );

        self.pending_triangle_vertex_ids.push(vertex_id);

        if self.pending_triangle_vertex_ids.len() < 3 {
            self.last_message = format!(
                 "Vertex #{} created in {} view. Pending triangle: {}/3 vertices.",
                vertex_id,
                view_kind.label(),
                self.pending_triangle_vertex_ids.len()
            );

            return;
        }

        let vertex_0_id = self.pending_triangle_vertex_ids[0];
        let vertex_1_id = self.pending_triangle_vertex_ids[1];
        let vertex_2_id = self.pending_triangle_vertex_ids[2];

        match self
            .mesh
            .add_triangle(vertex_0_id, vertex_1_id, vertex_2_id)
            {
                Ok(triangle_id) => {
                    self.last_message = format!(
                        "Triangle #{} created with vertices #{}, #{}, #{}.",
                        triangle_id,
                        vertex_0_id,
                        vertex_1_id,
                        vertex_2_id
                    );
                }

            Err(error) => {
                self.last_message = format!("Could not create triangle: {}", error);
            }
        }

        self.pending_triangle_vertex_ids.clear();

    }

    fn world_to_screen_position(
        world_position: [f32; 3],
        rect: egui::Rect,
        view_kind: ViewKind,
        zoom: f32,
    ) -> egui::Pos2 {
        let x = world_position[0];
        let y = world_position[1];
        let z = world_position[2];

        let (horizontal, vertical) = match view_kind {
            ViewKind::Front => {
                //Vue de Face : on voit x/y
                (x,y)
            }

            ViewKind::Left =>{
                //Vue de gauche : on voit z/y
                (z,y)
            }

            ViewKind::Top => {
                // Vue du dessus : on voit x/z
                (x,z)
            }

            ViewKind::Perspective3D => {
                //Projection 3D temporaire simpliste
                //Pas encore OpenGL

                let iso_x = x-z * 0.8;
                let iso_y = y - (x+z) * 0.35;

                (iso_x, iso_y)
            }
        };

        egui::pos2 ( 
            rect.center().x + horizontal * zoom, 
            rect.center().y - vertical * zoom,
        )
    }

    fn draw_vertices_in_view(
        &self,
        painter: &egui::Painter,
        rect: egui::Rect,
        view_kind: ViewKind,
    ){
        let vertex_positions = self.mesh.get_vertex_positions();

        for (vertex_id, world_position) in vertex_positions.iter().enumerate(){
            let screen_position = Self::world_to_screen_position(
                *world_position, 
                rect, 
                view_kind, 
                self.zoom,
            );

            let is_pending = self.pending_triangle_vertex_ids.contains(&vertex_id);

            let radius = if is_pending {
                5.0
            } else {
                4.0
            };

            let color = if is_pending {
                egui::Color32::YELLOW
            } else {
                egui::Color32::WHITE
            };

            painter.circle_filled(
                screen_position, 
                radius, 
                color,
            );

            painter.text(
                screen_position+ egui::vec2(6.0, -6.0),
                egui::Align2::LEFT_BOTTOM,
                format!("#{}", vertex_id),
                egui::FontId::monospace(10.0),
                egui::Color32::from_gray(180),
            );
        }
    }

}

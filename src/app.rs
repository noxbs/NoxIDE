use std::path::PathBuf;

use eframe::egui::{self, Color32, RichText, TextEdit};
use rfd::FileDialog;

use crate::{document::Document, nox, syntax, workspace::Workspace, workspace_tree::TreeNode};

const PLUM: Color32 = Color32::from_rgb(55, 39, 72);
const SURFACE: Color32 = Color32::from_rgb(48, 34, 63);
const ACCENT: Color32 = Color32::from_rgb(196, 167, 231);

pub struct NoxIdeApp {
    workspace: Workspace,
    documents: Vec<Document>,
    active_document: Option<usize>,
    output: String,
    status: String,
    tree: TreeNode,
}

impl NoxIdeApp {
    pub fn new(context: &eframe::CreationContext<'_>) -> Self {
        configure_theme(&context.egui_ctx);
        let workspace = Workspace::discover(PathBuf::from(".").as_path());
        Self {
            tree: TreeNode::workspace(workspace.root.clone()),
            workspace,
            documents: Vec::new(),
            active_document: None,
            output: "Nox output\n".into(),
            status: "Ready".into(),
        }
    }

    fn open_file(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            self.open_path(path);
        }
    }

    fn open_path(&mut self, path: PathBuf) {
        match Document::open(path.clone()) {
            Ok(document) => {
                self.workspace = Workspace::discover(&path);
                self.tree = TreeNode::workspace(self.workspace.root.clone());
                self.documents.push(document);
                self.active_document = Some(self.documents.len() - 1);
                self.status = format!("Opened {}", path.display());
            }
            Err(error) => self.output = format!("Open failed: {error}"),
        }
    }

    fn save_current(&mut self) {
        let Some(index) = self.active_document else {
            return;
        };
        match self.documents[index].save() {
            Ok(()) => self.status = format!("Saved {}", self.documents[index].path.display()),
            Err(error) => self.output = format!("Save failed: {error}"),
        }
    }

    fn run_nox(&mut self, command: &str) {
        match nox::run(&self.workspace.root, command) {
            Ok(result) => {
                self.output = format!(
                    "{}\n{}",
                    result.text,
                    result
                        .status
                        .map_or_else(|| "terminated".into(), |code| format!("exit {code}"))
                )
            }
            Err(error) => self.output = format!("Nox failed: {error}"),
        }
    }
}

impl eframe::App for NoxIdeApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(context, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("NOXIDE").color(Color32::WHITE).strong());
                ui.label(
                    RichText::new("focused code editor").color(Color32::from_rgb(185, 167, 199)),
                );
                ui.add_space(18.0);
                if ui.button("Open file").clicked() {
                    self.open_file();
                }
                if ui.button("Save").clicked() {
                    self.save_current();
                }
                if ui
                    .add(egui::Button::new(RichText::new("Build").color(PLUM)))
                    .clicked()
                {
                    self.run_nox("build");
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(self.status.clone()).color(ACCENT));
                });
            });
        });

        egui::SidePanel::left("explorer")
            .resizable(true)
            .default_width(250.0)
            .show(context, |ui| {
                ui.label(RichText::new("EXPLORER").color(ACCENT).strong());
                ui.separator();
                ui.label(
                    RichText::new(self.workspace.root.display().to_string()).color(Color32::WHITE),
                );
                ui.add_space(8.0);
                if let Some(path) = explorer_node(ui, &self.tree) {
                    self.open_path(path);
                }
            });

        egui::TopBottomPanel::bottom("output")
            .resizable(true)
            .default_height(150.0)
            .show(context, |ui| {
                ui.label(RichText::new("NOX OUTPUT").color(ACCENT).strong());
                ui.separator();
                ui.add(
                    TextEdit::multiline(&mut self.output)
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(5)
                        .interactive(false),
                );
            });

        egui::CentralPanel::default().show(context, |ui| {
            if self.documents.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new("Open a source file to start editing")
                            .size(18.0)
                            .color(Color32::from_rgb(210, 196, 222)),
                    );
                });
                return;
            }
            ui.horizontal_wrapped(|ui| {
                for (index, document) in self.documents.iter().enumerate() {
                    let title = format!(
                        "{}{}",
                        if document.dirty { "● " } else { "" },
                        document
                            .path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                    );
                    if ui
                        .selectable_label(self.active_document == Some(index), title)
                        .clicked()
                    {
                        self.active_document = Some(index);
                    }
                }
            });
            ui.separator();
            if let Some(index) = self.active_document {
                let extension = self.documents[index]
                    .path
                    .extension()
                    .and_then(|value| value.to_str())
                    .unwrap_or("")
                    .to_owned();
                let mut layouter = |ui: &egui::Ui, text: &str, width: f32| {
                    syntax::layout_job(ui, text, width, &extension)
                };
                let response = ui.add(
                    TextEdit::multiline(&mut self.documents[index].text)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .desired_rows(30)
                        .frame(false)
                        .layouter(&mut layouter),
                );
                if response.changed() {
                    self.documents[index].dirty = true;
                }
            }
        });
    }
}

fn explorer_node(ui: &mut egui::Ui, node: &TreeNode) -> Option<PathBuf> {
    if node.directory {
        let mut selected = None;
        egui::CollapsingHeader::new(RichText::new(format!("▾  {}", node.name)).color(ACCENT))
            .default_open(true)
            .show(ui, |ui| {
                for child in &node.children {
                    if let Some(path) = explorer_node(ui, child) {
                        selected = Some(path);
                    }
                }
            });
        selected
    } else if ui
        .selectable_label(false, format!("    {}", node.name))
        .clicked()
    {
        Some(node.path.clone())
    } else {
        None
    }
}

fn configure_theme(context: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_fill = SURFACE;
    visuals.panel_fill = PLUM;
    visuals.extreme_bg_color = Color32::from_rgb(38, 27, 49);
    visuals.faint_bg_color = Color32::from_rgb(74, 52, 93);
    visuals.selection.bg_fill = Color32::from_rgb(101, 75, 120);
    visuals.selection.stroke.color = ACCENT;
    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(106, 77, 126);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(90, 66, 109);
    context.set_visuals(visuals);
}

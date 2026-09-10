mod app;
mod document;
mod nox;
mod syntax;
mod workspace;
mod workspace_tree;

fn main() -> eframe::Result {
    eframe::run_native(
        "NoxIDE",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([1280.0, 820.0])
                .with_min_inner_size([900.0, 560.0]),
            ..Default::default()
        },
        Box::new(|creation_context| Ok(Box::new(app::NoxIdeApp::new(creation_context)))),
    )
}

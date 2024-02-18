mod search;
use crate::search::search_pdf;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "pdf folder search",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    search: String,
    picked_path: String,
    output: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search: "lorem".to_owned(),
            picked_path: "/home/example".to_owned(),
            output: "test".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PDF Suche");

            ui.horizontal(|ui| {
                let name_label = ui.label("Suche nach: ");
                ui.text_edit_singleline(&mut self.search)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                let _folder_label = ui.label("PDF Ordner: ");
                if ui.button("Auswählen").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.picked_path = path.display().to_string();
                    }
                }
            });

            ui.label(format!(
                "Suche nach '{}', im Ordner: {}",
                self.search, self.picked_path
            ));

            ui.label(format!(
                "{}",
                search_pdf(self.picked_path.clone(), self.search.clone())
            ));
        });
    }
}

use crate::search_pdf;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Search {
    search: String,
    picked_path: String,
    output: String,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            search: String::from("Lorem"),
            picked_path: String::from("example/"),
            output: String::default(),
        }
    }
}

impl Search {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for Search {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //ctx.set_pixels_per_point(2.0);
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

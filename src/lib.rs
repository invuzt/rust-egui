use eframe::egui;

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let options = eframe::NativeOptions::default();
    
    // Kita panggil melalui jalur winit yang lebih eksplisit jika root fail
    eframe::winit::run_android_app(app, options, Box::new(|_cc| Box::new(MyApp::default())))
        .expect("Failed to run eframe");
}

struct MyApp {
    text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: "Halo Ponorogo!".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui Android");
            ui.separator();
            ui.text_edit_singleline(&mut self.text);
            ui.label(format!("Isi: {}", self.text));
            if ui.button("Klik").clicked() {
                self.text = "Berhasil!".to_owned();
            }
        });
    }
}

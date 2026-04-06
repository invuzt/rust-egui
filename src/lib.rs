use eframe::egui;

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let options = eframe::NativeOptions::default();
    
    // Langsung panggil dari root eframe, ini adalah standar API untuk mobile
    eframe::run_android_app(app, options, Box::new(|_cc| Box::new(MyApp::default())))
        .expect("Failed to run eframe");
}

struct MyApp {
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Odfiz User".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui Android Berhasil!");
            ui.separator();
            ui.label(format!("Halo, {}!", self.name));
            ui.text_edit_singleline(&mut self.name);
        });
    }
}

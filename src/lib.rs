use eframe::egui;

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let options = eframe::NativeOptions::default();
    
    // Memanggil langsung dari root eframe
    eframe::run_android_app(app, options, Box::new(|_cc| Box::new(MyApp::default())))
        .expect("Gagal menjalankan eframe");
}

struct MyApp {
    nama: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            nama: "Guru Ponorogo".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui x Odfiz Berhasil!");
            ui.separator();
            ui.label(format!("Halo, {}!", self.nama));
            ui.text_edit_singleline(&mut self.nama);
            
            if ui.button("Klik Saya").clicked() {
                self.nama = "Rust itu Keren!".to_owned();
            }
        });
    }
}

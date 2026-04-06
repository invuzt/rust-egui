use eframe::egui;

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let options = eframe::NativeOptions::default();
    
    // Fungsi ini muncul hanya jika fitur android-native-activity di Cargo.toml aktif
    eframe::run_android_app(app, options, Box::new(|_cc| Box::new(MyApp::default())))
        .expect("Gagal menjalankan eframe");
}

struct MyApp {
    teks: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            teks: "Halo Ponorogo!".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui x Odfiz Berhasil!");
            ui.separator();
            ui.label(format!("Isi teks: {}", self.teks));
            ui.text_edit_singleline(&mut self.teks);
            
            if ui.button("Klik Saya").clicked() {
                self.teks = "Rust itu Powerfull!".to_owned();
            }
        });
    }
}

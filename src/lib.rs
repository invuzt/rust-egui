use eframe::egui;

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use eframe::NativeOptions;

    let options = NativeOptions {
        ..Default::default()
    };

    // Pada versi 0.27, eframe menggunakan run_native yang di-proxy untuk Android
    // Jika fungsi spesifik tidak ditemukan, kita panggil lewat entry point ini
    eframe::run_native(
        "Egui Android Demo",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ).expect("Failed to run eframe");
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Odfiz".to_owned(),
            age: 40,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui Android Berhasil!");
            ui.separator();
            ui.label(format!("Halo {}, umur kamu {}", self.name, self.age));
            
            ui.horizontal(|ui| {
                ui.label("Edit Nama: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("Umur"));
            
            if ui.button("Tambah Umur").clicked() {
                self.age += 1;
            }
        });
    }
}

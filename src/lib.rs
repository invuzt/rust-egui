use eframe::egui;

#[no_mangle]
fn android_main(app: eframe::android_activity::AndroidApp) {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_android_app(app, options, Box::new(|_cc| Box::new(MyApp::default()))).unwrap();
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
            ui.horizontal(|ui| {
                ui.label("Nama: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("Umur"));
            if ui.button("Tambah Umur").clicked() {
                self.age += 1;
            }
            ui.label(format!("Halo {}, umur kamu {}", self.name, self.age));
        });
    }
}

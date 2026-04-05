use eframe::egui;

#[no_mangle]
fn android_main(app: eframe::android_activity::AndroidApp) {
    let options = eframe::NativeOptions::default();
    
    // eframe 0.27 mengekspor fungsi ini jika fitur android diaktifkan
    eframe::run_android_app(app, options, Box::new(|_cc| Box::new(MyApp::default())))
        .expect("Failed to run eframe");
}

struct MyApp {
    text: String,
    value: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: "Halo Ponorogo!".to_owned(),
            value: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui Android - Sukses!");
            ui.add_space(10.0);
            
            ui.label("Input teks di bawah:");
            ui.text_edit_singleline(&mut self.text);
            
            ui.add_space(10.0);
            ui.add(egui::Slider::new(&mut self.value, 0.0..=100.0).text("Slide me"));
            
            ui.label(format!("Teks: {}", self.text));
        });
    }
}

use eframe::{egui, App, Frame};
use eframe::egui::{CentralPanel, Ui};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello World!",
        options,
        Box::new(|_cc|
            Ok(Box::new(MyApp::new()))
        ),
    )
}

struct MyApp {
    text: String,
    value: f64
}

impl MyApp {
    fn new() -> Self {
        MyApp { text: String::from("Test String"), value: 0.0 }
    }
}

impl App for MyApp {

fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
    egui::Window::new("Hello World").show(ui, |ui| {
        ui.heading("Egui Window");
    });

    CentralPanel::default().show(ui, |ui| {
        ui.heading("Egui CentralPanel");
        ui.horizontal(|ui| {
            ui.label("Text box: ");
            ui.text_edit_singleline(&mut self.text);
        });
        let slider = egui::Slider::new(&mut self.value, 0.0..=120.0).text("value");
        ui.add(slider);
        if ui.button("Increment").clicked() {
            self.value += 1.0;
        }
        ui.label(format!("Hello '{}', value: {}", self.text, self.value));
    });
}

}

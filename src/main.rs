use eframe::egui;
use eframe::emath::GuiRounding;
use eframe::wgpu::wgc::binding_model::BindingZone::Stage;
use hesap::DisplayNumber;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 450.0])
            .with_min_inner_size([300.0, 450.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hesap",
        options,
        Box::new(|_cc|
            Ok(Box::new(MyApp::default()))
        ),
    )
}

const MAX_DIGITS: usize = 15;

struct MyApp {
    // Keep current display as string to not have to worry about rounding errors
    // This is "kind" of like a decimal representation as this is a vec of u8
    display: DisplayNumber,
    memory: f64,
}

impl MyApp {
    fn reset(&mut self) {
        self.memory = 0.0;
        self.display.clear();
    }

    #[allow(clippy::cast_precision_loss)]
    fn buttons(&mut self, ui: &mut egui::Ui, spacing: f32) {
        let layout = [
            [ "CE", "C", "%", "/"],
            [  "7", "8", "9", "*"],
            [  "4", "5", "6", "-"],
            [  "1", "2", "3", "+"],
            ["+/-", "0", ".", "="],
        ];

        let rows = layout.len() as f32;
        let columns = layout[0].len() as f32;

        let btn_size = egui::vec2(
            (ui.available_width()) / columns - spacing * (columns - 1.0) / columns,
            (ui.available_height()) / rows - spacing * (rows - 1.0) / rows,
        );

        let font_size = btn_size.min_elem() / 2.5;
        // Round to every 5 pixel multiple to stop unneeded font changes
        let font_size = font_size.round_to_pixels(1.0 / 5.0);

        // button creation from: https://www.youtube.com/watch?v=hrFHcQXxGbs
        for row in &layout {
            ui.horizontal(|ui| {
                for &label in row {
                    enum ButtonType {
                        Operator,
                        Other,
                        Number
                    }

                    let button_type = match label {
                        "/" | "*" | "-" | "+" | "=" => ButtonType::Operator,
                        "CE" | "C" | "%" | "+/-" | "." => ButtonType::Other,
                        _ => ButtonType::Number,
                    };

                    let txt = egui::RichText::new(label).size(font_size).monospace();

                    let button = match button_type {
                        ButtonType::Operator =>
                            egui::Button::new(txt.color(egui::Color32::WHITE))
                                .fill(egui::Color32::from_rgb(255, 149, 0)),
                        ButtonType::Other =>
                            egui::Button::new(txt)
                                .fill(egui::Color32::from_rgb(80, 80, 80)),
                        ButtonType::Number =>
                            egui::Button::new(txt)
                                .fill(egui::Color32::from_rgb(50, 50, 55))
                    };

                    if ui.add_sized(btn_size, button).clicked() {
                        // TODO: use enum for layout instead of string
                        match button_type {
                            ButtonType::Operator => todo!(),
                            ButtonType::Other => {
                                match label {
                                    "CE" | "C" => self.reset(),
                                    "+/-" => self.display.swap_sign(),
                                    "." => self.display.be_fractional(),
                                    _ => todo!()
                                }
                            }
                            ButtonType::Number => {
                                let digits = self.display.digits_used();
                                if digits < MAX_DIGITS {
                                    self.display.add_number(label);
                                }
                            }
                        }
                    }
                }
            });
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            memory: 0.0,
            display: DisplayNumber::default()
        }
    }
}

impl eframe::App for MyApp {

fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ui, |ui| {
        let spacing = 4.0;
        ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

        let display_size = egui::vec2(
            ui.available_width(),
            ui.available_height() * 0.25,
        );

        ui.allocate_ui(
            display_size,
            |ui| {
                // copy how the buttons look
                let rounding = ui.visuals().widgets.inactive.corner_radius;

                let font_size = display_size.y / 4.0;
                // Round to every 5 pixel multiple to stop unneeded font changes
                let font_size = font_size.round_to_pixels(1.0 / 5.0);

                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(30, 30, 40))
                    .corner_radius(rounding)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                let label = egui::Label::new(
                                    egui::RichText::new(self.display.to_string())
                                        .size(font_size)
                                        .color(egui::Color32::WHITE)
                                        .monospace()
                                ).truncate();
                                ui.add(label);
                            }
                        );
                    });
            }
        );

        // makes spacing 2x
        ui.add_space(spacing);

        // Use up the rest of the space for buttons
        self.buttons(ui, spacing);
    });
}

}

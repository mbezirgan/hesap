use eframe::egui;
use eframe::emath::GuiRounding;

use rust_decimal::prelude::*;

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

enum CalculatorMode {
    Input,
    Addition, Subtraction, Multiplication, Division
}

struct MyApp {
    // Keep current display as string to not have to worry about rounding errors
    // This is "kind" of like a decimal representation as this is a vec of u8
    input: DisplayNumber,
    memory: Decimal,
    mode: CalculatorMode
}

impl MyApp {
    fn clear_entry(&mut self) {
        self.input.clear();
        self.mode = CalculatorMode::Input;
    }

    fn clear(&mut self) {
        self.memory = Decimal::zero();
        self.mode = CalculatorMode::Input;
        self.clear_entry();
    }

    #[must_use]
    fn evaluate(&self) -> Decimal {
        let left = self.memory;
        let right = self.input.to_decimal();
        match self.mode {
            CalculatorMode::Input => left,
            CalculatorMode::Addition => left + right,
            CalculatorMode::Subtraction => left - right,
            CalculatorMode::Multiplication => left * right,
            CalculatorMode::Division => left / right
        }
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
                            ButtonType::Operator => {
                                if matches!(self.mode, CalculatorMode::Input) {
                                    self.memory = self.input.to_decimal();
                                    self.input.clear();
                                }
                                match label {
                                    "/" => self.mode = CalculatorMode::Division,
                                    "*" => self.mode = CalculatorMode::Multiplication,
                                    "-" => self.mode = CalculatorMode::Subtraction,
                                    "+" => self.mode = CalculatorMode::Addition,
                                    "=" => {
                                        let result = self.evaluate();
                                        self.input.set_decimal(result);
                                        self.mode = CalculatorMode::Input;
                                    },
                                    _ => todo!()
                                }
                            },
                            ButtonType::Other => {
                                match label {
                                    "C" => self.clear(),
                                    "CE" => self.clear_entry(),
                                    "+/-" => self.input.swap_sign(),
                                    "." => self.input.be_fractional(),
                                    "%" => {
                                        /* NOTE: we could use the string representation
                                           and just move the decimal */
                                        let percentage = self.input.to_f64();
                                        let value = percentage / 100.0;
                                        self.input.set_f64(value);
                                    }
                                    _ => todo!()
                                }
                            }
                            ButtonType::Number => {
                                let digits = self.input.digits_used();
                                if digits < MAX_DIGITS {
                                    self.input.add_number(label);
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
            memory: Decimal::zero(),
            input: DisplayNumber::default(),
            mode: CalculatorMode::Input
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
                                    egui::RichText::new(self.input.to_string())
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

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use eframe::emath::GuiRounding;

use rust_decimal::prelude::*;

use hesap::{Digit, DisplayNumber};

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

#[derive(Debug, Clone, Copy)]
enum ButtonType {
    Operator,
    Other,
    Number
}


#[derive(Debug, Clone, Copy)]
enum Button {
    ClearEntry, Clear,
    Number(Digit),
    Multiply, Minus, Plus, Divide,
    SignChange, Percentage,
    Period,
    Evaluate,
}

impl Button {
    fn label(&self) -> &'static str {
        match self {
            Button::ClearEntry => "CE",
            Button::Clear => "C",
            Button::Number(digit) => digit.value_as_str(),
            Button::Multiply => "*",
            Button::Minus => "-",
            Button::Plus => "+",
            Button::Divide => "/",
            Button::SignChange => "+/-",
            Button::Percentage => "%",
            Button::Period => ".",
            Button::Evaluate => "=",
        }
    }

    fn button_type(&self) -> ButtonType {
        match self {
            Button::Divide | Button::Multiply | Button::Minus | Button::Plus | Button::Evaluate => crate::ButtonType::Operator,
            Button::ClearEntry | Button::Clear | Button::Percentage | Button::SignChange | Button::Period => ButtonType::Other,
            _ => ButtonType::Number,
        }
    }

    fn egui_button(self, font_size: f32) -> egui::Button<'static> {
        let txt = egui::RichText::new(self.label()).size(font_size).monospace();

        match self.button_type() {
            ButtonType::Operator =>
                egui::Button::new(txt.color(egui::Color32::WHITE))
                    .fill(egui::Color32::from_rgb(255, 149, 0)),
            ButtonType::Other =>
                egui::Button::new(txt)
                    .fill(egui::Color32::from_rgb(80, 80, 80)),
            ButtonType::Number =>
                egui::Button::new(txt)
                    .fill(egui::Color32::from_rgb(50, 50, 55))
        }
    }
}

struct MyApp {
    input: DisplayNumber,
    error: Option<&'static str>,
    memory: Decimal,
    mode: CalculatorMode
}

impl MyApp {
    fn clear_entry(&mut self) {
        self.input.clear();
        self.error = None;
    }

    fn clear(&mut self) {
        self.memory = Decimal::zero();
        self.mode = CalculatorMode::Input;
        self.clear_entry();
    }

    fn evaluate(&self) -> Result<Decimal, &'static str> {
        let left = self.memory;
        let right = self.input.to_decimal();
        match self.mode {
            CalculatorMode::Input => Ok(left),
            CalculatorMode::Addition => left.checked_add(right).ok_or("Addition overflow"),
            CalculatorMode::Subtraction => left.checked_sub(right).ok_or("Subtraction overflow"),
            CalculatorMode::Multiplication => left.checked_mul(right).ok_or("Multiplication overflow"),
            CalculatorMode::Division => {
                if right == Decimal::zero() {
                    return Err("Division by zero");
                }
                left.checked_div(right).ok_or("Division overflow")
            }
        }
    }

    fn enter(&mut self) {
        match self.evaluate() {
            Ok(value) => {
                self.input.set_decimal(value);
                self.mode = CalculatorMode::Input;
            }
            Err(err) => {
                self.error = Some(err);
            }
        }
    }

    fn input_to_percentage(&mut self) {
        /* NOTE: we could use the string representation
           and just move the decimal */
        let percentage = self.input.to_decimal();
        let value = percentage / Decimal::from(100);
        self.input.set_decimal(value);
    }

    fn on_button_press(&mut self, button: Button) {
        match button.button_type() {
            ButtonType::Operator => {
                if matches!(self.mode, CalculatorMode::Input) {
                    self.memory = self.input.to_decimal();
                    self.input.clear();
                }
                match button {
                    Button::Divide => self.mode = CalculatorMode::Division,
                    Button::Multiply => self.mode = CalculatorMode::Multiplication,
                    Button::Minus => self.mode = CalculatorMode::Subtraction,
                    Button::Plus => self.mode = CalculatorMode::Addition,
                    Button::Evaluate => self.enter(),
                    _ => unreachable!()
                }
            },
            ButtonType::Other => {
                match button {
                    Button::Clear => self.clear(),
                    Button::ClearEntry => self.clear_entry(),
                    Button::SignChange => self.input.swap_sign(),
                    Button::Period => self.input.be_fractional(),
                    Button::Percentage => self.input_to_percentage(),
                    _ => unreachable!()
                }
            }
            ButtonType::Number => {
                let digits = self.input.digits_used();
                if digits < MAX_DIGITS && let Button::Number(digit) = button {
                    self.input.add_digit(digit);
                }
            }
        }
    }

    #[must_use]
    fn display_output(&self) -> String {
        match self.error {
            Some(err) => err.to_owned(),
            None => self.input.to_string(),
        }
    }

    #[allow(clippy::enum_glob_use)]
    const LAYOUT: [[Button; 4]; 5] =  {
        use Button::*;
        use Digit::*;
        [
            [ ClearEntry,    Clear,         Percentage,    Divide   ],
            [ Number(Seven), Number(Eight), Number(Nine),  Multiply ],
            [ Number(Four),  Number(Five),  Number(Six),   Minus    ],
            [ Number(One),   Number(Two),   Number(Three), Plus     ],
            [ SignChange,    Number(Zero),  Period,        Evaluate ],
        ]
    };

    #[allow(clippy::cast_precision_loss)]
    fn buttons(&mut self, ui: &mut egui::Ui, spacing: f32) {
        let rows = Self::LAYOUT.len() as f32;
        let columns = Self::LAYOUT[0].len() as f32;

        let btn_size = egui::vec2(
            (ui.available_width()) / columns - spacing * (columns - 1.0) / columns,
            (ui.available_height()) / rows - spacing * (rows - 1.0) / rows,
        );

        let font_size = btn_size.min_elem() / 2.5;
        // Round to every 5 pixel multiple to stop unneeded font changes
        let font_size = font_size.round_to_pixels(1.0 / 5.0);

        // button creation from: https://www.youtube.com/watch?v=hrFHcQXxGbs
        for row in &Self::LAYOUT {
            ui.horizontal(|ui| {
                for &button in row {
                    if ui.add_sized(btn_size, button.egui_button(font_size)).clicked() {
                        self.on_button_press(button);
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
            error: None,
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
                                    egui::RichText::new(self.display_output())
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

    let mut copy_txt: Option<String> = None;

    // Keyboard Input
    ui.input_mut(|i| {
        for event in &i.events {
            // Do it this way to handle potentially non-standard keyboard layouts
            match event {
                egui::Event::Key { key: egui::Key::Enter, pressed: true, .. } => {
                    println!("Pressed Enter");
                    self.on_button_press(Button::Evaluate);
                }
                egui::Event::Key { key: egui::Key::Backspace, pressed: true, .. } => {
                    println!("Pressed Backspace");
                    self.input.remove_char();
                },
                // NOTE: Paste and Copy handle proper event order
                // (like user trying to copy from a text field) so this won't intefier with that
                egui::Event::Paste(text) => {
                    println!("Clipboard paste: {text}");
                    // Use decimal libraries input handling
                    match Decimal::from_str(text) {
                        Ok(decimal) => self.input.set_decimal(decimal),
                        Err(_) => self.error = Some("Invalid Input"),
                    }
                }
                egui::Event::Copy => {
                    let display = self.display_output();
                    println!("Clipboard copy: {display}");
                    copy_txt = Some(display);
                }
                egui::Event::Text(text) => {
                    println!("Text input: {text}");

                    let text = &text[..];
                    let button = match text {
                        "/" => Button::Divide,
                        "*" => Button::Multiply,
                        "-" => Button::Minus,
                        "+" => Button::Plus,
                        "=" => Button::Evaluate,
                        "c" => Button::Clear,
                        "C" => Button::ClearEntry,
                        "%" => Button::Percentage,
                        "s" => Button::SignChange,
                        "." | "," => Button::Period,
                        "0" => Button::Number(Digit::Zero),
                        "1" => Button::Number(Digit::One),
                        "2" => Button::Number(Digit::Two),
                        "3" => Button::Number(Digit::Three),
                        "4" => Button::Number(Digit::Four),
                        "5" => Button::Number(Digit::Five),
                        "6" => Button::Number(Digit::Six),
                        "7" => Button::Number(Digit::Seven),
                        "8" => Button::Number(Digit::Eight),
                        "9" => Button::Number(Digit::Nine),
                        _ => continue,
                    };

                    self.on_button_press(button);
                }
                _ => (),
            }
        }
    });

    // Deffer copy_text so that ui isn't modified inside input_mut
    if let Some(str) = copy_txt {
        ui.copy_text(str);
    }
}

}

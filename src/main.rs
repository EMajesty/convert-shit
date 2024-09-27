#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    // env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 620.0])
            // .with_decorations(false)
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "convert shit",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<App>::default())
        })
    )
}

#[derive(Debug,PartialEq)]
enum InputType {
    Hex,
    Dec,
    Ascii,
    Binary,
}

struct App {
    button_states: Vec<bool>,
    input: String,
    input_type: InputType,
    output_hex: String,
    output_dec: String,
    output_asc: String,
    output_bin: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            button_states: vec![false, true, true, true],
            input: "".to_owned(),
            input_type: InputType::Hex,
            output_hex: "".to_owned(),
            output_dec: "".to_owned(),
            output_asc: "".to_owned(),
            output_bin: "".to_owned(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    ui.add(
                        egui::Image::new(egui::include_image!("../assets/lain_sad.png")).max_width(128.0)
                    );

                    ui.horizontal(|ui| {
                        if ui.add_enabled(self.button_states[0], egui::Button::new("hex")).clicked() { 
                            self.input_type = InputType::Hex;
                            self.button_states = vec![true; 4];
                            self.button_states[0] = false;
                            self.input = "".to_owned();
                        }
                        if ui.add_enabled(self.button_states[1], egui::Button::new("dec")).clicked() { 
                            self.input_type = InputType::Dec;
                            self.button_states = vec![true; 4];
                            self.button_states[1] = false;
                            self.input = "".to_owned();
                        }
                        if ui.add_enabled(self.button_states[2], egui::Button::new("asc")).clicked() { 
                            self.input_type = InputType::Ascii;
                            self.button_states = vec![true; 4];
                            self.button_states[2] = false;
                            self.input = "".to_owned();
                        }
                        if ui.add_enabled(self.button_states[3], egui::Button::new("bin")).clicked() { 
                            self.input_type = InputType::Binary;
                            self.button_states = vec![true; 4];
                            self.button_states[3] = false;
                            self.input = "".to_owned();
                        }
                    });

                    if ui.code_editor(&mut self.input).changed() {
                        // self.output_hex = self.input.to_owned();
                        let hex = input_to_hex(&self.input, &self.input_type);
                        self.output_hex = hex.to_owned();
                        self.output_dec = hex_to_dec(&hex);
                        self.output_asc = hex_to_asc(&hex);
                        self.output_bin = hex_to_bin(&hex);
                    }

                    ui.add_space(20.0);

                    let output_hex_label = ui.label("hex");
                    ui.code_editor(&mut self.output_hex)
                        .labelled_by(output_hex_label.id);

                    ui.add_space(10.0);

                    let output_hex_label = ui.label("dec");
                    ui.code_editor(&mut self.output_dec)
                        .labelled_by(output_hex_label.id);

                    ui.add_space(10.0);

                    let output_hex_label = ui.label("asc");
                    ui.code_editor(&mut self.output_asc)
                        .labelled_by(output_hex_label.id);

                    ui.add_space(10.0);

                    let output_hex_label = ui.label("bin");
                    ui.code_editor(&mut self.output_bin)
                        .labelled_by(output_hex_label.id);
                });
            });
        });
    }
}

fn input_to_hex(input: &String, input_type: &InputType) -> String {
    let mut hex = "".to_owned();

    match input_type {
        InputType::Hex => hex = input.to_owned(), 
        InputType::Dec => {
            let string = input.to_string();
            let int = string.parse::<u128>().unwrap();
            hex = format!("{:X}", int);
        },
        InputType::Ascii => (),
        InputType::Binary => (),
    };

    hex
}

fn hex_to_dec(input: &String) -> String {input.to_owned()}
fn hex_to_asc(input: &String) -> String {input.to_owned()}
fn hex_to_bin(input: &String) -> String {input.to_owned()}

use crate::bg::{CanFrameRaw, MsgOut, RWUtils};
use eframe::egui::{self, CentralPanel, ComboBox, SidePanel, TopBottomPanel};

#[derive(Default)]
pub struct EveTele {
    pub ports: Vec<String>,
    pub selected: usize,
    pub baud: u32,
    pub dbc_path: String,
    pub csv_path: String,
    pub out_path: String,
}

impl eframe::App for EveTele {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |_ui| {
            // The header bar across the top of the window
            TopBottomPanel::top("Header").show(ctx, |ui| {
                // Makes things horizontal
                ui.horizontal(|ui| {
                    // A quick if statement that disables UART things if there is no UART devices
                    if self.ports.len() != 0 {
                        // The drop down box for port selection
                        ComboBox::from_label("Port Select").show_index(
                            ui,                        // our UI context to draw in
                            &mut self.selected,        // Current value
                            self.ports.len(),          // Length of list of all values
                            |i| self.ports[i].clone(), // Selected value from the list
                        );

                        // Button to test serial port
                        if ui.button("Connect").clicked() {
                            RWUtils::start_bg_read(
                                &self.dbc_path,
                                &self.ports[self.selected],
                                self.baud,
                            )
                        }
                    }
                });
            });

            SidePanel::left("Tools").show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Button to load DBC
                    if ui.button("Open DBC").clicked() {
                        self.dbc_path = rfd::FileDialog::new()
                            .pick_file()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();

                        println!("DBC Path: {}", self.dbc_path.clone());
                    }

                    // Button to load CSV
                    if ui.button("Open CSV").clicked() {
                        self.csv_path = rfd::FileDialog::new()
                            .pick_file()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();

                        println!("CSV Path: {}", self.csv_path.clone());
                    }

                    // Button to test DBC parser
                    if ui.button("Foxglove ws test").clicked() {
                        MsgOut::open_fg_ws();
                    }

                    // Button to parse a log file
                    if ui.button("Log parse test").clicked() {
                        RWUtils::parse_log(
                            &self.dbc_path,
                            &self.csv_path,
                            &"./test.csv".to_string(),
                        );
                    }

                    if ui.button("Proto build test").clicked() {
                        RWUtils::my_pb_test();
                    }

                    if ui.button("MCAP save test").clicked() {
                        RWUtils::my_mcap_test();
                    }

                    if ui.button("Parsing testing").clicked() {
                        CanFrameRaw::parse_test(&self.dbc_path);
                    }
                });
            });
        });
    }
}

impl EveTele {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        cc.storage;

        // Return base app state
        EveTele {
            ports: RWUtils::list_ports(),
            baud: 115200,
            ..Default::default()
        }
    }
}

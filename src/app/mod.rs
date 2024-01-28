use crate::bg::{CanFrameRaw, MsgOut, ReadPort};
use eframe::egui::{self, CentralPanel, ComboBox, SidePanel, TopBottomPanel};

pub struct EveTele {
    pub ports: Vec<String>,
    pub selected: usize,
    pub toggel_test: bool,
    pub dbc_path: String,
    pub csv_path: String,
}

impl eframe::App for EveTele {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
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
                            ReadPort::start_bg_read(self.ports[self.selected].clone(), 115200);
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

                        println!("CSV Path: {}", self.dbc_path.clone());
                    }

                    // Button to test DBC parser
                    if ui.button("Foxglove ws test").clicked() {
                        MsgOut::open_fg_ws();
                    }

                    // Button to parse a log file
                    if ui.button("Log parse test").clicked() {
                        CanFrameRaw::parse_log(self.csv_path.clone(), self.dbc_path.clone())
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
            ports: ReadPort::list_ports(),
            selected: 0,
            toggel_test: false,
            dbc_path: String::default(),
            csv_path: String::default(),
        }
    }
}

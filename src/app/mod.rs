use crate::bg::ReadPort;
use eframe::egui::{self, CentralPanel, ComboBox, SidePanel, TopBottomPanel, Window};

pub struct EveTele {
    pub ports: Vec<String>,
    pub selected: usize,
    pub toggel_test: bool,
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
                            ReadPort::start_bg_read(self.ports[self.selected].clone(), 9600);
                        }
                    }

                    // Button to test DBC parser
                    if ui.button("Load DBC test").clicked() {
                        // ReadPort::load_dbc("./ksu.dbc".to_string());
                        // ReadPort::load_dbc();
                    }
                });
            });

            SidePanel::left("Widgets").show(ctx, |ui| {
                ui.heading("Select Widgets"); // Helper text
                ui.separator();
                ui.checkbox(&mut self.toggel_test, "Enable test widget");
            });

            Window::new("Shock")
                .open(&mut self.toggel_test)
                .title_bar(false)
                .show(ctx, |ui| {});

            Window::new("Temp Δ")
                .open(&mut self.toggel_test)
                .title_bar(false)
                .movable(false)
                .show(ctx, |ui| ui.label(499.to_string()));
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
        }
    }
}

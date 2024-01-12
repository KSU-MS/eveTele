mod app;
mod bg;

fn main() {
    // Window setting things don't worry about it too much
    let native_options = eframe::NativeOptions::default();

    // Run the app
    eframe::run_native(
        "eveTele",
        native_options,
        Box::new(|cc| Box::new(app::EveTele::new(cc))),
    )
    .unwrap();
}

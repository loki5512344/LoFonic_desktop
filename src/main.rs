mod desktop;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        desktop::assets::APP_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(desktop::app::LoFonicDesktopApp::default()))),
    )
}

mod gui;
mod lexer;
mod utils;

use gui::ventana::AppGui;
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();

    eframe::run_native(
        "Analizador Léxico - Rust",
        options,
        Box::new(|cc| {
            Ok(Box::new(AppGui::new(cc)) as Box<dyn eframe::App>)
        }),
    )
}

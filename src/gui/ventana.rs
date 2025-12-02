use eframe::egui::{CentralPanel, Context};
use eframe::egui;

use super::controlador::ControladorGUI;

pub struct AppGui {
    controlador: ControladorGUI,
}

impl AppGui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            controlador: ControladorGUI::new(),
        }
    }
}

impl eframe::App for AppGui {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Analizador Léxico — Rust");

            ui.horizontal(|ui| {
                if ui.button("Cargar diccionario").clicked() {
                    self.controlador.cargar_diccionario();
                }

                if ui.button("Cargar texto").clicked() {
                    self.controlador.cargar_texto();
                }

                if ui.button("Analizar").clicked() {
                    self.controlador.analizar();
                }
            });

            ui.separator();

            ui.collapsing("Archivos cargados", |ui| {
                ui.label(format!(
                    "Diccionario: {}",
                    self.controlador
                        .ruta_diccionario
                        .as_ref()
                        .map(|p| p.to_string_lossy())
                        .unwrap_or_else(|| "(no cargado)".into())
                ));

                ui.label(format!(
                    "Texto: {}",
                    self.controlador
                        .ruta_texto
                        .as_ref()
                        .map(|p| p.to_string_lossy())
                        .unwrap_or_else(|| "(no cargado)".into())
                ));
            });

            ui.separator();

            ui.collapsing("Resultados del análisis léxico", |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.monospace(&self.controlador.texto_mostrar);
                });
            });

            ui.separator();

            ui.collapsing("Recorrido DFA del diccionario", |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.monospace(&self.controlador.recorrido_diccionario);
                });
            });

            ui.separator();

            ui.label(&self.controlador.stats);
        });
    }
}

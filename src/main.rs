//! bellforge - Arch Linux Kettlebell Training Timer & Logger
//!
//! PR 1 Foundation: Basic egui/eframe window with dark theme.
//! Follows the arch-linux-gui-app-builder persona.

use eframe::egui;

mod models;

fn main() -> eframe::Result<()> {
    // Set up logging for development (optional but nice on Arch)
    env_logger::init(); // Will be a no-op if env_logger not in Cargo.toml yet

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([640.0, 480.0])
            .with_title("bellforge")
            .with_icon(load_icon()),
        centered: true, // eframe handles centering nicely
        ..Default::default()
    };

    eframe::run_native(
        "bellforge",
        options,
        Box::new(|cc| {
            // Set a nice dark theme suitable for training (high contrast)
            let mut visuals = egui::Visuals::dark();
            visuals.override_text_color = Some(egui::Color32::from_rgb(230, 230, 230));
            cc.egui_ctx.set_visuals(visuals);

            Ok(Box::new(BellforgeApp::new()))
        }),
    )
}

/// Placeholder icon loader (we'll embed a real SVG later in PR 8)
fn load_icon() -> egui::IconData {
    // For PR 1 we use a simple default. Real icon comes in packaging PR.
    egui::IconData {
        rgba: vec![0; 4 * 32 * 32], // transparent 32x32
        width: 32,
        height: 32,
    }
}

/// Main application state (PR 1 is deliberately minimal)
struct BellforgeApp {
    // Will grow with: current_view, templates, config, etc.
}

impl BellforgeApp {
    fn new() -> Self {
        Self {}
    }
}

impl eframe::App for BellforgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);

                // Big, bold title - gym friendly
                ui.label(
                    egui::RichText::new("bellforge")
                        .size(72.0)
                        .strong()
                        .color(egui::Color32::from_rgb(255, 200, 80)),
                );

                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Kettlebell Training Timer for Arch Linux")
                        .size(20.0)
                        .italics(),
                );

                ui.add_space(40.0);

                // Placeholder buttons (real functionality in later PRs)
                if ui
                    .add_sized([280.0, 48.0], egui::Button::new("Start Workout"))
                    .clicked()
                {
                    // TODO(PR 3+): open session runner
                }

                ui.add_space(12.0);

                if ui
                    .add_sized([280.0, 40.0], egui::Button::new("Edit Templates"))
                    .clicked()
                {
                    // TODO(PR 2): open template editor
                }

                if ui
                    .add_sized([280.0, 40.0], egui::Button::new("History & Logs"))
                    .clicked()
                {
                    // TODO(PR 6): open history view
                }

                ui.add_space(60.0);

                ui.label(
                    egui::RichText::new("PR 1 Foundation — bellforge is booting correctly on Arch")
                        .size(14.0)
                        .weak(),
                );
                ui.label(
                    egui::RichText::new("See DESIGN_PLAN.md for the full roadmap")
                        .size(13.0)
                        .weak(),
                );
            });
        });

        // Bottom status bar
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("v0.1.0 • PR 1");
                ui.separator();
                ui.label("egui 0.31 • Wayland ready");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Ready for training");
                });
            });
        });
    }
}

//! bellforge - Arch Linux Kettlebell Training Timer & Logger
//!
//! PR 1 Foundation: Basic egui/eframe window with dark theme.
//! Follows the arch-linux-gui-app-builder persona.

use eframe::egui;
use std::path::PathBuf;

mod models;
mod persistence;
mod session;

use models::{FlowItem, WorkoutTemplate};
use session::{build_session_review_markdown, SessionCue, SessionRunner};

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

/// Which screen the user is currently looking at.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum View {
    #[default]
    Dashboard,
    TemplateEditor,
    ActiveSession,
}

/// Wrapper around the core engine for the UI layer.
struct ActiveSession {
    /// Full original template (provides name + rich metadata: tags, description for focus, etc.)
    /// Stored so the post-session review export can emit the full Obsidian YAML frontmatter
    /// matching the reference format (without needing to persist extra state).
    template: WorkoutTemplate,
    runner: SessionRunner,

    /// If true, automatically advance to the next step when a rest finishes.
    /// If false, wait for the user to manually press "Start Next".
    auto_advance_after_rest: bool,

    /// True if the review screen was reached via "End Session" (early abort / partial workout).
    /// Used to show a qualifier note in the review header and to set "partial" status in export.
    ended_early: bool,
}

/// Main application state for bellforge (PR 2 - Template Editor)
struct BellforgeApp {
    current_view: View,

    /// Built-in example templates (always available, not saved to disk)
    builtins: Vec<WorkoutTemplate>,

    /// User-created / edited templates (persisted to disk)
    user_templates: Vec<WorkoutTemplate>,

    /// The template currently being edited
    editing: Option<WorkoutTemplate>,

    /// Currently running workout session (PR 3)
    active_session: Option<ActiveSession>,

    // --- Add Exercise Popup state (PR 2) ---
    add_exercise_popup_open: bool,
    pending_exercise_name: String,
    pending_exercise_reps: u32,
    pending_exercise_sets: u32,
    pending_exercise_weight: Option<f32>,
    // If Some(index), we are adding the exercise inside a specific Repeat block
    pending_add_to_repeat_idx: Option<usize>,

    // For inner mutations inside Repeat blocks
    pending_inner_delete: Option<(usize, usize)>,   // (repeat_index, inner_index)
    pending_inner_move_up: Option<(usize, usize)>,
    pending_inner_move_down: Option<(usize, usize)>,

    // Export feedback
    export_status: Option<String>,
}

impl BellforgeApp {
    fn new() -> Self {
        let mut app = Self {
            current_view: View::Dashboard,
            builtins: Vec::new(),
            user_templates: Vec::new(),
            editing: None,
            add_exercise_popup_open: false,
            pending_exercise_name: "New Exercise".to_string(),
            pending_exercise_reps: 5,
            pending_exercise_sets: 1,
            pending_exercise_weight: None,
            pending_add_to_repeat_idx: None,
            pending_inner_delete: None,
            pending_inner_move_up: None,
            pending_inner_move_down: None,
            export_status: None,
            active_session: None,
        };

        app.load_builtin_templates();
        app.load_user_templates();
        app
    }

    fn load_builtin_templates(&mut self) {
        let mut ss = WorkoutTemplate::new("Simple & Sinister");
        ss.description = Some("100 one-arm swings + 10 get-ups".into());
        ss.tags = vec!["kettlebell".into(), "simpleandsinister".into()];
        ss.default_weight_kg = Some(32.0);
        ss.rest_between_exercises_s = 30;
        ss.rest_between_rounds_s = 60;
        ss.flow = vec![
            FlowItem::exercise("One-Arm Swing (L)", 10, 5),
            FlowItem::exercise("One-Arm Swing (R)", 10, 5),
            FlowItem::rest(60, "Round Rest"),
            FlowItem::exercise("Turkish Get-Up (L)", 1, 5),
            FlowItem::exercise("Turkish Get-Up (R)", 1, 5),
        ];
        self.builtins.push(ss);

        let mut complex = WorkoutTemplate::new("Kettlebell Complex");
        complex.description = Some("Swing → Clean → Press → Squat".into());
        complex.rest_between_exercises_s = 45;
        complex.rest_between_rounds_s = 90;
        complex.flow = vec![
            FlowItem::exercise("Two-Hand Swing", 8, 1),
            FlowItem::exercise("Double Clean", 6, 1),
            FlowItem::exercise("Double Press", 5, 1),
            FlowItem::exercise("Double Front Squat", 5, 1),
            FlowItem::rest(90, "Round Rest"),
        ];
        self.builtins.push(complex);

        // Rite of Passage inspired (Clean + Press + Squat)
        let mut rop = WorkoutTemplate::new("Rite of Passage");
        rop.description = Some("Clean + Press + Squat".into());
        rop.rest_between_exercises_s = 60;
        rop.rest_between_rounds_s = 120;
        rop.flow = vec![
            FlowItem::exercise("Double Clean", 1, 1),
            FlowItem::exercise("Double Press", 1, 1),
            FlowItem::exercise("Double Front Squat", 1, 1),
            FlowItem::rest(120, "Round Rest"),
        ];
        self.builtins.push(rop);

        // Double Bell Strength
        let mut strength = WorkoutTemplate::new("Double Bell Strength");
        strength.description = Some("Press + Row + Swing".into());
        strength.rest_between_exercises_s = 50;
        strength.rest_between_rounds_s = 100;
        strength.flow = vec![
            FlowItem::exercise("Double Press", 5, 1),
            FlowItem::exercise("Double Bent Over Row", 6, 1),
            FlowItem::exercise("Two-Hand Swing", 10, 1),
            FlowItem::rest(100, "Round Rest"),
        ];
        self.builtins.push(strength);

        // Swing + Snatch Complex
        let mut snatch = WorkoutTemplate::new("Swing & Snatch");
        snatch.description = Some("Swing + Snatch focus".into());
        snatch.rest_between_exercises_s = 40;
        snatch.rest_between_rounds_s = 90;
        snatch.flow = vec![
            FlowItem::exercise("Two-Hand Swing", 10, 1),
            FlowItem::exercise("One-Arm Snatch (L)", 5, 1),
            FlowItem::exercise("One-Arm Snatch (R)", 5, 1),
            FlowItem::rest(90, "Round Rest"),
        ];
        self.builtins.push(snatch);

        // No empty template — users should start with real programs
    }

    fn load_user_templates(&mut self) {
        self.user_templates = persistence::load_user_templates();
    }

    /// Starts a new workout session from a template
    fn start_session(&mut self, template: WorkoutTemplate) {
        let mut runner = SessionRunner::from_template(&template);

        // Skip any leading Rest cues so the user starts on an actual exercise
        while let Some(SessionCue::Rest { .. }) = runner.current_cue() {
            if !runner.advance() {
                break;
            }
        }

        self.active_session = Some(ActiveSession {
            template,
            runner,
            auto_advance_after_rest: true,
            ended_early: false,
        });

        self.export_status = None; // prevent template-export messages from leaking into the new review screen
        self.current_view = View::ActiveSession;
    }

    /// Associated function (no &self needed — pure dialog + write). Called as
    /// BellforgeApp::export_template... or Self::... from within the impl.
    fn export_template_as_markdown_with_dialog(template: &WorkoutTemplate) -> std::io::Result<PathBuf> {
        // Generate the Markdown content first
        let mut md = String::new();

        md.push_str(&format!("# {}\n\n", template.name));

        if let Some(desc) = &template.description {
            md.push_str(&format!("{}\n\n", desc));
        }

        md.push_str(&format!("**Estimated Duration**: ~{} minutes\n", template.estimated_duration_minutes()));
        md.push_str(&format!("**Rest between exercises**: {}s\n", template.rest_between_exercises_s));
        md.push_str(&format!("**Rest between rounds**: {}s\n\n", template.rest_between_rounds_s));

        md.push_str("## Workout Flow\n\n");

        for (i, item) in template.flow.iter().enumerate() {
            match item {
                FlowItem::Exercise { name, reps, sets, weight_kg, .. } => {
                    let weight = weight_kg.map_or("bodyweight".to_string(), |w| format!("{}kg", w));
                    md.push_str(&format!("{}. **{}** — {} × {} reps @ {}\n", i + 1, name, sets, reps, weight));
                }
                FlowItem::Rest { label, duration_s } => {
                    md.push_str(&format!("{}. *{}* — {} seconds\n", i + 1, label, duration_s));
                }
                FlowItem::Repeat { count, items } => {
                    md.push_str(&format!("{}. **Repeat ×{}**\n", i + 1, count));
                    for inner in items {
                        if let FlowItem::Exercise { name, reps, weight_kg, .. } = inner {
                            let weight = weight_kg.map_or("bodyweight".to_string(), |w| format!("{}kg", w));
                            md.push_str(&format!("    - {} × {} @ {}\n", name, reps, weight));
                        }
                    }
                }
            }
        }

        md.push_str("\n---\n*Exported from bellforge*");

        // Ask the user where to save using a native file dialog
        let default_filename = format!("{}.md", template.name.to_lowercase().replace(' ', "-"));

        let file_path = rfd::FileDialog::new()
            .set_title("Export Template as Markdown")
            .set_file_name(&default_filename)
            .add_filter("Markdown", &["md"])
            .save_file();

        let path = match file_path {
            Some(p) => p,
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File dialog was closed or failed to open."
            )),
        };

        std::fs::write(&path, md)?;
        Ok(path)
    }

    /// Generates and exports a post-workout review log (with user-edited actual reps)
    /// as a rich Obsidian-compatible Markdown file (YAML frontmatter + table + Notes)
    /// exactly matching the reference format in "2026-05-16 - Kettlebell .md".
    /// Uses the stored template for metadata and the pure helper for the body.
    /// Associated function (no &self) per reviewer nit.
    fn export_session_review_as_markdown_with_dialog(session: &ActiveSession) -> std::io::Result<PathBuf> {
        let now = ::chrono::Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H:%M").to_string();
        let datetime = now.format("%Y-%m-%dT%H:%M").to_string();
        let body_date = now.format("%Y-%m-%d %H:%M").to_string();
        // Use the pure extracted helper (testable, no side effects, no rfd/fs/chrono inside).
        let md = build_session_review_markdown(
            &session.runner,
            &session.template,
            &date,
            &time,
            &datetime,
            &body_date,
            session.ended_early,
        );

        // Native save dialog (rfd respects the DE — KDE, GNOME, etc.)
        let safe_name = session.template.name.to_lowercase().chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
            .collect::<String>();
        let default_filename = format!("workout-log-{}-{}.md", now.format("%Y%m%d-%H%M"), safe_name);

        let file_path = rfd::FileDialog::new()
            .set_title("Export Session Review as Markdown")
            .set_file_name(&default_filename)
            .add_filter("Markdown", &["md"])
            .save_file();

        let path = match file_path {
            Some(p) => p,
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File dialog was cancelled."
            )),
        };

        std::fs::write(&path, md)?;
        Ok(path)
    }

    fn edit_template(&mut self, template: WorkoutTemplate) {
        self.editing = Some(template);
        self.export_status = None; // prevent session-review messages from leaking into the template editor
        self.current_view = View::TemplateEditor;
    }

    fn exit_editor(&mut self, save: bool) {
        if save {
            if let Some(template) = self.editing.take() {
                // Persist to disk
                if let Err(e) = persistence::save_template(&template) {
                    eprintln!("Failed to save template: {e}");
                }

                // Add or replace in user_templates
                if let Some(pos) = self.user_templates.iter().position(|t| t.id == template.id) {
                    self.user_templates[pos] = template;
                } else {
                    self.user_templates.push(template);
                }
            }
        } else {
            self.editing = None;
        }
        self.current_view = View::Dashboard;
    }
}

impl eframe::App for BellforgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // === Session Timer Logic (runs every frame, outside any UI closure) ===
        if self.current_view == View::ActiveSession {
            if let Some(session) = &mut self.active_session {
                if session.runner.is_resting && !session.runner.paused {
                    // Check if rest naturally finished
                    if session.runner.check_rest_finished() {
                        session::play_rest_end_chime();

                        if session.auto_advance_after_rest {
                            // Advance after natural rest end. If this was the terminal rest, advance()
                            // on the final cue sets the finished flag (review appears next frame).
                            // Return value ignored only for the documented final-cue case.
                            let _ = session.runner.advance();
                        }
                    }
                }
            }
        }

        match self.current_view {
            View::Dashboard => self.show_dashboard(ctx),
            View::TemplateEditor => self.show_template_editor(ctx),
            View::ActiveSession => self.show_active_session(ctx),
        }

        // Global bottom status bar
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("v0.1.0 • PR 3 — {}", 
                    match self.current_view {
                        View::Dashboard => "Dashboard",
                        View::TemplateEditor => "Template Editor",
                        View::ActiveSession => "Active Session",
                    }
                ));
                ui.separator();
                ui.label("egui 0.31 • Wayland ready");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("bellforge");
                });
            });
        });
    }
}

/// Generates a smart, contextual button label
fn get_next_action_label(session: &ActiveSession) -> String {
    if let Some(cue) = session.runner.current_cue() {
        match cue {
            SessionCue::Perform { name, .. } => {
                // Look ahead to see if the next cue is the same exercise
                if let Some(next_cue) = session.runner.cues.get(session.runner.current_index + 1) {
                    if let SessionCue::Perform { name: next_name, .. } = next_cue {
                        if next_name == name {
                            return "Start Next Rep".to_string();
                        }
                    }
                }
                "Start Next Exercise".to_string()
            }
            SessionCue::Rest { .. } => "Start Next".to_string(),
        }
    } else {
        "Continue".to_string()
    }
}

impl BellforgeApp {
    fn show_dashboard(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);

                ui.label(
                    egui::RichText::new("bellforge")
                        .size(64.0)
                        .strong()
                        .color(egui::Color32::from_rgb(255, 200, 80)),
                );
                ui.label(
                    egui::RichText::new("Kettlebell Training Timer for Arch Linux")
                        .size(18.0)
                        .italics(),
                );

                ui.add_space(30.0);

                ui.heading("Built-in Templates");

                ui.add_space(8.0);

                let mut builtin_edit: Option<usize> = None;
                let mut builtin_start: Option<usize> = None;

                for (idx, template) in self.builtins.iter().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(&template.name).size(17.0).strong());
                                ui.label(egui::RichText::new(template.summary()).size(12.0).weak());
                            });
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Start").clicked() {
                                    builtin_start = Some(idx);
                                }
                                if ui.button("Edit").clicked() {
                                    builtin_edit = Some(idx);
                                }
                            });
                        });
                    });
                    ui.add_space(4.0);
                }

                if let Some(idx) = builtin_start {
                    self.start_session(self.builtins[idx].clone());
                }

                if let Some(idx) = builtin_edit {
                    let mut copy = self.builtins[idx].clone();
                    copy.id = uuid::Uuid::new_v4();
                    copy.name = format!("{} (Copy)", copy.name);
                    self.edit_template(copy);
                }

                ui.add_space(18.0);
                ui.heading("Your Templates");

                ui.add_space(8.0);

                if ui
                    .add_sized([240.0, 38.0], egui::Button::new("＋ New Template"))
                    .clicked()
                {
                    self.edit_template(WorkoutTemplate::new("Untitled Workout"));
                }

                ui.add_space(10.0);

                if self.user_templates.is_empty() {
                    ui.label(
                        egui::RichText::new("No saved templates yet. Create one above!")
                            .size(13.0)
                            .weak(),
                    );
                }

                let mut edit_user: Option<usize> = None;
                let mut delete_user: Option<usize> = None;
                let mut start_user: Option<usize> = None;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (idx, template) in self.user_templates.iter().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&template.name).size(17.0).strong());
                                    ui.label(egui::RichText::new(template.summary()).size(12.0).weak());
                                });
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.small_button("🗑").clicked() {
                                        delete_user = Some(idx);
                                    }
                                    if ui.button("Start").clicked() {
                                        start_user = Some(idx);
                                    }
                                    if ui.button("Edit").clicked() {
                                        edit_user = Some(idx);
                                    }
                                });
                            });
                        });
                        ui.add_space(4.0);
                    }
                });

                if let Some(idx) = start_user {
                    self.start_session(self.user_templates[idx].clone());
                }

                if let Some(idx) = edit_user {
                    let template = self.user_templates[idx].clone();
                    self.edit_template(template);
                }

                if let Some(idx) = delete_user {
                    let template = &self.user_templates[idx];
                    if let Err(e) = persistence::delete_template(template) {
                        eprintln!("Failed to delete template file: {e}");
                    }
                    self.user_templates.remove(idx);
                }
            });
        });
    }

    fn show_template_editor(&mut self, ctx: &egui::Context) {
        // Take ownership of the template we're editing to avoid long borrows
        let mut template = match self.editing.take() {
            Some(t) => t,
            None => {
                self.current_view = View::Dashboard;
                return;
            }
        };

        let mut request_save = false;
        let mut request_discard = false;
        let mut request_export = false;

        egui::CentralPanel::default().show(ctx, |ui| {
            // Top bar
            ui.horizontal(|ui| {
                if ui.button("← Back to Dashboard").clicked() {
                    request_discard = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Export as Markdown").clicked() {
                        request_export = true;
                    }
                    if ui.button("Save & Close").clicked() {
                        request_save = true;
                    }
                    if ui.button("Discard Changes").clicked() {
                        request_discard = true;
                    }
                });
            });

            ui.separator();

            // Export status feedback
            if let Some(msg) = &self.export_status {
                ui.label(egui::RichText::new(msg).color(egui::Color32::from_rgb(100, 220, 140)));
            }

            // Template header
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut template.name);
            });

            ui.horizontal(|ui| {
                ui.label("Description:");
                let mut desc = template.description.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut desc).changed() {
                    template.description = if desc.trim().is_empty() { None } else { Some(desc) };
                }
            });

            ui.add_space(8.0);

            // Global rest settings
            ui.collapsing("Global Rest Settings", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Rest between exercises:");
                    ui.add(egui::DragValue::new(&mut template.rest_between_exercises_s).suffix(" sec"));
                    ui.label("Rest between rounds:");
                    ui.add(egui::DragValue::new(&mut template.rest_between_rounds_s).suffix(" sec"));
                });
            });

            ui.add_space(12.0);

            // Estimated time
            ui.label(
                egui::RichText::new(format!(
                    "Estimated session time: ~{} minutes",
                    template.estimated_duration_minutes()
                ))
                .size(16.0)
                .color(egui::Color32::from_rgb(100, 200, 255)),
            );

            ui.add_space(12.0);
            ui.label(egui::RichText::new("Workout Flow").size(20.0).strong().color(egui::Color32::from_rgb(255, 200, 100)));

            // Flow items list — editable version (index-based to avoid borrow issues)
            let mut to_delete: Option<usize> = None;
            let mut to_move_up: Option<usize> = None;
            let mut to_move_down: Option<usize> = None;

            {
                let flow_len = template.flow.len();

                egui::ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
                    if flow_len == 0 {
                        ui.label("No steps yet. Use the buttons below to add exercises or rests.");
                        ui.add_space(8.0);
                    }

                    for i in 0..flow_len {
                        let item = &mut template.flow[i];

                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{}.", i + 1)).strong());

                                match item {
                                    FlowItem::Exercise {
                                        name,
                                        reps,
                                        sets,
                                        weight_kg,
                                        ..
                                    } => {
                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new("Exercise:").color(egui::Color32::from_rgb(140, 220, 140)));
                                                ui.text_edit_singleline(name);
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Reps:");
                                                ui.add(egui::DragValue::new(reps).range(1..=50));
                                                ui.label("Sets:");
                                                ui.add(egui::DragValue::new(sets).range(1..=20));
                                                ui.label("Weight (kg):");
                                                let mut w = weight_kg.unwrap_or(0.0);
                                                if ui.add(egui::DragValue::new(&mut w).range(0.0..=100.0).speed(0.5)).changed() {
                                                    *weight_kg = if w > 0.0 { Some(w) } else { None };
                                                }
                                            });
                                        });
                                    }

                                    FlowItem::Rest { duration_s, label } => {
                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new("Rest:").color(egui::Color32::from_rgb(120, 180, 240)));
                                                ui.text_edit_singleline(label);
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Duration:");
                                                ui.add(egui::DragValue::new(duration_s).range(5..=300).suffix(" sec"));
                                            });
                                        });
                                    }

                                    FlowItem::Repeat { count, items } => {
                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    egui::RichText::new(format!("🔁 Repeat ×{}", count))
                                                        .strong()
                                                        .color(egui::Color32::from_rgb(255, 180, 100)),
                                                );
                                                ui.add(egui::DragValue::new(count).range(1..=20));
                                            });

                                            ui.add_space(4.0);

                                            // Render inner items with editing capability
                                            let inner_len = items.len();

                                            if inner_len == 0 {
                                                ui.label(egui::RichText::new("    (no steps inside yet)").weak());
                                            } else {
                                                for inner_i in 0..inner_len {
                                                    let inner_item = &mut items[inner_i];

                                                    ui.horizontal(|ui| {
                                                        ui.add_space(18.0);

                                                        match inner_item {
                                                            FlowItem::Exercise { name, reps, sets, weight_kg, .. } => {
                                                                ui.label(egui::RichText::new("Exercise:").color(egui::Color32::from_rgb(140, 220, 140)));
                                                                ui.text_edit_singleline(name);
                                                                ui.label("Reps:");
                                                                ui.add(egui::DragValue::new(reps).range(1..=50));
                                                                ui.label("Sets:");
                                                                ui.add(egui::DragValue::new(sets).range(1..=20));
                                                                ui.label("kg:");
                                                                let mut w = weight_kg.unwrap_or(0.0);
                                                                if ui.add(egui::DragValue::new(&mut w).range(0.0..=100.0).speed(0.5)).changed() {
                                                                    *weight_kg = if w > 0.0 { Some(w) } else { None };
                                                                }
                                                            }
                                                            FlowItem::Rest { label, duration_s } => {
                                                                ui.label(egui::RichText::new("Rest:").color(egui::Color32::from_rgb(120, 180, 240)));
                                                                ui.text_edit_singleline(label);
                                                                ui.add(egui::DragValue::new(duration_s).range(5..=300).suffix("s"));
                                                            }
                                                            _ => {
                                                                ui.label(inner_item.display_name());
                                                            }
                                                        }

                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                            if ui.small_button("✕").clicked() {
                                                                self.pending_inner_delete = Some((i, inner_i));
                                                            }
                                                            if ui.small_button("↓").clicked() && inner_i + 1 < inner_len {
                                                                self.pending_inner_move_down = Some((i, inner_i));
                                                            }
                                                            if ui.small_button("↑").clicked() && inner_i > 0 {
                                                                self.pending_inner_move_up = Some((i, inner_i));
                                                            }
                                                        });
                                                    });
                                                }
                                            }

                                            ui.add_space(6.0);
                                            if ui.small_button("＋ Add Exercise inside this block").clicked() {
                                                self.pending_exercise_name = "New Exercise".to_string();
                                                self.pending_exercise_reps = 5;
                                                self.pending_exercise_sets = 1;
                                                self.pending_exercise_weight = Some(24.0);
                                                self.pending_add_to_repeat_idx = Some(i);
                                                self.add_exercise_popup_open = true;
                                            }
                                        });
                                    }
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.small_button("✕").clicked() {
                                        to_delete = Some(i);
                                    }
                                    if ui.small_button("↓").clicked() && i + 1 < flow_len {
                                        to_move_down = Some(i);
                                    }
                                    if ui.small_button("↑").clicked() && i > 0 {
                                        to_move_up = Some(i);
                                    }
                                });
                            });
                        });
                        ui.add_space(4.0);
                    }
                });
            }

            // Apply structural changes after the UI closure
            if let Some(i) = to_delete {
                template.flow.remove(i);
            }
            if let Some(i) = to_move_up {
                template.flow.swap(i, i - 1);
            }
            if let Some(i) = to_move_down {
                template.flow.swap(i, i + 1);
            }

            // Apply inner mutations for Repeat blocks
            if let Some((repeat_idx, inner_idx)) = self.pending_inner_delete.take() {
                if let FlowItem::Repeat { items, .. } = &mut template.flow[repeat_idx] {
                    if inner_idx < items.len() {
                        items.remove(inner_idx);
                    }
                }
            }
            if let Some((repeat_idx, inner_idx)) = self.pending_inner_move_up.take() {
                if let FlowItem::Repeat { items, .. } = &mut template.flow[repeat_idx] {
                    if inner_idx > 0 {
                        items.swap(inner_idx, inner_idx - 1);
                    }
                }
            }
            if let Some((repeat_idx, inner_idx)) = self.pending_inner_move_down.take() {
                if let FlowItem::Repeat { items, .. } = &mut template.flow[repeat_idx] {
                    if inner_idx + 1 < items.len() {
                        items.swap(inner_idx, inner_idx + 1);
                    }
                }
            }

            ui.add_space(16.0);

            // Add new items
            ui.horizontal(|ui| {
                if ui.button("＋ Add Exercise").clicked() {
                    self.pending_exercise_name = "New Exercise".to_string();
                    self.pending_exercise_reps = 5;
                    self.pending_exercise_sets = 1;
                    self.pending_exercise_weight = Some(24.0);
                    self.pending_add_to_repeat_idx = None;
                    self.add_exercise_popup_open = true;
                }
                if ui.button("＋ Add Rest").clicked() {
                    template.flow.push(FlowItem::rest(45, "Rest"));
                }
                if ui.button("＋ Add Repeat Block").clicked() {
                    template.flow.push(FlowItem::Repeat {
                        count: 3,
                        items: vec![],
                    });
                }
            });

            ui.add_space(20.0);
            ui.label(
                egui::RichText::new("PR 2 — Template Editor (MVP)")
                    .size(13.0)
                    .weak(),
            );
        });

        // Handle exit after the UI pass
        if request_save {
            self.editing = Some(template);
            self.exit_editor(true);
        } else if request_discard {
            self.exit_editor(false);
        } else if request_export {
            // Export while we still own the template
            match Self::export_template_as_markdown_with_dialog(&template) {
                Ok(path) => {
                    self.export_status = Some(format!("Exported to: {}", path.display()));
                }
                Err(e) => {
                    self.export_status = Some(format!("Export failed: {}", e));
                }
            }
            // Put the template back
            self.editing = Some(template);
        } else {
            // Put the (possibly modified) template back so we don't lose state
            self.editing = Some(template);
        }

        // === Add Exercise Popup ===
        if self.add_exercise_popup_open {
            let mut should_add = false;
            let mut should_cancel = false;

            egui::Window::new("Add Exercise")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("Exercise Name:");
                        ui.text_edit_singleline(&mut self.pending_exercise_name);

                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            ui.label("Reps:");
                            ui.add(egui::DragValue::new(&mut self.pending_exercise_reps).range(1..=50));
                            ui.label("Sets:");
                            ui.add(egui::DragValue::new(&mut self.pending_exercise_sets).range(1..=20));
                        });

                        ui.add_space(6.0);

                        ui.horizontal(|ui| {
                            ui.label("Weight (kg):");
                            let mut w = self.pending_exercise_weight.unwrap_or(24.0);
                            if ui.add(egui::DragValue::new(&mut w).range(0.0..=100.0).speed(0.5)).changed() {
                                self.pending_exercise_weight = if w > 0.0 { Some(w) } else { None };
                            }
                        });

                        ui.add_space(12.0);

                        ui.horizontal(|ui| {
                            if ui.button("Add Exercise").clicked() {
                                should_add = true;
                            }
                            if ui.button("Cancel").clicked() {
                                should_cancel = true;
                            }
                        });
                    });
                });

            if should_add {
                let mut new_ex = FlowItem::exercise(
                    self.pending_exercise_name.clone(),
                    self.pending_exercise_reps,
                    self.pending_exercise_sets,
                );

                if let FlowItem::Exercise { weight_kg, .. } = &mut new_ex {
                    *weight_kg = self.pending_exercise_weight;
                }

                if let Some(template) = &mut self.editing {
                    if let Some(repeat_idx) = self.pending_add_to_repeat_idx {
                        // Add inside a specific repeat block
                        if let FlowItem::Repeat { items, .. } = &mut template.flow[repeat_idx] {
                            items.push(new_ex);
                        }
                    } else {
                        // Add to top level
                        template.flow.push(new_ex);
                    }
                }

                self.add_exercise_popup_open = false;
                self.pending_add_to_repeat_idx = None;
            }

            if should_cancel {
                self.add_exercise_popup_open = false;
                self.pending_add_to_repeat_idx = None;
            }
        }
    }

    fn show_active_session(&mut self, ctx: &egui::Context) {
        // Request repaint frequently while resting for smooth live countdown (real-time calc in remaining_rest_seconds).
        // 250ms cadence is cheap and prevents perceived jumps; immediate request_repaint() in click handler
        // ensures the timer appears on the very next frame after "Start Rest".
        if let Some(session) = &self.active_session {
            if session.runner.is_resting && !session.runner.paused {
                ctx.request_repaint_after(std::time::Duration::from_millis(250));
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                let mut end_session_requested = false;
                let mut request_review_export = false;

                if let Some(session) = &mut self.active_session {
                    // Header
                    ui.label(egui::RichText::new(&session.template.name).size(28.0).strong());
                    let (current, total) = session.runner.progress();
                    ui.label(format!("Step {} / {}", current, total));

                    if session.runner.paused {
                        ui.label(egui::RichText::new("⏸ PAUSED").color(egui::Color32::YELLOW));
                    }

                    ui.add_space(12.0);

                    // When the session is finished we show the dedicated review screen instead of
                    // the normal cue + action UI (avoids any final-step overlap per past review feedback).
                    if !session.runner.is_finished() {
                        // Current cue / state display.
                        // CRITICAL: if is_resting, show REST + live countdown *regardless of current_cue type*.
                        // This ensures "Start Rest" on a Perform cue (even final Perform with no trailing Rest)
                        // immediately renders the visible timer after the repaint() kickoff.
                        // Re-uses engine's remaining_rest_seconds() + is_resting (no logic duplication).
                        if session.runner.is_resting {
                            ui.label(egui::RichText::new("REST").size(18.0).strong().color(egui::Color32::from_rgb(255, 180, 100)));
                            let rest_label = match session.runner.current_cue() {
                                Some(SessionCue::Rest { label, .. }) => label.clone(),
                                _ => "Rest".to_string(),
                            };
                            ui.label(rest_label);
                            let remaining = session.runner.remaining_rest_seconds();
                            ui.label(egui::RichText::new(format!("{}s", remaining)).size(52.0).strong());
                        } else if let Some(cue) = session.runner.current_cue() {
                            match cue {
                                SessionCue::Perform { name, target_reps, weight_kg } => {
                                    ui.label(egui::RichText::new("PERFORM").size(18.0).strong().color(egui::Color32::from_rgb(120, 220, 140)));
                                    ui.label(egui::RichText::new(name).size(26.0).strong());
                                    if let Some(w) = weight_kg {
                                        ui.label(format!("{} kg", w));
                                    }
                                    ui.label(format!("Target: {} reps", target_reps));
                                }

                                SessionCue::Rest { label, .. } => {
                                    ui.label(egui::RichText::new("REST").size(18.0).strong().color(egui::Color32::from_rgb(255, 180, 100)));
                                    ui.label(label);

                                    // !is_resting Rest cue (post-finish, auto-advance off): show 0s + complete
                                    // (the active resting timer is handled in the if above)
                                    ui.label(egui::RichText::new("0s").size(52.0).strong().color(egui::Color32::from_rgb(255, 200, 100)));
                                    ui.label(egui::RichText::new("Rest Complete").size(18.0).strong());
                                }
                            }
                        }

                        ui.add_space(20.0);

                        // === Main Action Button (Clean Flow) ===
                        // While actively resting we show *only* the live countdown (now rendered cue-independently above).
                        // Skip Rest is now available as a *secondary* (less prominent) button in the controls row.
                        // The finished flag (set only on explicit completion of the *last* cue) ensures we still
                        // render the final Perform cue (0-rest-after-last case) and give the user a "Complete Workout"
                        // button instead of jumping straight to review.
                        let is_final_perform = session.runner.is_on_final_perform();
                        let on_perform = matches!(session.runner.current_cue(), Some(SessionCue::Perform { .. })) && !session.runner.is_finished();
                        let on_rest = matches!(session.runner.current_cue(), Some(SessionCue::Rest { .. }));
                        let waiting_after_rest = on_rest && !session.runner.is_resting && !session.runner.is_finished();

                        if session.runner.is_resting {
                            // Actively resting: live countdown timer is shown above (in cue display).
                            // Primary is timer only; Skip Rest lives in secondary row below.
                        } else if waiting_after_rest {
                            // Rest finished, waiting for user (when auto-advance is off).
                            // "0s + Rest Complete" is already shown in the cue display arm; show only the action button here.
                            // For a *terminal* rest this button will advance() the last cue and set the finished flag.
                            let next_label = get_next_action_label(session);
                            if ui.add_sized([280.0, 52.0], egui::Button::new(next_label)).clicked() {
                                // May be advancing from the very last rest (terminal case). The return value is
                                // ignored because advance() on the final cue sets the finished flag internally.
                                let _ = session.runner.advance();
                            }
                        } else if on_perform {
                            // Normal Perform (or the special final Perform with 0-rest-after).
                            let label = if is_final_perform { "Complete Workout" } else { "Start Rest" };
                            if ui.add_sized([260.0, 48.0], egui::Button::new(label)).clicked() {
                                if is_final_perform {
                                    // For a 0-rest final Perform we never want a fake rest; we just mark finished
                                    // so the review screen appears on the next frame (with the last exercise's
                                    // actual_reps editable). This fixes the premature-review bug.
                                    session.runner.mark_finished();
                                    ui.ctx().request_repaint();
                                } else {
                                    // Advance into following Rest cue (or no-op for final Perform with 0-rest).
                                    // Then start timer. request_repaint() ensures *immediate* next frame shows
                                    // the REST + live countdown (solves click-frame display lag).
                                    // The return is ignored only for the (now-handled) final-Perform case; see
                                    // the is_final_perform branch above.
                                    let _ = session.runner.advance();
                                    session.runner.start_rest(60);
                                    ui.ctx().request_repaint();
                                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
                                }
                            }
                        } else if !session.runner.is_finished() {
                            // Fallback (only when not on the final step)
                            if ui.add_sized([260.0, 48.0], egui::Button::new("Continue")).clicked() {
                                // Non-final Continue; return ignored only on pathological empty templates.
                                let _ = session.runner.advance();
                            }
                        }

                        ui.add_space(12.0);

                        // Secondary controls (always visible during active workout)
                        ui.horizontal(|ui| {
                            if ui.button(if session.runner.paused { "▶ Resume" } else { "⏸ Pause" }).clicked() {
                                session.runner.toggle_pause();
                            }

                            if session.runner.is_resting {
                                if ui.button("⏭ Skip Rest").clicked() {
                                    session.runner.skip_rest();
                                    if session.auto_advance_after_rest {
                                        // May be skipping the terminal rest. advance() on final sets finished flag.
                                        // Return ignored only in that documented final-cue case.
                                        let _ = session.runner.advance();
                                    }
                                    ui.ctx().request_repaint();
                                }
                            }

                            if ui.button("End Session").clicked() {
                                // Route mid-workout (or any !finished) "End Session" through the review screen
                                // by setting the finished flag. This gives the user a chance to review/edit
                                // the actual_reps recorded so far instead of losing everything (addresses
                                // the "End Session bypasses review" suggestion). The review's own
                                // "Finish & Back" button performs the actual session clear.
                                session.ended_early = true;
                                session.runner.mark_finished();
                                // Do not set end_session_requested here — review will appear and its finish
                                // button will do the deferred clear (borrow-safe pattern).
                            }
                        });

                        // Setting
                        ui.checkbox(&mut session.auto_advance_after_rest, "Auto-advance after rest");
                    }

                    // === Post-session Review Screen ===
                    // Replaces the old simple "Session Complete!" banner. Lists every Perform step
                    // the runner tracked. User edits actual reps here (defaults = targets from template).
                    // Export uses the existing rfd + markdown style from the template editor.
                    if session.runner.is_finished() {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("📋 Session Review").size(22.0).strong().color(egui::Color32::from_rgb(120, 200, 255)));
                        if session.ended_early {
                            ui.label(egui::RichText::new("(ended early — partial results below)").color(egui::Color32::YELLOW).small());
                        }
                        ui.label("Edit the actual reps completed for each exercise below. Defaults start at the planned targets.");
                        ui.add_space(4.0);

                        // Show export feedback on the frame after the click (status set after borrow ends).
                        // Scope to session/review messages only (prevents template-export pollution from
                        // the shared export_status field; cleared on start_session/edit_template anyway).
                        if let Some(msg) = &self.export_status {
                            // Robust scoping using internal marker set by the session export path
                            // (clears on view transitions are the primary defense; this is secondary and non-fragile).
                            if msg.starts_with("__SESSION_REVIEW__") {
                                let display = msg.strip_prefix("__SESSION_REVIEW__ ").unwrap_or(msg);
                                ui.label(egui::RichText::new(display).color(egui::Color32::from_rgb(100, 220, 140)).small());
                            }
                        }

                        ui.add_space(6.0);

                        // Scrollable review list — one row per performed exercise with live-editable actual
                        egui::ScrollArea::vertical().max_height(240.0).show(ui, |ui| {
                            let mut ex_num = 1usize;
                            for i in 0..session.runner.cues.len() {
                                if let SessionCue::Perform { name, target_reps, weight_kg } = &session.runner.cues[i] {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{}.", ex_num));
                                        ui.strong(name.clone());
                                        if let Some(w) = weight_kg {
                                            ui.label(format!("@ {}kg", w));
                                        }
                                        ui.add_space(10.0);
                                        ui.label(format!("target: {}", target_reps));
                                        ui.add_space(6.0);
                                        ui.label("actual:");
                                        let actual = &mut session.runner.actual_reps[i];
                                        ui.add(egui::DragValue::new(actual).range(0..=150).speed(0.5));
                                        ui.label("reps");
                                        if *actual != *target_reps {
                                            ui.colored_label(egui::Color32::from_rgb(255, 180, 80), "(edited)");
                                        }
                                    });
                                    ex_num += 1;
                                }
                            }
                        });

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.add_sized([170.0, 32.0], egui::Button::new("📤 Export as Markdown")).clicked() {
                                request_review_export = true;
                            }
                            // Always "Finish & Back to Dashboard" in review (whether natural completion or
                            // ended_early via End Session). The "Complete Workout" label is only for the
                            // live last-Perform action button (is_on_final_perform case).
                            if ui.add_sized([210.0, 32.0], egui::Button::new("✓ Finish & Back to Dashboard")).clicked() {
                                end_session_requested = true;
                            }
                        });
                    }
                } else {
                    ui.label("No active session.");
                    if ui.button("Back to Dashboard").clicked() {
                        self.current_view = View::Dashboard;
                    }
                }

                if end_session_requested {
                    self.active_session = None;
                    self.current_view = View::Dashboard;
                }

                if request_review_export {
                    if let Some(sess) = &self.active_session {
                        match Self::export_session_review_as_markdown_with_dialog(sess) {
                            Ok(path) => {
                                // Use a robust internal marker (not user-visible words) so the scoping
                                // heuristic in the review screen is not fragile string matching on "session"/"review".
                                self.export_status = Some(format!("__SESSION_REVIEW__ Exported to: {}", path.display()));
                            }
                            Err(e) => {
                                self.export_status = Some(format!("Export failed: {}", e));
                            }
                        }
                    }
                }
            });
        });
    }
}

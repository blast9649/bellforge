//! Persistence layer for bellforge templates (PR 2)
//!
//! Templates are stored as individual TOML files in:
//!   $XDG_DATA_HOME/bellforge/templates/   (usually ~/.local/share/bellforge/templates/)
//!
//! This makes them easy to edit by hand and back up.

use std::fs;
use std::path::PathBuf;

use crate::models::WorkoutTemplate;

/// Returns the directory where user templates are stored.
pub fn templates_dir() -> PathBuf {
    // Use the `xdg` crate for proper XDG Base Directory compliance
    let xdg_dirs = xdg::BaseDirectories::with_prefix("bellforge")
        .unwrap_or_else(|_| xdg::BaseDirectories::new().unwrap());

    xdg_dirs.get_data_home().join("templates")
}

/// Ensure the templates directory exists.
pub fn ensure_templates_dir() -> std::io::Result<PathBuf> {
    let dir = templates_dir();
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Load all user templates from disk.
pub fn load_user_templates() -> Vec<WorkoutTemplate> {
    let dir = match ensure_templates_dir() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Warning: Could not create templates directory: {e}");
            return Vec::new();
        }
    };

    let mut templates = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                match fs::read_to_string(&path) {
                    Ok(content) => match toml::from_str::<WorkoutTemplate>(&content) {
                        Ok(template) => templates.push(template),
                        Err(e) => {
                            eprintln!("Failed to parse template {}: {}", path.display(), e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    // Sort by name for nicer display
    templates.sort_by(|a, b| a.name.cmp(&b.name));
    templates
}

/// Save (or overwrite) a single template as TOML.
pub fn save_template(template: &WorkoutTemplate) -> std::io::Result<PathBuf> {
    let dir = ensure_templates_dir()?;
    let safe_name = sanitize_filename(&template.name);
    let filename = format!("{}.toml", safe_name);
    let path = dir.join(filename);

    let toml = toml::to_string_pretty(template).map_err(std::io::Error::other)?;

    fs::write(&path, toml)?;
    Ok(path)
}

/// Very basic filename sanitization.
fn sanitize_filename(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' | '-' | '_' => c,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

/// Delete a user template file from disk.
pub fn delete_template(template: &WorkoutTemplate) -> std::io::Result<()> {
    let dir = templates_dir();
    let safe_name = sanitize_filename(&template.name);
    let filename = format!("{}.toml", safe_name);
    let path = dir.join(filename);

    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

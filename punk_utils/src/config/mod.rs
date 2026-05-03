pub mod keybinder;

use serde::Deserialize;
use std::fs;
use std::path::Path;

const DEFAULT_GUI_FONT_FAMILY: &str = "JetBrainsMonoNerdFont-Regular";
const DEFAULT_GUI_FONT_SIZE: f32 = 14.0;

#[derive(Debug, Clone)]
pub struct GuiConfig {
    pub font_family: String,
    pub font_size: f32,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            font_family: DEFAULT_GUI_FONT_FAMILY.to_string(),
            font_size: DEFAULT_GUI_FONT_SIZE,
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawGuiConfig {
    font_family: Option<String>,
    font_size: Option<f32>,
}

pub fn path_of_file(file_name: String) -> String {
    let mut base_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    base_path.push_str("/resources/");
    base_path.push_str(file_name.as_str());
    base_path
}

pub fn path_to_user_conf(file_name: String) -> String {
    let mut base_path: String = std::env::var("HOME").unwrap();
    base_path.push_str("/.config/punk/");
    base_path.push_str(file_name.as_str());
    base_path
}

pub fn load_gui_config() -> GuiConfig {
    let cfg_path = path_to_user_conf("gui.yml".to_string());
    load_gui_config_from_path(Path::new(&cfg_path))
}

fn load_gui_config_from_path(path: &Path) -> GuiConfig {
    let default_cfg = GuiConfig::default();

    let raw_yaml = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return default_cfg,
    };

    parse_gui_config_yaml(&raw_yaml)
}

fn parse_gui_config_yaml(raw_yaml: &str) -> GuiConfig {
    let default_cfg = GuiConfig::default();
    let parsed: RawGuiConfig = match serde_yaml::from_str(&raw_yaml) {
        Ok(v) => v,
        Err(_) => return default_cfg,
    };

    let font_family = parsed
        .font_family
        .filter(|f| !f.trim().is_empty())
        .unwrap_or(default_cfg.font_family.clone());
    let font_size = parsed
        .font_size
        .filter(|s| s.is_finite() && *s > 0.0)
        .unwrap_or(default_cfg.font_size);

    GuiConfig {
        font_family,
        font_size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file_path(name: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("punk_{name}_{nanos}.yml"))
    }

    #[test]
    fn gui_config_defaults_when_missing() {
        let path = temp_file_path("missing");
        let cfg = load_gui_config_from_path(&path);
        assert_eq!(cfg.font_family, DEFAULT_GUI_FONT_FAMILY);
        assert_eq!(cfg.font_size, DEFAULT_GUI_FONT_SIZE);
    }

    #[test]
    fn gui_config_partial_font_size_only() {
        let cfg = parse_gui_config_yaml("font_size: 18");
        assert_eq!(cfg.font_family, DEFAULT_GUI_FONT_FAMILY);
        assert_eq!(cfg.font_size, 18.0);
    }

    #[test]
    fn gui_config_valid_both_fields() {
        let cfg =
            parse_gui_config_yaml("font_family: JetBrainsMonoNerdFont-Regular\nfont_size: 16.5\n");
        assert_eq!(cfg.font_family, "JetBrainsMonoNerdFont-Regular");
        assert_eq!(cfg.font_size, 16.5);
    }

    #[test]
    fn gui_config_invalid_yaml_falls_back() {
        let cfg = parse_gui_config_yaml("font_size: [oops");
        assert_eq!(cfg.font_family, DEFAULT_GUI_FONT_FAMILY);
        assert_eq!(cfg.font_size, DEFAULT_GUI_FONT_SIZE);
    }
}

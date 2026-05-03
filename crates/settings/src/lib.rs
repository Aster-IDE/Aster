use serde::{Deserialize, Serialize};
use egui;

pub struct CherryBlossomTheme;

impl CherryBlossomTheme {
    pub fn BG_DARKEST() -> egui::Color32 { egui::Color32::from_rgb(35, 20, 28) }
    pub fn BG_DARK() -> egui::Color32 { egui::Color32::from_rgb(45, 28, 38) }
    pub fn BG_MID() -> egui::Color32 { egui::Color32::from_rgb(55, 35, 45) }
    pub fn BG_LIGHT() -> egui::Color32 { egui::Color32::from_rgb(70, 45, 58) }
    pub fn BG_LIGHTER() -> egui::Color32 { egui::Color32::from_rgb(85, 55, 70) }
    pub fn TEXT_PRIMARY() -> egui::Color32 { egui::Color32::from_rgb(255, 235, 245) }
    pub fn TEXT_SECONDARY() -> egui::Color32 { egui::Color32::from_rgb(200, 160, 180) }
    pub fn TEXT_MUTED() -> egui::Color32 { egui::Color32::from_rgb(150, 110, 130) }
    pub fn ACCENT_PINK() -> egui::Color32 { egui::Color32::from_rgb(255, 130, 180) }
    pub fn ACCENT_HOT() -> egui::Color32 { egui::Color32::from_rgb(255, 90, 150) }
    pub fn ACCENT_LIGHT() -> egui::Color32 { egui::Color32::from_rgb(255, 200, 220) }
}

pub fn setting_card(
    ui: &mut egui::Ui,
    title: &str,
    content: impl FnOnce(&mut egui::Ui),
) {
    let card_margin = 16.0;
    let corner_roundness = 8.0;
    let is_dark = ui.ctx().style().visuals.dark_mode;

    let (bg_color, stroke_color, text_color) = if is_dark {
        (
            CherryBlossomTheme::BG_DARK(),
            CherryBlossomTheme::BG_LIGHT(),
            CherryBlossomTheme::TEXT_PRIMARY(),
        )
    } else {
        (
            egui::Color32::from_rgb(240, 240, 245),
            egui::Color32::from_rgb(200, 200, 210),
            egui::Color32::from_rgb(40, 40, 40),
        )
    };

    egui::Frame::group(ui.style())
        .fill(bg_color)
        .rounding(egui::Rounding::same(corner_roundness))
        .stroke(egui::Stroke::new(1.0, stroke_color))
        .inner_margin(egui::Margin::same(card_margin))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.add(
                egui::Label::new(
                    egui::RichText::new(title)
                        .size(14.0)
                        .strong()
                        .color(text_color)
                ).selectable(false)
            );

            ui.add_space(12.0);

            ui.painter().line_segment(
                [
                    ui.cursor().left_center(),
                    ui.cursor().left_center() + egui::vec2(ui.available_width(), 0.0),
                ],
                egui::Stroke::new(1.0, stroke_color),
            );
            ui.add_space(12.0);

            content(ui);
        });
}

pub fn cozy_row(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    control: impl FnOnce(&mut egui::Ui),
) {
    let is_dark = ui.ctx().style().visuals.dark_mode;
    let (title_color, desc_color) = if is_dark {
        (
            CherryBlossomTheme::TEXT_PRIMARY(),
            CherryBlossomTheme::TEXT_MUTED(),
        )
    } else {
        (
            egui::Color32::from_rgb(40, 40, 40),
            egui::Color32::from_rgb(100, 100, 100),
        )
    };

    ui.horizontal(|ui| {
        ui.set_width(ui.available_width());

        ui.vertical(|ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(title)
                        .size(13.0)
                        .color(title_color)
                ).selectable(false)
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(description)
                        .size(11.0)
                        .color(desc_color)
                ).selectable(false)
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            control(ui);
        });
    });

    ui.add_space(12.0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabOrientation {
    Horizontal,
    Vertical,
}

impl Default for TabOrientation {
    fn default() -> Self {
        TabOrientation::Horizontal
    }
}

impl TabOrientation {
    pub fn name(&self) -> &'static str {
        match self {
            TabOrientation::Horizontal => "Horizontal",
            TabOrientation::Vertical => "Vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabStyle {
    Tabs,
    Spaces,
}

impl Default for TabStyle {
    fn default() -> Self {
        TabStyle::Spaces
    }
}

impl TabStyle {
    pub fn name(&self) -> &'static str {
        match self {
            TabStyle::Tabs => "Tabs",
            TabStyle::Spaces => "Spaces",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemePreference {
    System,
    Dark,
    Light,
}

impl Default for ThemePreference {
    fn default() -> Self {
        ThemePreference::System
    }
}

impl ThemePreference {
    pub fn name(&self) -> &'static str {
        match self {
            ThemePreference::System => "System",
            ThemePreference::Dark => "Dark",
            ThemePreference::Light => "Light",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme_preference: ThemePreference,
    pub tab_orientation: TabOrientation,
    pub tab_style: TabStyle,
    pub tab_size: usize,
    pub word_wrap: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme_preference: ThemePreference::System,
            tab_orientation: TabOrientation::Horizontal,
            tab_style: TabStyle::Spaces,
            tab_size: 4,
            word_wrap: true,
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                setting_card(ui, "Appearance", |ui| {
                    cozy_row(ui, "Theme", "Select the color scheme", |ui| {
                        egui::ComboBox::from_id_salt("theme_preference")
                            .selected_text(self.theme_preference.name())
                            .width(120.0)
                            .show_ui(ui, |ui| {
                                for preference in [
                                    ThemePreference::System,
                                    ThemePreference::Dark,
                                    ThemePreference::Light,
                                ] {
                                    if ui
                                        .selectable_label(self.theme_preference == preference, preference.name())
                                        .clicked()
                                    {
                                        self.theme_preference = preference;
                                    }
                                }
                            });
                    });
                    
                    cozy_row(ui, "Word Wrap", "Wrap lines at window edge", |ui| {
                        ui.checkbox(&mut self.word_wrap, "");
                    });
                });
                
                ui.add_space(12.0);
                
                setting_card(ui, "Editor", |ui| {
                    cozy_row(ui, "Tab Style", "Use tabs or spaces for indentation", |ui| {
                        egui::ComboBox::from_id_salt("tab_style")
                            .selected_text(self.tab_style.name())
                            .width(100.0)
                            .show_ui(ui, |ui| {
                                for style in [TabStyle::Tabs, TabStyle::Spaces] {
                                    if ui
                                        .selectable_label(self.tab_style == style, style.name())
                                        .clicked()
                                    {
                                        self.tab_style = style;
                                    }
                                }
                            });
                    });
                    
                    cozy_row(ui, "Tab Size", "Number of spaces per tab", |ui| {
                        ui.add(egui::Slider::new(&mut self.tab_size, 1..=8).show_value(true));
                    });
                    
                    cozy_row(ui, "Tab Orientation", "Horizontal or vertical tab bar", |ui| {
                        egui::ComboBox::from_id_salt("tab_orientation")
                            .selected_text(self.tab_orientation.name())
                            .width(100.0)
                            .show_ui(ui, |ui| {
                                for orientation in [TabOrientation::Horizontal, TabOrientation::Vertical] {
                                    if ui
                                        .selectable_label(self.tab_orientation == orientation, orientation.name())
                                        .clicked()
                                    {
                                        self.tab_orientation = orientation;
                                    }
                                }
                            });
                    });
                });
            });
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }

    pub fn config_path() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|dir| dir.join("aster").join("config.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                if let Some(settings) = Self::from_json(&contents) {
                    return settings;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&path, self.to_json());
        }
    }
}

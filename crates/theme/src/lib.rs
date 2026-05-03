use egui::Color32;

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub bg_darkest: Color32,
    pub bg_dark: Color32,
    pub bg_mid: Color32,
    pub bg_light: Color32,
    pub bg_lighter: Color32,
    pub border: Color32,

    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,

    pub accent_primary: Color32,
    pub accent_hot: Color32,
    pub accent_light: Color32,
}

impl ThemeColors {
    pub const fn cherry_blossom_dark() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(35, 20, 28),
            bg_dark: Color32::from_rgb(45, 28, 38),
            bg_mid: Color32::from_rgb(55, 35, 45),
            bg_light: Color32::from_rgb(70, 45, 58),
            bg_lighter: Color32::from_rgb(85, 55, 70),
            border: Color32::from_rgb(85, 55, 70),
            text_primary: Color32::from_rgb(255, 235, 245),
            text_secondary: Color32::from_rgb(200, 160, 180),
            text_muted: Color32::from_rgb(150, 110, 130),
            accent_primary: Color32::from_rgb(255, 130, 180),
            accent_hot: Color32::from_rgb(255, 90, 150),
            accent_light: Color32::from_rgb(255, 200, 220),
        }
    }

    pub const fn cherry_blossom_light() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(245, 220, 230),
            bg_dark: Color32::from_rgb(255, 248, 252),
            bg_mid: Color32::from_rgb(255, 240, 248),
            bg_light: Color32::from_rgb(245, 230, 240),
            bg_lighter: Color32::from_rgb(235, 220, 230),
            border: Color32::from_rgb(235, 220, 230),
            text_primary: Color32::from_rgb(80, 40, 60),
            text_secondary: Color32::from_rgb(130, 80, 105),
            text_muted: Color32::from_rgb(170, 120, 145),
            accent_primary: Color32::from_rgb(220, 80, 140),
            accent_hot: Color32::from_rgb(255, 90, 150),
            accent_light: Color32::from_rgb(255, 180, 210),
        }
    }

    pub const fn rose_pine() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(31, 29, 46),
            bg_dark: Color32::from_rgb(38, 35, 58),
            bg_mid: Color32::from_rgb(33, 32, 46),
            bg_light: Color32::from_rgb(49, 46, 73),
            bg_lighter: Color32::from_rgb(64, 61, 82),
            border: Color32::from_rgb(82, 79, 103),
            text_primary: Color32::from_rgb(224, 222, 244),
            text_secondary: Color32::from_rgb(144, 140, 170),
            text_muted: Color32::from_rgb(110, 106, 134),
            accent_primary: Color32::from_rgb(196, 167, 231),
            accent_hot: Color32::from_rgb(196, 167, 231),
            accent_light: Color32::from_rgb(156, 207, 216),
        }
    }

    pub const fn rose_pine_moon() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(35, 33, 54),
            bg_dark: Color32::from_rgb(42, 39, 63),
            bg_mid: Color32::from_rgb(57, 53, 82),
            bg_light: Color32::from_rgb(42, 40, 62),
            bg_lighter: Color32::from_rgb(68, 65, 90),
            border: Color32::from_rgb(86, 82, 110),
            text_primary: Color32::from_rgb(224, 222, 244),
            text_secondary: Color32::from_rgb(144, 140, 170),
            text_muted: Color32::from_rgb(110, 106, 134),
            accent_primary: Color32::from_rgb(196, 167, 231),
            accent_hot: Color32::from_rgb(196, 167, 231),
            accent_light: Color32::from_rgb(156, 207, 216),
        }
    }

    pub const fn rose_pine_dawn() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(250, 244, 237),
            bg_dark: Color32::from_rgb(255, 250, 243),
            bg_mid: Color32::from_rgb(242, 233, 222),
            bg_light: Color32::from_rgb(223, 218, 211),
            bg_lighter: Color32::from_rgb(206, 202, 195),
            border: Color32::from_rgb(189, 185, 177),
            text_primary: Color32::from_rgb(87, 82, 91),
            text_secondary: Color32::from_rgb(121, 112, 122),
            text_muted: Color32::from_rgb(152, 147, 165),
            accent_primary: Color32::from_rgb(196, 167, 231),
            accent_hot: Color32::from_rgb(196, 167, 231),
            accent_light: Color32::from_rgb(156, 207, 216),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    CherryBlossomDark,
    CherryBlossomLight,
    RosePine,
    RosePineMoon,
    RosePineDawn,
}

impl ThemeVariant {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::CherryBlossomDark => "Dark",
            ThemeVariant::CherryBlossomLight => "Light",
            ThemeVariant::RosePine => "Rose Pine",
            ThemeVariant::RosePineMoon => "Rose Pine Moon",
            ThemeVariant::RosePineDawn => "Rose Pine Dawn",
        }
    }

    pub fn colors(&self) -> ThemeColors {
        match self {
            ThemeVariant::CherryBlossomDark => ThemeColors::cherry_blossom_dark(),
            ThemeVariant::CherryBlossomLight => ThemeColors::cherry_blossom_light(),
            ThemeVariant::RosePine => ThemeColors::rose_pine(),
            ThemeVariant::RosePineMoon => ThemeColors::rose_pine_moon(),
            ThemeVariant::RosePineDawn => ThemeColors::rose_pine_dawn(),
        }
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let colors = self.colors();
        let is_dark = colors.bg_darkest.r() < 128;
        let mut visuals = if is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        let rounding = 8.0;

        visuals.window_fill = colors.bg_dark;
        visuals.panel_fill = colors.bg_dark;
        visuals.window_stroke = egui::Stroke::new(1.0, colors.bg_light);

        visuals.widgets.noninteractive.rounding = rounding.into();
        visuals.widgets.noninteractive.bg_fill = colors.bg_mid;
        visuals.widgets.inactive.rounding = rounding.into();
        visuals.widgets.inactive.bg_fill = colors.bg_light;
        visuals.widgets.hovered.rounding = rounding.into();
        visuals.widgets.hovered.bg_fill = colors.bg_lighter;
        visuals.widgets.active.rounding = rounding.into();
        visuals.widgets.active.bg_fill = colors.accent_hot;
        visuals.widgets.open.rounding = rounding.into();
        visuals.widgets.open.bg_fill = colors.accent_primary;

        visuals.selection.bg_fill = colors.accent_primary;
        visuals.selection.stroke = egui::Stroke::new(1.0, colors.accent_light);

        visuals.override_text_color = Some(colors.text_primary);
        visuals.hyperlink_color = colors.accent_primary;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, colors.text_primary);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, colors.accent_light);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, colors.bg_darkest);

        ctx.set_visuals(visuals);

        let mut style = egui::Style::default();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(13.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Button, egui::FontId::new(13.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(11.0, egui::FontFamily::Proportional)),
        ].into();
        ctx.set_style(style);
    }
}

pub struct CherryBlossomTheme;

impl CherryBlossomTheme {
    pub fn BG_DARKEST() -> Color32 { ThemeColors::cherry_blossom_dark().bg_darkest }
    pub fn BG_DARK() -> Color32 { ThemeColors::cherry_blossom_dark().bg_dark }
    pub fn BG_MID() -> Color32 { ThemeColors::cherry_blossom_dark().bg_mid }
    pub fn BG_LIGHT() -> Color32 { ThemeColors::cherry_blossom_dark().bg_light }
    pub fn BG_LIGHTER() -> Color32 { ThemeColors::cherry_blossom_dark().bg_lighter }
    pub fn BORDER_PINK() -> Color32 { ThemeColors::cherry_blossom_dark().border }
    pub fn TEXT_PRIMARY() -> Color32 { ThemeColors::cherry_blossom_dark().text_primary }
    pub fn TEXT_SECONDARY() -> Color32 { ThemeColors::cherry_blossom_dark().text_secondary }
    pub fn TEXT_MUTED() -> Color32 { ThemeColors::cherry_blossom_dark().text_muted }
    pub fn ACCENT_PINK() -> Color32 { ThemeColors::cherry_blossom_dark().accent_primary }
    pub fn ACCENT_HOT() -> Color32 { ThemeColors::cherry_blossom_dark().accent_hot }
    pub fn ACCENT_LIGHT() -> Color32 { ThemeColors::cherry_blossom_dark().accent_light }

    pub fn apply(ctx: &egui::Context) {
        ThemeVariant::CherryBlossomDark.apply(ctx);
    }
}

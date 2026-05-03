use egui;

pub struct OrgParser;

impl OrgParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn render_to_egui(&self, ui: &mut egui::Ui, text: &str) {
        // TODO: Implement Org mode parsing
        ui.label("Org format not yet supported");
        ui.monospace(text);
    }
    
    pub fn render_to_html(&self, text: &str, _is_dark: bool) -> String {
        // TODO: Implement Org mode parsing
        format!("<pre>{}</pre>", html_escape(text))
    }
}

impl Default for OrgParser {
    fn default() -> Self {
        Self::new()
    }
}

fn html_escape(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => ch.to_string(),
        })
        .collect()
}

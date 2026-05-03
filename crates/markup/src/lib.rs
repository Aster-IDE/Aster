use egui;
use std::path::Path;

pub mod formats;

pub use formats::markdown::MarkdownParser;

#[derive(Debug, Clone)]
pub enum MarkupFormat {
    Markdown,
    Rst,
    Org,
    Adoc,
    Plain,
}

impl MarkupFormat {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "md" | "markdown" => MarkupFormat::Markdown,
            "rst" => MarkupFormat::Rst,
            "org" => MarkupFormat::Org,
            "adoc" | "asciidoc" => MarkupFormat::Adoc,
            _ => MarkupFormat::Plain,
        }
    }
    
    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| Self::from_extension(ext))
            .unwrap_or(MarkupFormat::Plain)
    }
}

pub struct MarkupPreview {
    markdown_parser: MarkdownParser,
    // todo:
    // rst_parser: RstParser,
    // org_parser: OrgParser,
    // adoc_parser: AdocParser,
}

impl MarkupPreview {
    pub fn new() -> Self {
        Self {
            markdown_parser: MarkdownParser::new(),
        }
    }
    
    pub fn render_to_egui(&self, ui: &mut egui::Ui, text: &str, format: &MarkupFormat) {
        let is_dark = ui.ctx().style().visuals.dark_mode;
        match format {
            MarkupFormat::Markdown => {
                self.markdown_parser.render_to_egui_with_theme(ui, text, is_dark);
            }
            MarkupFormat::Rst => {
                ui.label("RST format not yet supported");
                ui.monospace(text);
            }
            MarkupFormat::Org => {
                ui.label("Org format not yet supported");
                ui.monospace(text);
            }
            MarkupFormat::Adoc => {
                ui.label("AsciiDoc format not yet supported");
                ui.monospace(text);
            }
            MarkupFormat::Plain => {
                ui.monospace(text);
            }
        }
    }
    
    pub fn render_to_html(&self, text: &str, format: &MarkupFormat, is_dark: bool) -> String {
        match format {
            MarkupFormat::Markdown => {
                self.markdown_parser.render_to_html(text, is_dark)
            }
            MarkupFormat::Rst => {
                format!("<pre>{}</pre>", html_escape(text))
            }
            MarkupFormat::Org => {
                format!("<pre>{}</pre>", html_escape(text))
            }
            MarkupFormat::Adoc => {
                format!("<pre>{}</pre>", html_escape(text))
            }
            MarkupFormat::Plain => {
                format!("<pre>{}</pre>", html_escape(text))
            }
        }
    }
    
    pub fn detect_format(&self, text: &str, path: Option<&Path>) -> MarkupFormat {
        if let Some(path) = path {
            return MarkupFormat::from_path(path);
        }
        
        if text.trim().is_empty() {
            return MarkupFormat::Plain;
        }
        
        let lines: Vec<&str> = text.lines().take(10).collect();
        
        let markdown_indicators = [
            lines.iter().any(|line| line.starts_with('#')),
            lines.iter().any(|line| line.starts_with("- ") || line.starts_with("* ")),
            lines.iter().any(|line| line.contains("```")),
            lines.iter().any(|line| line.contains("**") || line.contains("*")),
        ];
        
        if markdown_indicators.iter().any(|&x| x) {
            return MarkupFormat::Markdown;
        }
        
        let rst_indicators = [
            lines.iter().any(|line| line.starts_with(".. ")),
            lines.iter().any(|line| line.contains("===") || line.contains("---")),
            lines.iter().any(|line| line.starts_with(":")),
        ];
        
        if rst_indicators.iter().any(|&x| x) {
            return MarkupFormat::Rst;
        }
        
        let org_indicators = [
            lines.iter().any(|line| line.starts_with("* ")),
            lines.iter().any(|line| line.starts_with("#+")),
        ];
        
        if org_indicators.iter().any(|&x| x) {
            return MarkupFormat::Org;
        }
        
        MarkupFormat::Plain
    }
}

impl Default for MarkupPreview {
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
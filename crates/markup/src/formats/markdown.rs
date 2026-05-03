use pulldown_cmark::{Parser, Options, CodeBlockKind, Tag, Event, HeadingLevel};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use egui;

pub struct MarkdownParser {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl MarkdownParser {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }
    
    pub fn render_to_egui(&self, ui: &mut egui::Ui, text: &str) {
        self.render_to_egui_with_theme(ui, text, ui.ctx().style().visuals.dark_mode)
    }
    
    pub fn render_to_egui_with_theme(&self, ui: &mut egui::Ui, text: &str, is_dark: bool) {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        
        let parser = Parser::new_ext(text, options);
        
        let mut in_code_block = false;
        let mut in_paragraph = false;
        let mut list_depth: i32 = 0;
        let mut current_text = String::new();
        let mut code_block_content = String::new();
        let mut code_block_lang = String::new();
        let mut emphasis_stack = Vec::new();
        let mut link_text = String::new();
        let mut link_url = String::new();
        
        ui.add_space(8.0);
        
        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Heading(level, _, _) => {
                            ui.add_space(12.0);
                            if !current_text.is_empty() {
                                self.render_formatted_text(ui, &current_text, &emphasis_stack, &link_text, &link_url);
                                current_text.clear();
                                emphasis_stack.clear();
                                link_text.clear();
                                link_url.clear();
                            }
                        }
                        Tag::Paragraph => {
                            in_paragraph = true;
                        }
                        Tag::BlockQuote => {
                            ui.add_space(5.0);
                        }
                        Tag::CodeBlock(CodeBlockKind::Fenced(lang)) => {
                            in_code_block = true;
                            code_block_lang = lang.to_string();
                            code_block_content.clear();
                        }
                        Tag::List(_) => {
                            list_depth += 1;
                        }
                        Tag::Item => {
                            ui.add_space(4.0);
                        }
                        Tag::Emphasis => {
                            if !current_text.is_empty() {
                                self.render_formatted_text(ui, &current_text, &emphasis_stack, &link_text, &link_url);
                                current_text.clear();
                            }
                            emphasis_stack.push("italic");
                        }
                        Tag::Strong => {
                            if !current_text.is_empty() {
                                self.render_formatted_text(ui, &current_text, &emphasis_stack, &link_text, &link_url);
                                current_text.clear();
                            }
                            emphasis_stack.push("bold");
                        }
                        Tag::Strikethrough => {
                            if !current_text.is_empty() {
                                self.render_formatted_text(ui, &current_text, &emphasis_stack, &link_text, &link_url);
                                current_text.clear();
                            }
                            emphasis_stack.push("strikethrough");
                        }
                        Tag::Link(_, url, _) => {
                            link_url = url.to_string();
                        }
                        _ => {}
                    }
                }
                Event::End(tag) => {
                    match tag {
                        Tag::Heading(level, _, _) => {
                            let heading_text = current_text.clone();
                            if !heading_text.is_empty() {
                                let formatted = self.apply_formatting(&heading_text, &emphasis_stack);
                                match level {
                                    HeadingLevel::H1 => { 
                                        ui.label(formatted.size(28.0).strong()); 
                                        ui.add_space(8.0);
                                    }
                                    HeadingLevel::H2 => { 
                                        ui.label(formatted.size(24.0).strong()); 
                                        ui.add_space(6.0);
                                    }
                                    HeadingLevel::H3 => { 
                                        ui.label(formatted.size(20.0).strong()); 
                                        ui.add_space(4.0);
                                    }
                                    HeadingLevel::H4 => { 
                                        ui.label(formatted.size(18.0).strong()); 
                                        ui.add_space(3.0);
                                    }
                                    HeadingLevel::H5 => { 
                                        ui.label(formatted.size(16.0).strong()); 
                                        ui.add_space(2.0);
                                    }
                                    HeadingLevel::H6 => { 
                                        ui.label(formatted.size(14.0).strong()); 
                                        ui.add_space(2.0);
                                    }
                                }
                                current_text.clear();
                            }
                            emphasis_stack.clear();
                        }
                        Tag::Paragraph => {
                            if in_paragraph && !current_text.is_empty() {
                                self.render_formatted_text(ui, &current_text, &emphasis_stack, &link_text, &link_url);
                                current_text.clear();
                                ui.add_space(12.0);
                            }
                            in_paragraph = false;
                            emphasis_stack.clear();
                            link_text.clear();
                            link_url.clear();
                        }
                        Tag::BlockQuote => {
                            if !current_text.is_empty() {
                                ui.add_space(4.0);
                                let (bar_color, text_color) = if is_dark {
                                    (egui::Color32::from_rgb(150, 150, 150), egui::Color32::from_rgb(120, 120, 120))
                                } else {
                                    (egui::Color32::from_rgb(100, 100, 100), egui::Color32::from_rgb(80, 80, 80))
                                };
                                
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("│").color(bar_color));
                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new(&current_text).color(text_color).italics());
                                });
                                ui.add_space(4.0);
                                current_text.clear();
                            }
                        }
                        Tag::CodeBlock(_) => {
                            if in_code_block {
                                ui.add_space(8.0);
                                let (bg_color, text_color, lang_color) = if is_dark {
                                    (egui::Color32::from_rgb(40, 44, 52), egui::Color32::from_rgb(220, 220, 220), egui::Color32::from_rgb(150, 150, 150))
                                } else {
                                    (egui::Color32::from_rgb(245, 245, 245), egui::Color32::from_rgb(40, 40, 40), egui::Color32::from_rgb(100, 100, 100))
                                };
                                
                                egui::Frame::none()
                                    .fill(bg_color)
                                    .rounding(4.0)
                                    .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                                    .show(ui, |ui| {
                                        if !code_block_lang.is_empty() {
                                            ui.label(egui::RichText::new(format!("📝 {}", code_block_lang))
                                                .color(lang_color)
                                                .size(12.0));
                                            ui.add_space(4.0);
                                        }
                                        ui.monospace(egui::RichText::new(&code_block_content)
                                            .color(text_color));
                                    });
                                ui.add_space(8.0);
                                in_code_block = false;
                                code_block_content.clear();
                                code_block_lang.clear();
                            }
                        }
                        Tag::List(_) => {
                            list_depth = list_depth.saturating_sub(1);
                        }
                        Tag::Emphasis | Tag::Strong | Tag::Strikethrough => {
                            if let Some(_) = emphasis_stack.pop() {
                            }
                        }
                        Tag::Link(_, _, _) => {
                            link_text = current_text.clone();
                            if !link_text.is_empty() && !link_url.is_empty() {
                                ui.hyperlink_to(&link_text, &link_url);
                            }
                            current_text.clear();
                            link_text.clear();
                            link_url.clear();
                        }
                        _ => {}
                    }
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else {
                        if list_depth > 0 && !current_text.is_empty() {
                            ui.horizontal(|ui| {
                                for _ in 0..list_depth {
                                    ui.add_space(24.0);
                                }
                                ui.label(egui::RichText::new("•").size(14.0));
                                ui.add_space(8.0);
                                let formatted = self.apply_formatting(&text, &emphasis_stack);
                                ui.label(formatted);
                            });
                            ui.add_space(2.0);
                        } else {
                            current_text.push_str(&text);
                        }
                    }
                }
                Event::Code(code) => {
                    if !current_text.is_empty() {
                        self.render_formatted_text(ui, &current_text, &emphasis_stack, &link_text, &link_url);
                        current_text.clear();
                    }
                    let (bg_color, text_color) = if is_dark {
                        (egui::Color32::from_rgb(60, 65, 75), egui::Color32::from_rgb(220, 220, 220))
                    } else {
                        (egui::Color32::from_rgb(230, 230, 230), egui::Color32::from_rgb(40, 40, 40))
                    };
                    
                    egui::Frame::none()
                        .fill(bg_color)
                        .rounding(3.0)
                        .inner_margin(egui::Margin::symmetric(4.0, 2.0))
                        .show(ui, |ui| {
                            ui.monospace(egui::RichText::new(format!("{}", code))
                                .color(text_color)
                                .size(13.0));
                        });
                }
                Event::SoftBreak | Event::HardBreak => {
                    if in_code_block {
                        code_block_content.push('\n');
                    } else {
                        if !current_text.is_empty() {
                            self.render_formatted_text(ui, &current_text, &emphasis_stack, &link_text, &link_url);
                            current_text.clear();
                        }
                        ui.add_space(4.0);
                    }
                }
                Event::Rule => {
                    if !current_text.is_empty() {
                        self.render_formatted_text(ui, &current_text, &emphasis_stack, &link_text, &link_url);
                        current_text.clear();
                    }
                    ui.add_space(8.0);
                    let rule_color = if is_dark {
                        egui::Color32::from_rgb(128, 128, 128)
                    } else {
                        egui::Color32::from_rgb(180, 180, 180)
                    };
                    
                    let available_width = ui.available_width();
                    ui.painter().line_segment(
                        [
                            ui.cursor().left_top() + egui::vec2(0.0, 1.0),
                            ui.cursor().left_top() + egui::vec2(available_width, 1.0),
                        ],
                        egui::Stroke::new(2.0, rule_color),
                    );
                    ui.add_space(16.0);
                }
                Event::TaskListMarker(checked) => {
                    let marker = if checked { "☑" } else { "☐" };
                    let (checked_color, unchecked_color) = if is_dark {
                        (egui::Color32::from_rgb(100, 200, 100), egui::Color32::from_rgb(150, 150, 150))
                    } else {
                        (egui::Color32::from_rgb(50, 150, 50), egui::Color32::from_rgb(120, 120, 120))
                    };
                    
                    ui.label(egui::RichText::new(marker)
                        .color(if checked { checked_color } else { unchecked_color })
                        .size(14.0));
                }
                _ => {}
            }
        }
        
        if !current_text.is_empty() {
            self.render_formatted_text(ui, &current_text, &emphasis_stack, &link_text, &link_url);
        }
    }
    
    fn render_formatted_text(&self, ui: &mut egui::Ui, text: &str, emphasis_stack: &[&str], link_text: &str, link_url: &str) {
        if text.trim().is_empty() {
            return;
        }
        
        if !link_text.is_empty() && !link_url.is_empty() {
            ui.hyperlink_to(link_text, link_url);
        } else {
            let formatted = self.apply_formatting(text, emphasis_stack);
            ui.label(formatted);
        }
    }
    
    fn apply_formatting(&self, text: &str, emphasis_stack: &[&str]) -> egui::RichText {
        let mut rich_text = egui::RichText::new(text);
        
        for emphasis in emphasis_stack {
            match *emphasis {
                "bold" => rich_text = rich_text.strong(),
                "italic" => rich_text = rich_text.italics(),
                "strikethrough" => rich_text = rich_text.strikethrough(),
                _ => {}
            }
        }
        
        rich_text
    }
    
    pub fn render_to_html(&self, text: &str, is_dark: bool) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        
        let parser = Parser::new_ext(text, options);
        
        let mut html_output = String::new();
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_block_content = String::new();
        
        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    in_code_block = true;
                    code_block_lang = lang.to_string();
                    code_block_content.clear();
                }
                Event::End(Tag::CodeBlock(_)) => {
                    if in_code_block {
                        let highlighted = if code_block_lang.is_empty() {
                            let mut escaped = String::new();
                            for ch in code_block_content.chars() {
                                match ch {
                                    '&' => escaped.push_str("&amp;"),
                                    '<' => escaped.push_str("&lt;"),
                                    '>' => escaped.push_str("&gt;"),
                                    '"' => escaped.push_str("&quot;"),
                                    '\'' => escaped.push_str("&#x27;"),
                                    _ => escaped.push(ch),
                                }
                            }
                            format!("<pre><code>{}</code></pre>", escaped)
                        } else {
                            match self.syntax_set.find_syntax_by_token(&code_block_lang) {
                                Some(syntax) => {
                                    let theme = if is_dark {
                                        &self.theme_set.themes["base16-ocean.dark"]
                                    } else {
                                        &self.theme_set.themes["base16-ocean.light"]
                                    };
                                    
                                    highlighted_html_for_string(
                                        &code_block_content,
                                        &self.syntax_set,
                                        &syntax,
                                        theme,
                                    ).unwrap_or_else(|_| {
                                        let mut escaped = String::new();
                                        for ch in code_block_content.chars() {
                                            match ch {
                                                '&' => escaped.push_str("&amp;"),
                                                '<' => escaped.push_str("&lt;"),
                                                '>' => escaped.push_str("&gt;"),
                                                '"' => escaped.push_str("&quot;"),
                                                '\'' => escaped.push_str("&#x27;"),
                                                _ => escaped.push(ch),
                                            }
                                        }
                                        format!("<pre><code>{}</code></pre>", escaped)
                                    })
                                }
                                None => {
                                    let mut escaped = String::new();
                                    for ch in code_block_content.chars() {
                                        match ch {
                                            '&' => escaped.push_str("&amp;"),
                                            '<' => escaped.push_str("&lt;"),
                                            '>' => escaped.push_str("&gt;"),
                                            '"' => escaped.push_str("&quot;"),
                                            '\'' => escaped.push_str("&#x27;"),
                                            _ => escaped.push(ch),
                                        }
                                    }
                                    format!("<pre><code>{}</code></pre>", escaped)
                                }
                            }
                        };
                        html_output.push_str(&highlighted);
                        in_code_block = false;
                    }
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else {
                        for ch in text.chars() {
                            match ch {
                                '&' => html_output.push_str("&amp;"),
                                '<' => html_output.push_str("&lt;"),
                                '>' => html_output.push_str("&gt;"),
                                '"' => html_output.push_str("&quot;"),
                                '\'' => html_output.push_str("&#x27;"),
                                _ => html_output.push(ch),
                            }
                        }
                    }
                }
                Event::Code(code) => {
                    let mut escaped = String::new();
                    for ch in code.chars() {
                        match ch {
                            '&' => escaped.push_str("&amp;"),
                            '<' => escaped.push_str("&lt;"),
                            '>' => escaped.push_str("&gt;"),
                            '"' => escaped.push_str("&quot;"),
                            '\'' => escaped.push_str("&#x27;"),
                            _ => escaped.push(ch),
                        }
                    }
                    html_output.push_str(&format!("<code>{}</code>", escaped));
                }
                Event::SoftBreak | Event::HardBreak => {
                    if in_code_block {
                        code_block_content.push('\n');
                    } else {
                        html_output.push_str("<br>");
                    }
                }
                _ => {
                    if !in_code_block {
                        let mut temp_html = String::new();
                        pulldown_cmark::html::push_html(&mut temp_html, std::iter::once(event));
                        html_output.push_str(&temp_html);
                    }
                }
            }
        }
        
        html_output
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}
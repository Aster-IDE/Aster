use eframe::egui;
use std::path::PathBuf;
use aster_settings::{Settings, ThemePreference, TabOrientation};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Aster"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Aster",
        options,
        Box::new(|_cc| Ok(Box::new(Aster::default()))),
    )
}

enum Tab {
    Editor(String, Option<PathBuf>, bool),
    Settings,
}

struct Aster {
    tabs: Vec<Tab>,
    active_tab: usize,
    settings: Settings,
    font_size: f32,
}

impl Default for Aster {
    fn default() -> Self {
        Self {
            tabs: vec![Tab::Editor(String::new(), None, false)],
            active_tab: 0,
            settings: Settings::load(),
            font_size: 14.0,
        }
    }
}

impl eframe::App for Aster {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let use_dark = match self.settings.theme_preference {
            ThemePreference::System => ctx.options(|opt| opt.theme_preference) == egui::ThemePreference::Dark,
            ThemePreference::Dark => true,
            ThemePreference::Light => false,
        };
        
        if use_dark {
            let mut visuals = egui::Visuals::dark();
            visuals.window_fill = egui::Color32::from_rgb(45, 28, 38);
            visuals.panel_fill = egui::Color32::from_rgb(45, 28, 38);
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(55, 35, 45);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(85, 55, 70);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(255, 90, 150);
            visuals.override_text_color = Some(egui::Color32::from_rgb(255, 235, 245));
            ctx.set_visuals(visuals);
        } else {
            let mut visuals = egui::Visuals::light();
            visuals.window_fill = egui::Color32::from_rgb(255, 245, 250);
            visuals.panel_fill = egui::Color32::from_rgb(255, 245, 250);
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(255, 235, 245);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(250, 215, 235);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(255, 90, 150);
            visuals.override_text_color = Some(egui::Color32::from_rgb(80, 40, 60));
            ctx.set_visuals(visuals);
        }
        
        self.render_tab_bar(ctx);
        
        ctx.input(|i| {
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                if i.modifiers.shift {
                    self.save_file_as();
                } else {
                    self.save_file();
                }
            }
            if i.modifiers.command && i.key_pressed(egui::Key::N) {
                self.new_file();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                self.open_file();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::W) {
                self.close_current_tab();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::Comma) {
                self.open_settings_tab();
            }
        });

        let mod_key = if cfg!(target_os = "macos") { "Cmd" } else { "Ctrl" };

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button(format!("New         {mod_key}+N")).clicked() {
                        self.new_file();
                        ui.close_menu();
                    }
                    if ui.button(format!("Open        {mod_key}+O")).clicked() {
                        self.open_file();
                        ui.close_menu();
                    }
                    if ui.button(format!("Save        {mod_key}+S")).clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    if ui.button(format!("Save As     {mod_key}+Shift+S")).clicked() {
                        self.save_file_as();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button(format!("Close Tab   {mod_key}+W")).clicked() {
                        self.close_current_tab();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Redo").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Cut").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Copy").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Paste").clicked() {
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Select All").clicked() {
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Format", |ui| {
                    if ui.button("Word Wrap").clicked() {
                        self.settings.word_wrap = !self.settings.word_wrap;
                        self.settings.save();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Settings", |ui| {
                    if ui.button(format!("Open Settings    {mod_key}+,")).clicked() {
                        self.open_settings_tab();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Horizontal Tabs").clicked() {
                        self.settings.tab_orientation = TabOrientation::Horizontal;
                        self.settings.save();
                        ui.close_menu();
                    }
                    if ui.button("Vertical Tabs").clicked() {
                        self.settings.tab_orientation = TabOrientation::Vertical;
                        self.settings.save();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Word Wrap").clicked() {
                        self.settings.word_wrap = !self.settings.word_wrap;
                        self.settings.save();
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙").clicked() {
                        self.open_settings_tab();
                    }
                });
            });
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(egui::Color32::from_rgb(25, 15, 20)))
            .show(ctx, |ui| {
                if self.tabs.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(ui.available_height() * 0.08);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Aster 🌸")
                                    .size(48.0)
                                    .color(egui::Color32::from_rgb(255, 130, 180)),
                            ).selectable(false).sense(egui::Sense::hover())
                        );
                        ui.add_space(10.0);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("A Simpler Text Editor written in Rust.")
                                    .size(16.0)
                                    .color(egui::Color32::from_rgb(200, 160, 180)),
                            ).selectable(false).sense(egui::Sense::hover())
                        );
                    });

                    ui.add_space(60.0);

                    ui.horizontal(|ui| {
                        let total_width = ui.available_width();
                        let total_width = total_width.max(400.0);
                        let left_width = total_width;

                        ui.allocate_ui_with_layout(
                            egui::vec2(left_width, ui.available_height()),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                egui::Frame::group(&ui.style())
                                    .fill(egui::Color32::from_rgb(45, 28, 38))
                                    .inner_margin(20.0)
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 130, 180)))
                                    .show(ui, |ui| {
                                        let button_size = egui::vec2(200.0, 40.0);
                                        let button_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 130, 180));
                                        if ui
                                            .add_sized(button_size, egui::Button::new("📄  Open File").stroke(button_stroke))
                                            .clicked()
                                        {
                                            self.open_file();
                                        }
                                        ui.add_space(10.0);
                                        if ui
                                            .add_sized(button_size, egui::Button::new("📝  New File").stroke(button_stroke))
                                            .clicked()
                                        {
                                            self.new_file();
                                        }
                                        ui.add_space(10.0);
                                        if ui
                                            .add_sized(button_size, egui::Button::new("⚙  Settings").stroke(button_stroke))
                                            .clicked()
                                        {
                                            self.open_settings_tab();
                                        }
                                    });
                            },
                        );
                    });

                } else if self.active_tab < self.tabs.len() {
                    let is_editor = matches!(self.tabs[self.active_tab], Tab::Editor(_, _, _));
                    
                    if is_editor {
                        if let Tab::Editor(ref mut text, _, ref mut modified) = self.tabs[self.active_tab] {
                            let line_count = Self::get_line_count_inner(text);
                            let char_count = text.chars().count();
                            let line_ending = Self::get_line_ending_type_inner(text);
                            let cursor_pos = Self::get_cursor_position();
                            let is_modified = *modified;
                            
                            let text_edit = egui::TextEdit::multiline(text)
                                .desired_width(f32::INFINITY)
                                .font(egui::FontId::new(self.font_size, egui::FontFamily::Monospace));
                            
                            let response = ui.add(text_edit);
                            if response.changed() {
                                *modified = true;
                            }
                            
                            egui::TopBottomPanel::bottom("status_bar").show_inside(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("Ln: {}, Col: {}", line_count, cursor_pos));
                                    ui.separator();
                                    ui.label("UTF-8");
                                    ui.separator();
                                    if is_modified {
                                        ui.label("Modified");
                                    } else {
                                        ui.label("Saved");
                                    }
                                    ui.separator();
                                    ui.label(format!("{} lines", line_count));
                                    ui.separator();
                                    ui.label(format!("{} characters", char_count));
                                    ui.separator();
                                    ui.label(line_ending);
                                    ui.separator();
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label("Ready");
                                    });
                                });
                            });
                        }
                    } else {
                        self.settings.ui(ui);
                    }
                }
            });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.settings.save();
    }
}

impl Aster {
    fn render_tab_bar(&mut self, ctx: &egui::Context) {
        if self.tabs.is_empty() {
            return;
        }
        
        let is_vertical = self.settings.tab_orientation == TabOrientation::Vertical;
        
        if is_vertical {
            let tabs_info: Vec<_> = self.tabs.iter().enumerate().map(|(index, tab)| {
                let is_active = index == self.active_tab;
                let tab_text = match tab {
                    Tab::Editor(_, path, modified) => {
                        if let Some(p) = path {
                            format!("{}{}", p.file_name().unwrap_or_default().to_string_lossy(), if *modified { "*" } else { "" })
                        } else {
                            format!("Untitled{}", if *modified { "*" } else { "" })
                        }
                    }
                    Tab::Settings => "Settings".to_string(),
                };
                (index, is_active, tab_text)
            }).collect();
            let active_tab_idx = self.active_tab;
            let fixed_width = 160.0;
            let tab_height = 26.0;
            let tab_width = fixed_width - 16.0;
            
            egui::SidePanel::left("tab_bar").resizable(false).exact_width(fixed_width).show(ctx, |ui| {
                ui.add_space(4.0);
                let mut tab_to_remove: Option<usize> = None;
                let mut new_active_tab = active_tab_idx;
                
                for (index, is_active, tab_text) in tabs_info {
                    let full_tab_width = tab_width;
                    let close_width = 20.0;
                    let text_width = full_tab_width - close_width;
                    
                    let (full_rect, response) = ui.allocate_exact_size(
                        egui::vec2(full_tab_width, tab_height),
                        egui::Sense::click(),
                    );
                    
                    let bg_color = if is_active {
                        egui::Color32::from_rgb(85, 55, 70)
                    } else if response.hovered() {
                        egui::Color32::from_rgb(70, 45, 58)
                    } else {
                        egui::Color32::from_rgb(45, 28, 38)
                    };
                    
                    ui.painter().rect_filled(full_rect, 0.0, bg_color);
                    
                    if is_active {
                        let indicator_rect = egui::Rect::from_min_size(
                            full_rect.left_top() + egui::vec2(0.0, 6.0),
                            egui::vec2(3.0, 14.0),
                        );
                        ui.painter().rect_filled(indicator_rect, 0.0, egui::Color32::from_rgb(255, 130, 180));
                    }
                    
                    let text_color = if is_active {
                        egui::Color32::from_rgb(255, 235, 245)
                    } else {
                        egui::Color32::from_rgb(200, 160, 180)
                    };
                    
                    let text_rect = egui::Rect::from_min_size(
                        full_rect.left_top() + egui::vec2(if is_active { 10.0 } else { 8.0 }, 0.0),
                        egui::vec2(text_width - 16.0, tab_height),
                    );
                    ui.painter().text(
                        text_rect.left_center(),
                        egui::Align2::LEFT_CENTER,
                        &tab_text,
                        egui::FontId::new(12.0, egui::FontFamily::Proportional),
                        text_color,
                    );
                    
                    let close_rect = egui::Rect::from_min_size(
                        full_rect.right_top() - egui::vec2(close_width, 0.0),
                        egui::vec2(close_width, tab_height),
                    );
                    
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        let is_hovering_close = close_rect.contains(pos);
                        let x_color = if is_hovering_close {
                            egui::Color32::from_rgb(255, 235, 245)
                        } else {
                            egui::Color32::from_rgb(200, 160, 180)
                        };
                        
                        if is_active || is_hovering_close {
                            ui.painter().text(
                                close_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "×",
                                egui::FontId::new(14.0, egui::FontFamily::Proportional),
                                x_color,
                            );
                        }
                    }
                    
                    let is_hovering_close = ui.input(|i| {
                        i.pointer.hover_pos().map_or(false, |pos| close_rect.contains(pos))
                    });

                    let is_middle_click = ui.input(|i| {
                        i.pointer.button_pressed(egui::PointerButton::Middle) &&
                        i.pointer.hover_pos().map_or(false, |pos| full_rect.contains(pos))
                    });

                    if is_middle_click {
                        tab_to_remove = Some(index);
                    } else if response.clicked() && is_hovering_close {
                        tab_to_remove = Some(index);
                    } else if response.clicked() {
                        new_active_tab = index;
                    }

                    ui.add_space(2.0);
                }

                if let Some(index) = tab_to_remove {
                    self.tabs.remove(index);
                    if !self.tabs.is_empty() {
                        if active_tab_idx == index {
                            new_active_tab = active_tab_idx.min(self.tabs.len().saturating_sub(1));
                        } else if active_tab_idx > index {
                            new_active_tab = active_tab_idx.saturating_sub(1);
                        }
                        self.active_tab = new_active_tab.min(self.tabs.len().saturating_sub(1));
                    }
                }
            });
        } else {
            let tabs_info: Vec<_> = self.tabs.iter().enumerate().map(|(index, tab)| {
                let is_active = index == self.active_tab;
                let tab_text = match tab {
                    Tab::Editor(_, path, modified) => {
                        if let Some(p) = path {
                            format!("{}{}", p.file_name().unwrap_or_default().to_string_lossy(), if *modified { "*" } else { "" })
                        } else {
                            format!("Untitled{}", if *modified { "*" } else { "" })
                        }
                    }
                    Tab::Settings => "Settings".to_string(),
                };
                (index, is_active, tab_text)
            }).collect();
            let active_tab_idx = self.active_tab;
            let tab_height = 24.0;
            let tab_width = 140.0;
            let fixed_height = 32.0;
            
            egui::TopBottomPanel::top("tab_bar").resizable(false).exact_height(fixed_height).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    let mut tab_to_remove: Option<usize> = None;
                    let mut new_active_tab = active_tab_idx;
                    
                    for (index, is_active, tab_text) in tabs_info {
                        let full_tab_width = tab_width;
                        let close_width = 20.0;
                        let text_width = full_tab_width - close_width;
                        
                        let (full_rect, response) = ui.allocate_exact_size(
                            egui::vec2(full_tab_width, tab_height),
                            egui::Sense::click(),
                        );
                        
                        let bg_color = if is_active {
                            egui::Color32::from_rgb(85, 55, 70)
                        } else if response.hovered() {
                            egui::Color32::from_rgb(70, 45, 58)
                        } else {
                            egui::Color32::from_rgb(45, 28, 38)
                        };
                        
                        ui.painter().rect_filled(full_rect, 0.0, bg_color);
                        
                        if is_active {
                            let indicator_rect = egui::Rect::from_min_size(
                                full_rect.left_top(),
                                egui::vec2(full_tab_width, 2.0),
                            );
                            ui.painter().rect_filled(indicator_rect, 0.0, egui::Color32::from_rgb(255, 130, 180));
                        }
                        
                        let text_color = if is_active {
                            egui::Color32::from_rgb(255, 235, 245)
                        } else {
                            egui::Color32::from_rgb(200, 160, 180)
                        };
                        
                        let text_rect = egui::Rect::from_min_size(
                            full_rect.left_top() + egui::vec2(8.0, 0.0),
                            egui::vec2(text_width - 8.0, tab_height),
                        );
                        ui.painter().text(
                            text_rect.left_center(),
                            egui::Align2::LEFT_CENTER,
                            &tab_text,
                            egui::FontId::new(12.0, egui::FontFamily::Proportional),
                            text_color,
                        );
                        
                        let close_rect = egui::Rect::from_min_size(
                            full_rect.right_top() - egui::vec2(close_width, 0.0),
                            egui::vec2(close_width, tab_height),
                        );
                        
                        if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                            let is_hovering_close = close_rect.contains(pos);
                            let x_color = if is_hovering_close {
                                egui::Color32::from_rgb(255, 235, 245)
                            } else {
                                egui::Color32::from_rgb(200, 160, 180)
                            };
                            
                            if is_active || is_hovering_close {
                                ui.painter().text(
                                    close_rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    "×",
                                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                                    x_color,
                                );
                            }
                        }
                        
                        let is_hovering_close = ui.input(|i| {
                            i.pointer.hover_pos().map_or(false, |pos| close_rect.contains(pos))
                        });

                        let is_middle_click = ui.input(|i| {
                            i.pointer.button_pressed(egui::PointerButton::Middle) &&
                            i.pointer.hover_pos().map_or(false, |pos| full_rect.contains(pos))
                        });

                        if is_middle_click {
                            tab_to_remove = Some(index);
                        } else if response.clicked() && is_hovering_close {
                            tab_to_remove = Some(index);
                        } else if response.clicked() {
                            new_active_tab = index;
                        }

                        ui.add_space(2.0);
                    }

                    if let Some(index) = tab_to_remove {
                        self.tabs.remove(index);
                        if !self.tabs.is_empty() {
                            if active_tab_idx == index {
                                new_active_tab = active_tab_idx.min(self.tabs.len().saturating_sub(1));
                            } else if active_tab_idx > index {
                                new_active_tab = active_tab_idx.saturating_sub(1);
                            }
                            self.active_tab = new_active_tab.min(self.tabs.len().saturating_sub(1));
                        }
                    }
                });
            });
        }
    }
    
    fn open_settings_tab(&mut self) {
        if !self.tabs.iter().any(|t| matches!(t, Tab::Settings)) {
            self.tabs.push(Tab::Settings);
        }
        if let Some(index) = self.tabs.iter().position(|t| matches!(t, Tab::Settings)) {
            self.active_tab = index;
        }
    }

    fn close_current_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.tabs.remove(self.active_tab);
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }

    fn new_file(&mut self) {
        self.tabs.push(Tab::Editor(String::new(), None, false));
        self.active_tab = self.tabs.len() - 1;
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                self.tabs.push(Tab::Editor(content, Some(path), false));
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(Tab::Editor(text, file_path, modified)) = self.tabs.get_mut(self.active_tab) {
            if let Some(path) = file_path {
                if let Ok(()) = std::fs::write(path, text) {
                    *modified = false;
                }
            } else {
                self.save_file_as();
            }
        }
    }

    fn save_file_as(&mut self) {
        if let Some(Tab::Editor(text, file_path, modified)) = self.tabs.get_mut(self.active_tab) {
            if let Some(path) = rfd::FileDialog::new().save_file() {
                if let Ok(()) = std::fs::write(&path, text) {
                    *file_path = Some(path);
                    *modified = false;
                }
            }
        }
    }
    
    fn get_line_count_inner(text: &str) -> usize {
        if text.is_empty() {
            1
        } else {
            text.lines().count()
        }
    }
    
    fn get_cursor_position() -> usize {
        1
    }
    
    fn get_line_ending_type_inner(text: &str) -> String {
        if text.contains("\r\n") {
            "Windows (CRLF)".to_string()
        } else if text.contains('\r') && !text.contains('\n') {
            "Classic Mac (CR)".to_string()
        } else {
            "Unix (LF)".to_string()
        }
    }
}

use egui::{Align, Layout, RichText, Ui};

pub struct StatusBar<'a> {
    message: &'a str,
    detail: Option<&'a str>,
    mode: Option<&'a str>,
}

impl<'a> StatusBar<'a> {
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            detail: None,
            mode: None,
        }
    }

    pub fn detail(mut self, detail: &'a str) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn mode(mut self, mode: &'a str) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(self.message).small());
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some(detail) = self.detail {
                    ui.label(RichText::new(detail).small());
                }
                if let Some(mode) = self.mode {
                    ui.separator();
                    ui.label(RichText::new(mode).small());
                }
            });
        });
    }
}

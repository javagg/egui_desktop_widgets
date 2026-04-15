use crate::inputs::NumericField;
use egui::{Align, Layout, RichText, Ui};

#[derive(Clone)]
pub struct PanelHeaderAction<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub selected: bool,
}

impl<'a> PanelHeaderAction<'a> {
    pub fn new(id: &'a str, label: &'a str) -> Self {
        Self {
            id,
            label,
            selected: false,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

pub struct PanelHeader<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
    actions: Vec<PanelHeaderAction<'a>>,
}

pub struct PanelHeaderResponse {
    pub clicked_action: Option<String>,
}

impl<'a> PanelHeader<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            subtitle: None,
            actions: Vec::new(),
        }
    }

    pub fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn action(mut self, action: PanelHeaderAction<'a>) -> Self {
        self.actions.push(action);
        self
    }

    pub fn show(self, ui: &mut Ui) -> PanelHeaderResponse {
        let mut clicked_action = None;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(self.title).strong());
                if let Some(subtitle) = self.subtitle {
                    ui.label(RichText::new(subtitle).small());
                }
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                for action in self.actions.iter().rev() {
                    let clicked = ui
                        .add(
                            egui::Button::new(action.label)
                                .selected(action.selected)
                                .small(),
                        )
                        .clicked();
                    if clicked {
                        clicked_action = Some(action.id.to_owned());
                    }
                }
            });
        });

        PanelHeaderResponse { clicked_action }
    }
}

pub struct PropertyGrid<'a> {
    id_source: &'a str,
    label_width: f32,
}

pub struct PropertyGridResponse {
    pub changed: bool,
    pub last_changed: Option<String>,
}

pub struct PropertySection<'a> {
    pub title: &'a str,
    pub rows: Vec<PropertyRow<'a>>,
}

pub struct PropertyRow<'a> {
    pub label: &'a str,
    pub value: PropertyValue<'a>,
    pub hint: Option<&'a str>,
    pub enabled: bool,
}

pub enum PropertyValue<'a> {
    Text(&'a mut String),
    Bool(&'a mut bool),
    Number(NumericField<'a>),
    ReadOnly(&'a str),
}

impl<'a> PropertyGrid<'a> {
    pub fn new(id_source: &'a str) -> Self {
        Self {
            id_source,
            label_width: 112.0,
        }
    }

    pub fn label_width(mut self, label_width: f32) -> Self {
        self.label_width = label_width;
        self
    }

    pub fn show(self, ui: &mut Ui, sections: Vec<PropertySection<'a>>) -> PropertyGridResponse {
        let mut changed = false;
        let mut last_changed = None;

        ui.push_id(self.id_source, |ui| {
            for section in sections {
                ui.group(|ui| {
                    PanelHeader::new(section.title).show(ui);
                    ui.add_space(4.0);

                    for row in section.rows {
                        let PropertyRow {
                            label,
                            value,
                            hint,
                            enabled,
                        } = row;

                        let row_changed = ui
                            .add_enabled_ui(enabled, |ui| {
                                ui.horizontal(|ui| {
                                    ui.add_sized(
                                        [self.label_width, ui.spacing().interact_size.y],
                                        egui::Label::new(label),
                                    );

                                    match value {
                                        PropertyValue::Text(value) => {
                                            ui.text_edit_singleline(value).changed()
                                        }
                                        PropertyValue::Bool(value) => {
                                            ui.checkbox(value, "").changed()
                                        }
                                        PropertyValue::Number(field) => field.show(ui).changed(),
                                        PropertyValue::ReadOnly(value) => {
                                            ui.label(RichText::new(value).small());
                                            false
                                        }
                                    }
                                })
                                .inner
                            })
                            .inner;

                        if row_changed {
                            changed = true;
                            last_changed = Some(label.to_owned());
                        }

                        if let Some(hint) = hint {
                            ui.add_space(2.0);
                            ui.label(RichText::new(hint).small());
                        }

                        ui.add_space(4.0);
                    }
                });

                ui.add_space(8.0);
            }
        });

        PropertyGridResponse {
            changed,
            last_changed,
        }
    }
}

impl<'a> PropertySection<'a> {
    pub fn new(title: &'a str, rows: Vec<PropertyRow<'a>>) -> Self {
        Self { title, rows }
    }
}

impl<'a> PropertyRow<'a> {
    pub fn new(label: &'a str, value: PropertyValue<'a>) -> Self {
        Self {
            label,
            value,
            hint: None,
            enabled: true,
        }
    }

    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

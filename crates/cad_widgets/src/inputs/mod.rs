use egui::{DragValue, Response, Ui};
use std::ops::RangeInclusive;

pub struct NumericField<'a> {
    value: &'a mut f64,
    speed: f64,
    range: Option<RangeInclusive<f64>>,
    unit: Option<&'a str>,
    width: Option<f32>,
    enabled: bool,
}

impl<'a> NumericField<'a> {
    pub fn new(value: &'a mut f64) -> Self {
        Self {
            value,
            speed: 0.1,
            range: None,
            unit: None,
            width: Some(88.0),
            enabled: true,
        }
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    pub fn range(mut self, range: RangeInclusive<f64>) -> Self {
        self.range = Some(range);
        self
    }

    pub fn unit(mut self, unit: &'a str) -> Self {
        self.unit = Some(unit);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.add_enabled_ui(self.enabled, |ui| {
            ui.horizontal(|ui| {
                let mut drag_value = DragValue::new(self.value).speed(self.speed);

                if let Some(range) = self.range.clone() {
                    drag_value = drag_value.range(range);
                }

                let response = match self.width {
                    Some(width) => ui.add_sized([width, ui.spacing().interact_size.y], drag_value),
                    None => ui.add(drag_value),
                };

                if let Some(unit) = self.unit {
                    ui.label(unit);
                }

                response
            })
            .inner
        })
        .inner
    }
}

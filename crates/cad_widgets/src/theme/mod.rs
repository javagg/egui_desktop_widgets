use egui::{Color32, Context, FontFamily, FontId, Style, TextStyle, Visuals};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CadThemeMode {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug)]
pub struct CadColorTokens {
    pub app_bg: Color32,
    pub surface_bg: Color32,
    pub panel_bg: Color32,
    pub panel_fill: Color32,
    pub panel_stroke: Color32,
    pub toolbar_bg: Color32,
    pub status_bg: Color32,
    pub viewport_bg: Color32,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
}

#[derive(Clone, Copy, Debug)]
pub struct CadSpacingTokens {
    pub panel_inner_margin: i8,
    pub panel_gap: f32,
    pub compact_height: f32,
    pub regular_height: f32,
    pub toolbar_height: f32,
    pub status_height: f32,
    pub corner_radius: u8,
    pub border_width: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct CadTypographyTokens {
    pub body_size: f32,
    pub small_size: f32,
    pub heading_size: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct CadTheme {
    pub mode: CadThemeMode,
    pub colors: CadColorTokens,
    pub spacing: CadSpacingTokens,
    pub typography: CadTypographyTokens,
}

impl CadTheme {
    pub fn light() -> Self {
        Self {
            mode: CadThemeMode::Light,
            colors: CadColorTokens {
                app_bg: Color32::from_rgb(232, 235, 239),
                surface_bg: Color32::from_rgb(246, 248, 250),
                panel_bg: Color32::from_rgb(238, 241, 245),
                panel_fill: Color32::from_rgb(250, 251, 252),
                panel_stroke: Color32::from_rgb(177, 184, 192),
                toolbar_bg: Color32::from_rgb(225, 229, 234),
                status_bg: Color32::from_rgb(214, 220, 227),
                viewport_bg: Color32::from_rgb(207, 214, 222),
                accent: Color32::from_rgb(40, 98, 163),
                accent_soft: Color32::from_rgb(211, 225, 242),
                text_primary: Color32::from_rgb(33, 37, 43),
                text_muted: Color32::from_rgb(95, 104, 115),
            },
            spacing: CadSpacingTokens {
                panel_inner_margin: 8,
                panel_gap: 8.0,
                compact_height: 24.0,
                regular_height: 30.0,
                toolbar_height: 34.0,
                status_height: 26.0,
                corner_radius: 4,
                border_width: 1.0,
            },
            typography: CadTypographyTokens {
                body_size: 13.0,
                small_size: 12.0,
                heading_size: 15.0,
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: CadThemeMode::Dark,
            colors: CadColorTokens {
                app_bg: Color32::from_rgb(26, 30, 34),
                surface_bg: Color32::from_rgb(32, 36, 41),
                panel_bg: Color32::from_rgb(37, 42, 47),
                panel_fill: Color32::from_rgb(42, 47, 53),
                panel_stroke: Color32::from_rgb(76, 85, 94),
                toolbar_bg: Color32::from_rgb(34, 39, 44),
                status_bg: Color32::from_rgb(29, 34, 39),
                viewport_bg: Color32::from_rgb(21, 25, 29),
                accent: Color32::from_rgb(98, 158, 227),
                accent_soft: Color32::from_rgb(46, 73, 106),
                text_primary: Color32::from_rgb(225, 229, 234),
                text_muted: Color32::from_rgb(167, 176, 186),
            },
            spacing: CadSpacingTokens {
                panel_inner_margin: 8,
                panel_gap: 8.0,
                compact_height: 24.0,
                regular_height: 30.0,
                toolbar_height: 34.0,
                status_height: 26.0,
                corner_radius: 4,
                border_width: 1.0,
            },
            typography: CadTypographyTokens {
                body_size: 13.0,
                small_size: 12.0,
                heading_size: 15.0,
            },
        }
    }

    pub fn visuals(&self) -> Visuals {
        match self.mode {
            CadThemeMode::Light => Visuals::light(),
            CadThemeMode::Dark => Visuals::dark(),
        }
    }

    pub fn style(&self) -> Style {
        let mut style = Style {
            visuals: self.visuals(),
            ..Style::default()
        };

        style.spacing.item_spacing =
            egui::vec2(self.spacing.panel_gap, self.spacing.panel_gap * 0.75);
        style.spacing.button_padding = egui::vec2(8.0, 4.0);
        style.spacing.interact_size.y = self.spacing.regular_height;
        style.visuals.widgets.noninteractive.bg_fill = self.colors.panel_fill;
        style.visuals.widgets.noninteractive.bg_stroke.color = self.colors.panel_stroke;
        style.visuals.widgets.inactive.bg_fill = self.colors.surface_bg;
        style.visuals.widgets.inactive.bg_stroke.color = self.colors.panel_stroke;
        style.visuals.widgets.hovered.bg_fill = self.colors.accent_soft;
        style.visuals.widgets.hovered.bg_stroke.color = self.colors.accent;
        style.visuals.widgets.active.bg_fill = self.colors.accent;
        style.visuals.widgets.active.fg_stroke.color = self.colors.text_primary;
        style.visuals.window_fill = self.colors.surface_bg;
        style.visuals.panel_fill = self.colors.panel_bg;
        style.visuals.faint_bg_color = self.colors.surface_bg;
        style.visuals.extreme_bg_color = self.colors.app_bg;
        style.visuals.override_text_color = Some(self.colors.text_primary);
        style.visuals.selection.bg_fill = self.colors.accent_soft;
        style.visuals.selection.stroke.color = self.colors.accent;
        style.visuals.window_stroke.color = self.colors.panel_stroke;
        style.visuals.widgets.noninteractive.corner_radius = self.spacing.corner_radius.into();
        style.visuals.widgets.inactive.corner_radius = self.spacing.corner_radius.into();
        style.visuals.widgets.hovered.corner_radius = self.spacing.corner_radius.into();
        style.visuals.widgets.active.corner_radius = self.spacing.corner_radius.into();
        style.visuals.widgets.open.corner_radius = self.spacing.corner_radius.into();

        style.text_styles = [
            (
                TextStyle::Body,
                FontId::new(self.typography.body_size, FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                FontId::new(self.typography.body_size, FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(self.typography.small_size, FontFamily::Proportional),
            ),
            (
                TextStyle::Heading,
                FontId::new(self.typography.heading_size, FontFamily::Proportional),
            ),
        ]
        .into();

        style
    }
}

pub fn apply_theme(ctx: &Context, theme: &CadTheme) {
    ctx.set_style(theme.style());
}

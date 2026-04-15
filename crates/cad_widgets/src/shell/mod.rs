use crate::feedback::StatusBar;
use crate::panels::{PanelHeader, PanelHeaderAction};
use crate::theme::CadTheme;
use egui::{Align, CentralPanel, Context, Frame, Layout, SidePanel, TopBottomPanel, Ui};

#[derive(Clone, Debug)]
pub struct ToolbarAction<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub selected: bool,
    pub enabled: bool,
}

impl<'a> ToolbarAction<'a> {
    pub fn new(id: &'a str, label: &'a str) -> Self {
        Self {
            id,
            label,
            selected: false,
            enabled: true,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

pub struct Toolbar<'a> {
    id_source: &'a str,
    actions: &'a [ToolbarAction<'a>],
}

#[derive(Default)]
pub struct ToolbarResponse {
    pub clicked: Option<String>,
}

impl<'a> Toolbar<'a> {
    pub fn new(id_source: &'a str, actions: &'a [ToolbarAction<'a>]) -> Self {
        Self { id_source, actions }
    }

    pub fn show(self, ui: &mut Ui) -> ToolbarResponse {
        let mut response = ToolbarResponse::default();

        ui.push_id(self.id_source, |ui| {
            ui.horizontal_wrapped(|ui| {
                for action in self.actions {
                    let clicked = ui
                        .add_enabled(
                            action.enabled,
                            egui::Button::new(action.label).selected(action.selected),
                        )
                        .clicked();

                    if clicked {
                        response.clicked = Some(action.id.to_owned());
                    }
                }
            });
        });

        response
    }
}

#[derive(Clone, Debug)]
pub struct ShellPanelState {
    pub title: String,
    pub open: bool,
    pub pinned: bool,
    pub width: f32,
}

impl ShellPanelState {
    pub fn new(title: impl Into<String>, width: f32) -> Self {
        Self {
            title: title.into(),
            open: true,
            pinned: true,
            width,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShellBandState {
    pub title: String,
    pub open: bool,
    pub pinned: bool,
    pub height: f32,
}

impl ShellBandState {
    pub fn new(title: impl Into<String>, height: f32) -> Self {
        Self {
            title: title.into(),
            open: true,
            pinned: true,
            height,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShellLayoutState {
    pub top: ShellBandState,
    pub left: ShellPanelState,
    pub right: ShellPanelState,
    pub bottom: ShellBandState,
    pub status_message: String,
    pub status_detail: String,
}

impl Default for ShellLayoutState {
    fn default() -> Self {
        Self {
            top: ShellBandState::new("Tool Options", 72.0),
            left: ShellPanelState::new("Model", 260.0),
            right: ShellPanelState::new("Properties", 300.0),
            bottom: ShellBandState::new("Messages", 160.0),
            status_message: "Ready".to_owned(),
            status_detail: "100% | mm".to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShellFrameConfig {
    pub app_title: String,
    pub menu_height: f32,
    pub toolbar_height: f32,
    pub collapsed_strip_size: f32,
}

impl Default for ShellFrameConfig {
    fn default() -> Self {
        Self {
            app_title: "CAD Workspace".to_owned(),
            menu_height: 28.0,
            toolbar_height: 34.0,
            collapsed_strip_size: 22.0,
        }
    }
}

pub struct ShellFrame<'a> {
    pub theme: &'a CadTheme,
    pub config: &'a ShellFrameConfig,
    pub state: &'a mut ShellLayoutState,
}

impl<'a> ShellFrame<'a> {
    pub fn new(
        theme: &'a CadTheme,
        config: &'a ShellFrameConfig,
        state: &'a mut ShellLayoutState,
    ) -> Self {
        Self {
            theme,
            config,
            state,
        }
    }

    pub fn show<MenuBar, Toolbar, TopPanel, LeftPanel, Center, RightPanel, Bottom>(
        self,
        ctx: &Context,
        menu_bar: MenuBar,
        toolbar: Toolbar,
        top_panel: TopPanel,
        left_panel: LeftPanel,
        center: Center,
        right_panel: RightPanel,
        bottom: Bottom,
    ) where
        MenuBar: FnOnce(&mut Ui, &mut ShellLayoutState),
        Toolbar: FnOnce(&mut Ui, &mut ShellLayoutState),
        TopPanel: FnOnce(&mut Ui, &mut ShellLayoutState),
        LeftPanel: FnOnce(&mut Ui, &mut ShellLayoutState),
        Center: FnOnce(&mut Ui, &mut ShellLayoutState),
        RightPanel: FnOnce(&mut Ui, &mut ShellLayoutState),
        Bottom: FnOnce(&mut Ui, &mut ShellLayoutState),
    {
        let theme = self.theme;
        let config = self.config;
        let state = self.state;

        TopBottomPanel::top("shell_menu_bar")
            .exact_height(config.menu_height)
            .frame(panel_frame(
                theme.colors.toolbar_bg,
                theme.colors.panel_stroke,
            ))
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.strong(&config.app_title);
                    ui.separator();
                    menu_bar(ui, state);
                });
            });

        TopBottomPanel::top("shell_toolbar")
            .exact_height(config.toolbar_height)
            .frame(panel_frame(
                theme.colors.toolbar_bg,
                theme.colors.panel_stroke,
            ))
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    toolbar(ui, state);
                });
            });

        if state.top.open {
            TopBottomPanel::top("shell_top_panel")
                .resizable(true)
                .default_height(state.top.height)
                .min_height(52.0)
                .frame(panel_frame(
                    theme.colors.panel_bg,
                    theme.colors.panel_stroke,
                ))
                .show(ctx, |ui| {
                    let title = state.top.title.clone();
                    let response = PanelHeader::new(&title)
                        .subtitle("Docked top panel")
                        .action(
                            PanelHeaderAction::new(
                                "pin",
                                if state.top.pinned { "[P]" } else { "[ ]" },
                            )
                            .selected(state.top.pinned),
                        )
                        .action(PanelHeaderAction::new("collapse", "▴"))
                        .show(ui);
                    match response.clicked_action.as_deref() {
                        Some("pin") => state.top.pinned = !state.top.pinned,
                        Some("collapse") => state.top.open = false,
                        _ => {}
                    }
                    ui.separator();
                    top_panel(ui, state);
                });
        } else {
            TopBottomPanel::top("shell_top_restore")
                .exact_height(config.collapsed_strip_size)
                .frame(panel_frame(
                    theme.colors.panel_bg,
                    theme.colors.panel_stroke,
                ))
                .show(ctx, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        if ui.small_button("▾ Tool Options").clicked() {
                            state.top.open = true;
                        }
                    });
                });
        }

        TopBottomPanel::bottom("shell_status_bar")
            .exact_height(theme.spacing.status_height)
            .frame(panel_frame(
                theme.colors.status_bg,
                theme.colors.panel_stroke,
            ))
            .show(ctx, |ui| {
                StatusBar::new(&state.status_message)
                    .detail(&state.status_detail)
                    .show(ui);
            });

        if state.bottom.open {
            TopBottomPanel::bottom("shell_bottom_panel")
                .resizable(true)
                .default_height(state.bottom.height)
                .min_height(120.0)
                .frame(panel_frame(
                    theme.colors.panel_bg,
                    theme.colors.panel_stroke,
                ))
                .show(ctx, |ui| {
                    let title = state.bottom.title.clone();
                    let response = PanelHeader::new(&title)
                        .subtitle("Logs, command feedback, validation")
                        .action(
                            PanelHeaderAction::new(
                                "pin",
                                if state.bottom.pinned { "[P]" } else { "[ ]" },
                            )
                            .selected(state.bottom.pinned),
                        )
                        .action(PanelHeaderAction::new("collapse", "▾"))
                        .show(ui);
                    match response.clicked_action.as_deref() {
                        Some("pin") => state.bottom.pinned = !state.bottom.pinned,
                        Some("collapse") => state.bottom.open = false,
                        _ => {}
                    }
                    ui.separator();
                    bottom(ui, state);
                });
        } else {
            TopBottomPanel::bottom("shell_bottom_restore")
                .exact_height(config.collapsed_strip_size)
                .frame(panel_frame(
                    theme.colors.panel_bg,
                    theme.colors.panel_stroke,
                ))
                .show(ctx, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        if ui.small_button("▴ Messages").clicked() {
                            state.bottom.open = true;
                        }
                    });
                });
        }

        if state.left.open {
            SidePanel::left("shell_left_panel")
                .resizable(true)
                .default_width(state.left.width)
                .min_width(180.0)
                .frame(panel_frame(
                    theme.colors.panel_bg,
                    theme.colors.panel_stroke,
                ))
                .show(ctx, |ui| {
                    let title = state.left.title.clone();
                    let response = PanelHeader::new(&title)
                        .action(
                            PanelHeaderAction::new(
                                "pin",
                                if state.left.pinned { "[P]" } else { "[ ]" },
                            )
                            .selected(state.left.pinned),
                        )
                        .action(PanelHeaderAction::new("collapse", "◂"))
                        .show(ui);
                    match response.clicked_action.as_deref() {
                        Some("pin") => state.left.pinned = !state.left.pinned,
                        Some("collapse") => state.left.open = false,
                        _ => {}
                    }
                    ui.separator();
                    left_panel(ui, state);
                });
        } else {
            SidePanel::left("shell_left_restore")
                .exact_width(config.collapsed_strip_size)
                .frame(panel_frame(
                    theme.colors.panel_bg,
                    theme.colors.panel_stroke,
                ))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        if ui.small_button("▸").clicked() {
                            state.left.open = true;
                        }
                    });
                });
        }

        if state.right.open {
            SidePanel::right("shell_right_panel")
                .resizable(true)
                .default_width(state.right.width)
                .min_width(220.0)
                .frame(panel_frame(
                    theme.colors.panel_bg,
                    theme.colors.panel_stroke,
                ))
                .show(ctx, |ui| {
                    let title = state.right.title.clone();
                    let response = PanelHeader::new(&title)
                        .action(
                            PanelHeaderAction::new(
                                "pin",
                                if state.right.pinned { "[P]" } else { "[ ]" },
                            )
                            .selected(state.right.pinned),
                        )
                        .action(PanelHeaderAction::new("collapse", "▸"))
                        .show(ui);
                    match response.clicked_action.as_deref() {
                        Some("pin") => state.right.pinned = !state.right.pinned,
                        Some("collapse") => state.right.open = false,
                        _ => {}
                    }
                    ui.separator();
                    right_panel(ui, state);
                });
        } else {
            SidePanel::right("shell_right_restore")
                .exact_width(config.collapsed_strip_size)
                .frame(panel_frame(
                    theme.colors.panel_bg,
                    theme.colors.panel_stroke,
                ))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        if ui.small_button("◂").clicked() {
                            state.right.open = true;
                        }
                    });
                });
        }

        CentralPanel::default()
            .frame(Frame::default().fill(theme.colors.viewport_bg))
            .show(ctx, |ui| {
                center(ui, state);
            });
    }
}

fn panel_frame(fill: egui::Color32, stroke: egui::Color32) -> Frame {
    Frame::default()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
        .inner_margin(8)
}

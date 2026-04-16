use cad_widgets::{
    CadTheme, CadThemeMode, DemoDockArea, DemoDockDragPayload, DemoDockDropSlot,
    DemoDockLayout, DemoDockSnapshot, DocumentKind,
    DocumentWorkspaceBuilder, DocumentWorkspaceLayout, DocumentWorkspaceNode,
    DocumentWorkspaceSnapshot, ShellFrame, ShellFrameConfig, ShellLayoutState, Toolbar,
    ToolbarAction, WorkspaceDocument, apply_theme,
};
use eframe::egui::{self, Align, Layout, RichText};
use std::cell::RefCell;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1440.0, 900.0])
            .with_title("CAD Widgets Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "CAD Widgets Demo",
        native_options,
        Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
    )
}

struct DemoApp {
    theme_mode: CadThemeMode,
    shell_config: ShellFrameConfig,
    shell_state: ShellLayoutState,
    active_tool: String,
    selected_object_name: String,
    top_dock_layout: DemoDockLayout,
    left_dock_layout: DemoDockLayout,
    center_workspace_layout: DocumentWorkspaceLayout,
    saved_workspace_snapshot: Option<DocumentWorkspaceSnapshot>,
    saved_dock_snapshot: Option<DockHostSnapshotBundle>,
    right_dock_layout: DemoDockLayout,
    bottom_dock_layout: DemoDockLayout,
    message_log: Vec<String>,
}

#[derive(Clone)]
struct DockHostSnapshotBundle {
    top: DemoDockSnapshot,
    left: DemoDockSnapshot,
    right: DemoDockSnapshot,
    bottom: DemoDockSnapshot,
}

impl DemoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            theme_mode: CadThemeMode::Light,
            shell_config: ShellFrameConfig::default(),
            shell_state: ShellLayoutState::default(),
            active_tool: "Select".to_owned(),
            selected_object_name: "Body-01".to_owned(),
            top_dock_layout: DemoDockLayout::for_area(DemoDockArea::Top),
            left_dock_layout: DemoDockLayout::for_area(DemoDockArea::Left),
            center_workspace_layout: build_center_workspace(),
            saved_workspace_snapshot: None,
            saved_dock_snapshot: None,
            right_dock_layout: DemoDockLayout::for_area(DemoDockArea::Right),
            bottom_dock_layout: DemoDockLayout::for_area(DemoDockArea::Bottom),
            message_log: vec![
                "Workspace initialized".to_owned(),
                "Shell layout demo ready".to_owned(),
                "Central workspace now supports egui_tiles tabs and splits".to_owned(),
                "Dock regions are now rendered by egui_tiles".to_owned(),
            ],
        }
    }

    fn theme(&self) -> CadTheme {
        match self.theme_mode {
            CadThemeMode::Light => CadTheme::light(),
            CadThemeMode::Dark => CadTheme::dark(),
        }
    }
}

fn push_message(log: &RefCell<Vec<String>>, message: impl Into<String>) {
    let mut log = log.borrow_mut();
    log.push(message.into());
    if log.len() > 12 {
        let drain_count = log.len() - 12;
        log.drain(0..drain_count);
    }
}

fn transfer_dock_pane(
    source: &RefCell<DemoDockLayout>,
    target: &RefCell<DemoDockLayout>,
    title: &str,
    slot: DemoDockDropSlot,
) -> bool {
    let Some(pane) = source.borrow_mut().transfer_pane(title) else {
        return false;
    };
    target.borrow_mut().receive_pane_at(pane, slot);
    true
}

fn build_center_workspace() -> DocumentWorkspaceLayout {
    DocumentWorkspaceBuilder::new(
        "center_document_tree",
        DocumentWorkspaceNode::vertical(vec![
            DocumentWorkspaceNode::horizontal(vec![
                DocumentWorkspaceNode::single_tab("doc:model_3d"),
                DocumentWorkspaceNode::tabs(vec![
                    DocumentWorkspaceNode::document("doc:force_plot"),
                    DocumentWorkspaceNode::document("doc:bom_table"),
                    DocumentWorkspaceNode::document("doc:inspection_report"),
                ]),
            ]),
            DocumentWorkspaceNode::single_tab("doc:notes"),
        ]),
    )
    .documents([
        WorkspaceDocument::new(
            "doc:model_3d",
            "Chassis-3D",
            DocumentKind::Model3d,
            "Primary modeling viewport with 3D interaction.",
            vec![
                "Viewport: Perspective".to_owned(),
                "Selection: Body-01".to_owned(),
                "Gizmo: Enabled".to_owned(),
            ],
            egui::Color32::from_rgb(77, 126, 189),
        )
        .standalone()
        .dirty(true)
        .closable(false),
        WorkspaceDocument::new(
            "doc:force_plot",
            "Force Plot",
            DocumentKind::Plot2d,
            "Analysis plot view grouped as document tabs.",
            vec![
                "Series: Axial Force".to_owned(),
                "Cursor: x=42.0, y=18.7".to_owned(),
                "Legend: Visible".to_owned(),
            ],
            egui::Color32::from_rgb(82, 161, 125),
        ),
        WorkspaceDocument::new(
            "doc:bom_table",
            "BOM Table",
            DocumentKind::Table,
            "Bill of materials and parameter grid.",
            vec![
                "Rows: 18".to_owned(),
                "Columns: Item / Qty / Material".to_owned(),
                "Filter: Active".to_owned(),
            ],
            egui::Color32::from_rgb(190, 132, 76),
        ),
        WorkspaceDocument::new(
            "doc:inspection_report",
            "Inspection Report",
            DocumentKind::Report,
            "Review notes, inspection checklist and snapshots.",
            vec![
                "Sections: 4".to_owned(),
                "Checklist: 9 / 12 completed".to_owned(),
                "Attachments: 3".to_owned(),
            ],
            egui::Color32::from_rgb(146, 101, 170),
        )
        .dirty(true),
        WorkspaceDocument::new(
            "doc:notes",
            "Notes",
            DocumentKind::Report,
            "Supplemental design notes in a secondary split.",
            vec![
                "TODO: Review mounting holes".to_owned(),
                "Pending: tolerance verification".to_owned(),
            ],
            egui::Color32::from_rgb(116, 121, 138),
        )
        .standalone(),
    ])
    .build()
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = self.theme();
        apply_theme(ctx, &theme);

        let theme_mode = RefCell::new(self.theme_mode);
        let active_tool = RefCell::new(self.active_tool.clone());
        let selected_object_name = RefCell::new(self.selected_object_name.clone());
        let message_log = RefCell::new(self.message_log.clone());
        let saved_workspace_snapshot = RefCell::new(self.saved_workspace_snapshot.clone());
        let saved_dock_snapshot = RefCell::new(self.saved_dock_snapshot.clone());
        let pending_dock_drop =
            RefCell::new(None::<(DemoDockDragPayload, DemoDockArea, DemoDockDropSlot)>);
        let top_dock_layout = RefCell::new(std::mem::replace(
            &mut self.top_dock_layout,
            DemoDockLayout::for_area(DemoDockArea::Top),
        ));
        let left_dock_layout = RefCell::new(std::mem::replace(
            &mut self.left_dock_layout,
            DemoDockLayout::for_area(DemoDockArea::Left),
        ));
        let center_workspace_layout = RefCell::new(std::mem::replace(
            &mut self.center_workspace_layout,
            build_center_workspace(),
        ));
        let right_dock_layout = RefCell::new(std::mem::replace(
            &mut self.right_dock_layout,
            DemoDockLayout::for_area(DemoDockArea::Right),
        ));
        let bottom_dock_layout = RefCell::new(std::mem::replace(
            &mut self.bottom_dock_layout,
            DemoDockLayout::for_area(DemoDockArea::Bottom),
        ));

        let toolbar_actions = [
            ToolbarAction::new("select", "Select").selected(self.active_tool == "Select"),
            ToolbarAction::new("sketch", "Sketch").selected(self.active_tool == "Sketch"),
            ToolbarAction::new("extrude", "Extrude").selected(self.active_tool == "Extrude"),
            ToolbarAction::new("measure", "Measure").selected(self.active_tool == "Measure"),
        ];

        ShellFrame::new(&theme, &self.shell_config, &mut self.shell_state).show(
            ctx,
            |ui, state| {
                if ui.button("File").clicked() {
                    state.status_message = "File menu placeholder".to_owned();
                }
                if ui.button("Edit").clicked() {
                    state.status_message = "Edit menu placeholder".to_owned();
                }
                if ui.button("View").clicked() {
                    state.status_message = "View menu placeholder".to_owned();
                }
            },
            |ui, state| {
                let toolbar_response = Toolbar::new("main_toolbar", &toolbar_actions).show(ui);
                if let Some(action_id) = toolbar_response.clicked {
                    *active_tool.borrow_mut() = match action_id.as_str() {
                        "select" => "Select",
                        "sketch" => "Sketch",
                        "extrude" => "Extrude",
                        "measure" => "Measure",
                        _ => "Select",
                    }
                    .to_owned();
                    state.status_message = format!("{} tool activated", active_tool.borrow());
                    push_message(&message_log, format!("Toolbar command: {}", active_tool.borrow()));
                }
            },
            |ui, state| {
                let dock_response = top_dock_layout.borrow_mut().show(ui);
                if let Some((payload, slot)) = dock_response.dropped_payload {
                    *pending_dock_drop.borrow_mut() = Some((payload, DemoDockArea::Top, slot));
                }
                state.status_message = format!("Top dock host active for {}", active_tool.borrow());
            },
            |ui, state| {
                let dock_response = left_dock_layout.borrow_mut().show(ui);
                if let Some((payload, slot)) = dock_response.dropped_payload {
                    *pending_dock_drop.borrow_mut() = Some((payload, DemoDockArea::Left, slot));
                }
                state.status_message = "Left dock host rendered with tabs and vertical split".to_owned();
            },
            |ui, state| {
                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    let workspace_height = (ui.available_height() - 88.0).max(260.0);
                    let closed_documents = center_workspace_layout.borrow().closed_documents();
                    let left_closed = left_dock_layout.borrow().closed_panes();
                    let top_closed = top_dock_layout.borrow().closed_panes();
                    let right_closed = right_dock_layout.borrow().closed_panes();
                    let bottom_closed = bottom_dock_layout.borrow().closed_panes();
                    let left_visible = left_dock_layout.borrow().visible_panes();
                    let top_visible = top_dock_layout.borrow().visible_panes();
                    let right_visible = right_dock_layout.borrow().visible_panes();
                    let bottom_visible = bottom_dock_layout.borrow().visible_panes();

                    ui.heading("Central Document Workspace");
                    ui.label("中央文档区当前由 builder 描述初始 pane tree，并支持快照恢复；四边 dock 区提供一致的关闭后恢复入口。dock pane 现在可以直接拖放到其他 dock host，中央 document pane 仍然不参与跨区移动。");
                    ui.add_space(8.0);

                    ui.group(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("Save Dock Host Snapshot").clicked() {
                                *saved_dock_snapshot.borrow_mut() = Some(DockHostSnapshotBundle {
                                    top: top_dock_layout.borrow().snapshot(),
                                    left: left_dock_layout.borrow().snapshot(),
                                    right: right_dock_layout.borrow().snapshot(),
                                    bottom: bottom_dock_layout.borrow().snapshot(),
                                });
                                state.status_message = "Dock host snapshot saved".to_owned();
                                push_message(&message_log, "Saved dock host snapshot bundle");
                            }

                            if ui
                                .add_enabled(
                                    saved_dock_snapshot.borrow().is_some(),
                                    egui::Button::new("Restore Dock Host Snapshot"),
                                )
                                .clicked()
                            {
                                if let Some(snapshot) = saved_dock_snapshot.borrow().clone() {
                                    top_dock_layout.borrow_mut().restore(snapshot.top);
                                    left_dock_layout.borrow_mut().restore(snapshot.left);
                                    right_dock_layout.borrow_mut().restore(snapshot.right);
                                    bottom_dock_layout.borrow_mut().restore(snapshot.bottom);
                                    state.status_message = "Dock host snapshot restored".to_owned();
                                    push_message(
                                        &message_log,
                                        "Restored dock host snapshot bundle",
                                    );
                                }
                            }

                            if ui.button("Save Workspace Snapshot").clicked() {
                                *saved_workspace_snapshot.borrow_mut() =
                                    Some(center_workspace_layout.borrow().snapshot());
                                state.status_message = "Workspace snapshot saved".to_owned();
                                push_message(&message_log, "Saved central workspace snapshot");
                            }

                            if ui
                                .add_enabled(
                                    saved_workspace_snapshot.borrow().is_some(),
                                    egui::Button::new("Restore Workspace Snapshot"),
                                )
                                .clicked()
                            {
                                if let Some(snapshot) = saved_workspace_snapshot.borrow().clone() {
                                    center_workspace_layout.borrow_mut().restore(snapshot);
                                    state.status_message = "Workspace snapshot restored".to_owned();
                                    push_message(
                                        &message_log,
                                        "Restored central workspace snapshot",
                                    );
                                }
                            }

                            if ui
                                .add_enabled(
                                    !closed_documents.is_empty(),
                                    egui::Button::new("Restore All Closed Documents"),
                                )
                                .clicked()
                            {
                                    center_workspace_layout.borrow_mut().restore_all_documents();
                                state.status_message = "Closed documents restored".to_owned();
                                push_message(&message_log, "Restored all closed documents");
                            }
                        });

                        if !closed_documents.is_empty() {
                            ui.horizontal_wrapped(|ui| {
                                ui.label("Closed documents:");
                                for document in closed_documents {
                                    if ui
                                        .small_button(format!("Restore {}", document.title))
                                        .clicked()
                                    {
                                        center_workspace_layout
                                            .borrow_mut()
                                            .restore_document(&document.id);
                                        state.status_message =
                                            format!("Restored document {}", document.title);
                                        push_message(
                                            &message_log,
                                            format!("Restored document {}", document.title),
                                        );
                                    }
                                }
                            });
                        }

                        ui.horizontal_wrapped(|ui| {
                            ui.label("Closed dock panes:");

                            if ui
                                .add_enabled(!left_closed.is_empty(), egui::Button::new("Restore Left"))
                                .clicked()
                            {
                                if let Some(title) = left_dock_layout.borrow_mut().restore_last_closed() {
                                    push_message(&message_log, format!("Restored left pane {title}"));
                                }
                            }
                            if ui
                                .add_enabled(!top_closed.is_empty(), egui::Button::new("Restore Top"))
                                .clicked()
                            {
                                if let Some(title) = top_dock_layout.borrow_mut().restore_last_closed() {
                                    push_message(&message_log, format!("Restored top pane {title}"));
                                }
                            }
                            if ui
                                .add_enabled(!right_closed.is_empty(), egui::Button::new("Restore Right"))
                                .clicked()
                            {
                                if let Some(title) = right_dock_layout.borrow_mut().restore_last_closed() {
                                    push_message(&message_log, format!("Restored right pane {title}"));
                                }
                            }
                            if ui
                                .add_enabled(!bottom_closed.is_empty(), egui::Button::new("Restore Bottom"))
                                .clicked()
                            {
                                if let Some(title) = bottom_dock_layout.borrow_mut().restore_last_closed() {
                                    push_message(&message_log, format!("Restored bottom pane {title}"));
                                }
                            }

                            if ui
                                .add_enabled(
                                    !left_closed.is_empty()
                                        || !top_closed.is_empty()
                                        || !right_closed.is_empty()
                                        || !bottom_closed.is_empty(),
                                    egui::Button::new("Restore All Dock Panes"),
                                )
                                .clicked()
                            {
                                left_dock_layout.borrow_mut().restore_all_closed();
                                top_dock_layout.borrow_mut().restore_all_closed();
                                right_dock_layout.borrow_mut().restore_all_closed();
                                bottom_dock_layout.borrow_mut().restore_all_closed();
                                push_message(&message_log, "Restored all closed dock panes");
                            }
                        });

                        ui.separator();
                        ui.label(RichText::new("Dock Host Transfers").strong());

                        render_dock_transfer_row(
                            ui,
                            "Left",
                            &left_visible,
                            ("Top", &top_dock_layout),
                            ("Right", &right_dock_layout),
                            ("Bottom", &bottom_dock_layout),
                            &left_dock_layout,
                            &message_log,
                            state,
                        );
                        render_dock_transfer_row(
                            ui,
                            "Top",
                            &top_visible,
                            ("Left", &left_dock_layout),
                            ("Right", &right_dock_layout),
                            ("Bottom", &bottom_dock_layout),
                            &top_dock_layout,
                            &message_log,
                            state,
                        );
                        render_dock_transfer_row(
                            ui,
                            "Right",
                            &right_visible,
                            ("Left", &left_dock_layout),
                            ("Top", &top_dock_layout),
                            ("Bottom", &bottom_dock_layout),
                            &right_dock_layout,
                            &message_log,
                            state,
                        );
                        render_dock_transfer_row(
                            ui,
                            "Bottom",
                            &bottom_visible,
                            ("Left", &left_dock_layout),
                            ("Top", &top_dock_layout),
                            ("Right", &right_dock_layout),
                            &bottom_dock_layout,
                            &message_log,
                            state,
                        );
                    });

                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), workspace_height),
                        Layout::top_down(Align::Min),
                        |ui| {
                            center_workspace_layout.borrow_mut().show(ui);
                        },
                    );

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.label("Theme:");
                        if ui
                            .selectable_label(matches!(*theme_mode.borrow(), CadThemeMode::Light), "Light")
                            .clicked()
                        {
                            *theme_mode.borrow_mut() = CadThemeMode::Light;
                            state.status_message = "Switched to light theme".to_owned();
                            push_message(&message_log, "Theme switched to light");
                        }
                        if ui
                            .selectable_label(matches!(*theme_mode.borrow(), CadThemeMode::Dark), "Dark")
                            .clicked()
                        {
                            *theme_mode.borrow_mut() = CadThemeMode::Dark;
                            state.status_message = "Switched to dark theme".to_owned();
                            push_message(&message_log, "Theme switched to dark");
                        }
                    });

                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Panels:");
                        if ui
                            .selectable_label(state.left.open, "Left")
                            .clicked()
                        {
                            state.left.open = !state.left.open;
                        }
                        if ui
                            .selectable_label(state.top.open, "Top")
                            .clicked()
                        {
                            state.top.open = !state.top.open;
                        }
                        if ui
                            .selectable_label(state.right.open, "Right")
                            .clicked()
                        {
                            state.right.open = !state.right.open;
                        }
                        if ui
                            .selectable_label(state.bottom.open, "Bottom")
                            .clicked()
                        {
                            state.bottom.open = !state.bottom.open;
                        }
                    });
                });
            },
            |ui, state| {
                let dock_response = right_dock_layout.borrow_mut().show(ui);
                if let Some((payload, slot)) = dock_response.dropped_payload {
                    *pending_dock_drop.borrow_mut() = Some((payload, DemoDockArea::Right, slot));
                }
                state.status_message = "Right dock host rendered with tabbed and split panes".to_owned();
            },
            |ui, _state| {
                let dock_response = bottom_dock_layout.borrow_mut().show(ui);
                if let Some((payload, slot)) = dock_response.dropped_payload {
                    *pending_dock_drop.borrow_mut() = Some((payload, DemoDockArea::Bottom, slot));
                }
                ui.separator();
                ui.label(RichText::new("Recent Shell Events").strong());
                for entry in message_log.borrow().iter().rev().take(4) {
                    ui.label(format!("- {entry}"));
                }
            },
        );

        if let Some((payload, target_area, target_slot)) = pending_dock_drop.into_inner() {
            if payload.source_area != target_area {
                let moved = match (payload.source_area, target_area) {
                    (DemoDockArea::Top, DemoDockArea::Left) => {
                        transfer_dock_pane(
                            &top_dock_layout,
                            &left_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Top, DemoDockArea::Right) => {
                        transfer_dock_pane(
                            &top_dock_layout,
                            &right_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Top, DemoDockArea::Bottom) => {
                        transfer_dock_pane(
                            &top_dock_layout,
                            &bottom_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Left, DemoDockArea::Top) => {
                        transfer_dock_pane(
                            &left_dock_layout,
                            &top_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Left, DemoDockArea::Right) => {
                        transfer_dock_pane(
                            &left_dock_layout,
                            &right_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Left, DemoDockArea::Bottom) => {
                        transfer_dock_pane(
                            &left_dock_layout,
                            &bottom_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Right, DemoDockArea::Top) => {
                        transfer_dock_pane(
                            &right_dock_layout,
                            &top_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Right, DemoDockArea::Left) => {
                        transfer_dock_pane(
                            &right_dock_layout,
                            &left_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Right, DemoDockArea::Bottom) => {
                        transfer_dock_pane(
                            &right_dock_layout,
                            &bottom_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Bottom, DemoDockArea::Top) => {
                        transfer_dock_pane(
                            &bottom_dock_layout,
                            &top_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Bottom, DemoDockArea::Left) => {
                        transfer_dock_pane(
                            &bottom_dock_layout,
                            &left_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    (DemoDockArea::Bottom, DemoDockArea::Right) => {
                        transfer_dock_pane(
                            &bottom_dock_layout,
                            &right_dock_layout,
                            &payload.title,
                            target_slot,
                        )
                    }
                    _ => false,
                };

                if moved {
                    self.shell_state.status_message = format!(
                        "Moved {} from {:?} to {:?}",
                        payload.title, payload.source_area, target_area
                    );
                    push_message(&message_log, format!(
                        "Dragged dock pane {} from {:?} to {:?}",
                        payload.title, payload.source_area, target_area
                    ));
                }
            }
        }

        self.theme_mode = *theme_mode.borrow();
        self.active_tool = active_tool.into_inner();
        self.selected_object_name = selected_object_name.into_inner();
        self.top_dock_layout = top_dock_layout.into_inner();
        self.left_dock_layout = left_dock_layout.into_inner();
        self.center_workspace_layout = center_workspace_layout.into_inner();
        self.saved_workspace_snapshot = saved_workspace_snapshot.into_inner();
        self.saved_dock_snapshot = saved_dock_snapshot.into_inner();
        self.right_dock_layout = right_dock_layout.into_inner();
        self.bottom_dock_layout = bottom_dock_layout.into_inner();
        self.message_log = message_log.into_inner();
    }
}

fn render_dock_transfer_row(
    ui: &mut egui::Ui,
    source_name: &str,
    pane_titles: &[String],
    target_a: (&str, &RefCell<DemoDockLayout>),
    target_b: (&str, &RefCell<DemoDockLayout>),
    target_c: (&str, &RefCell<DemoDockLayout>),
    source_layout: &RefCell<DemoDockLayout>,
    message_log: &RefCell<Vec<String>>,
    state: &mut ShellLayoutState,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(format!("{source_name}:"));
        for title in pane_titles {
            if ui.small_button(format!("{title} -> {}", target_a.0)).clicked()
                && transfer_dock_pane(source_layout, target_a.1, title, DemoDockDropSlot::Center)
            {
                state.status_message = format!("Moved {title} from {source_name} to {}", target_a.0);
                push_message(
                    message_log,
                    format!("Moved dock pane {title} from {source_name} to {}", target_a.0),
                );
            }
            if ui.small_button(format!("{title} -> {}", target_b.0)).clicked()
                && transfer_dock_pane(source_layout, target_b.1, title, DemoDockDropSlot::Center)
            {
                state.status_message = format!("Moved {title} from {source_name} to {}", target_b.0);
                push_message(
                    message_log,
                    format!("Moved dock pane {title} from {source_name} to {}", target_b.0),
                );
            }
            if ui.small_button(format!("{title} -> {}", target_c.0)).clicked()
                && transfer_dock_pane(source_layout, target_c.1, title, DemoDockDropSlot::Center)
            {
                state.status_message = format!("Moved {title} from {source_name} to {}", target_c.0);
                push_message(
                    message_log,
                    format!("Moved dock pane {title} from {source_name} to {}", target_c.0),
                );
            }
        }
    });
}

use egui::{Color32, RichText, Sense, Ui, WidgetText};
use egui_tiles::{Behavior, SimplificationOptions, TileId, Tiles, Tree, UiResponse};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaneHeaderStyle {
    Tabbed,
    Standalone,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DemoDockArea {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DemoDockDragPayload {
    pub source_area: DemoDockArea,
    pub title: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DemoDockDropSlot {
    Center,
    LeftEdge,
    RightEdge,
    TopEdge,
    BottomEdge,
}

#[derive(Default)]
pub struct DemoDockShowResponse {
    pub dropped_payload: Option<(DemoDockDragPayload, DemoDockDropSlot)>,
}

#[derive(Clone, Debug)]
pub struct DemoDockContent {
    pub title: String,
    pub badge: String,
    pub summary: String,
    pub lines: Vec<String>,
    pub accent: Color32,
    pub header_style: PaneHeaderStyle,
    pub pinned: bool,
    pub closable: bool,
}

impl DemoDockContent {
    pub fn new(
        title: impl Into<String>,
        badge: impl Into<String>,
        summary: impl Into<String>,
        lines: Vec<String>,
        accent: Color32,
    ) -> Self {
        Self {
            title: title.into(),
            badge: badge.into(),
            summary: summary.into(),
            lines,
            accent,
            header_style: PaneHeaderStyle::Standalone,
            pinned: true,
            closable: true,
        }
    }

    pub fn tabbed(mut self) -> Self {
        self.header_style = PaneHeaderStyle::Tabbed;
        self
    }

    pub fn standalone(mut self) -> Self {
        self.header_style = PaneHeaderStyle::Standalone;
        self
    }

    pub fn pinned(mut self, pinned: bool) -> Self {
        self.pinned = pinned;
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

#[derive(Clone)]
pub struct DemoDockLayout {
    area: DemoDockArea,
    tree: Tree<DemoDockContent>,
    pane_catalog: Vec<DockPaneRecord>,
    imported_panes: Vec<ImportedDockPane>,
    closed_titles: Vec<String>,
    transferred_titles: Vec<String>,
    tab_bar_height: f32,
    gap_width: f32,
}

#[derive(Clone, Debug)]
pub struct DemoDockSnapshot {
    pub area: DemoDockArea,
    pub imported_panes: Vec<ImportedDockPane>,
    pub closed_titles: Vec<String>,
    pub transferred_titles: Vec<String>,
    pub tab_bar_height: f32,
    pub gap_width: f32,
}

#[derive(Clone, Debug)]
pub struct ImportedDockPane {
    pub slot: DemoDockDropSlot,
    pub content: DemoDockContent,
}

#[derive(Clone)]
struct DockPaneRecord {
    tile_id: TileId,
    title: String,
    content: DemoDockContent,
}

impl DemoDockLayout {
    pub fn for_area(area: DemoDockArea) -> Self {
        let (tree, pane_catalog) = build_tree_for_area(area, &[]);

        Self {
            area,
            tree,
            pane_catalog,
            imported_panes: Vec::new(),
            closed_titles: Vec::new(),
            transferred_titles: Vec::new(),
            tab_bar_height: 24.0,
            gap_width: 2.0,
        }
    }

    pub fn closed_panes(&self) -> Vec<String> {
        self.closed_titles.clone()
    }

    pub fn visible_panes(&self) -> Vec<String> {
        self.pane_catalog
            .iter()
            .filter(|record| {
                !self.closed_titles.contains(&record.title)
                    && !self.transferred_titles.contains(&record.title)
            })
            .map(|record| record.title.clone())
            .collect()
    }

    pub fn snapshot(&self) -> DemoDockSnapshot {
        DemoDockSnapshot {
            area: self.area,
            imported_panes: self.imported_panes.clone(),
            closed_titles: self.closed_titles.clone(),
            transferred_titles: self.transferred_titles.clone(),
            tab_bar_height: self.tab_bar_height,
            gap_width: self.gap_width,
        }
    }

    pub fn restore(&mut self, snapshot: DemoDockSnapshot) {
        self.area = snapshot.area;
        self.imported_panes = snapshot.imported_panes;
        self.closed_titles = snapshot.closed_titles;
        self.transferred_titles = snapshot.transferred_titles;
        self.tab_bar_height = snapshot.tab_bar_height;
        self.gap_width = snapshot.gap_width;
        self.rebuild();
    }

    pub fn restore_last_closed(&mut self) -> Option<String> {
        let title = self.closed_titles.pop()?;
        self.set_title_visible(&title, true);
        Some(title)
    }

    pub fn restore_all_closed(&mut self) {
        let titles = self.closed_titles.clone();
        self.closed_titles.clear();
        for title in titles {
            self.set_title_visible(&title, true);
        }
    }

    pub fn transfer_pane(&mut self, title: &str) -> Option<DemoDockContent> {
        if self.closed_titles.iter().any(|item| item == title)
            || self.transferred_titles.iter().any(|item| item == title)
        {
            return None;
        }

        let record = self
            .pane_catalog
            .iter()
            .find(|record| record.title == title)?
            .clone();

        self.transferred_titles.push(record.title.clone());
        self.set_title_visible(title, false);
        Some(record.content)
    }

    pub fn receive_pane(&mut self, pane: DemoDockContent) {
        self.receive_pane_at(pane, DemoDockDropSlot::Center);
    }

    pub fn receive_pane_at(&mut self, pane: DemoDockContent, slot: DemoDockDropSlot) {
        if self
            .transferred_titles
            .iter()
            .any(|title| title == &pane.title)
        {
            self.transferred_titles.retain(|title| title != &pane.title);
            self.rebuild();
            return;
        }

        if self.pane_catalog.iter().any(|record| record.title == pane.title)
            || self
                .imported_panes
                .iter()
                .any(|item| item.content.title == pane.title)
        {
            return;
        }

        self.imported_panes.push(ImportedDockPane {
            slot,
            content: pane,
        });
        self.rebuild();
    }

    pub fn show(&mut self, ui: &mut Ui) -> DemoDockShowResponse {
        let mut behavior = DemoDockBehavior {
            area: self.area,
            tab_bar_height: self.tab_bar_height,
            gap_width: self.gap_width,
            close_requests: Vec::new(),
        };

        let drop_frame = egui::Frame::default().inner_margin(0);
        let mut drop_rect = None;
        let (_, dropped_payload) = ui.dnd_drop_zone::<DemoDockDragPayload, _>(drop_frame, |ui| {
            ui.set_min_size(ui.available_size());
            drop_rect = Some(ui.max_rect());
            self.tree.ui(&mut behavior, ui);
        });

        for title in behavior.close_requests {
            self.set_title_visible(&title, false);
            if !self.closed_titles.contains(&title) {
                self.closed_titles.push(title);
            }
        }

        DemoDockShowResponse {
            dropped_payload: dropped_payload.map(|payload| {
                let pointer_pos = ui.ctx().input(|input| input.pointer.latest_pos());
                let slot = match (drop_rect, pointer_pos) {
                    (Some(rect), Some(pointer)) => classify_drop_slot(rect, pointer),
                    _ => DemoDockDropSlot::Center,
                };
                ((*payload).clone(), slot)
            }),
        }
    }

    fn rebuild(&mut self) {
        let (tree, pane_catalog) = build_tree_for_area(self.area, &self.imported_panes);
        self.tree = tree;
        self.pane_catalog = pane_catalog;

        let hidden_titles: Vec<String> = self
            .closed_titles
            .iter()
            .chain(self.transferred_titles.iter())
            .cloned()
            .collect();
        for title in hidden_titles {
            self.set_title_visible(&title, false);
        }
    }

    fn set_title_visible(&mut self, title: &str, visible: bool) {
        if let Some(tile_id) = self
            .pane_catalog
            .iter()
            .find(|record| record.title == title)
            .map(|record| record.tile_id)
        {
            self.tree.tiles.set_visible(tile_id, visible);
        }
    }
}

struct DemoDockBehavior {
    area: DemoDockArea,
    tab_bar_height: f32,
    gap_width: f32,
    close_requests: Vec<String>,
}

impl Behavior<DemoDockContent> for DemoDockBehavior {
    fn tab_title_for_pane(&mut self, pane: &DemoDockContent) -> WidgetText {
        format!("{} {}", pane.badge, pane.title).into()
    }

    fn pane_ui(&mut self, ui: &mut Ui, tile_id: TileId, pane: &mut DemoDockContent) -> UiResponse {
        let mut response = UiResponse::None;

        egui::Frame::default()
            .fill(pane.accent.gamma_multiply(0.12))
            .stroke(egui::Stroke::new(1.0, pane.accent.gamma_multiply(0.7)))
            .inner_margin(10)
            .show(ui, |ui| {
                let available = ui.available_size();
                ui.set_min_size(available);
                render_pane_header(
                    ui,
                    self.area,
                    tile_id,
                    pane,
                    &mut self.close_requests,
                    &mut response,
                );

                ui.add_space(8.0);
                for line in &pane.lines {
                    ui.label(line);
                }

                ui.allocate_space(ui.available_size());
            });

        response
    }

    fn tab_bar_height(&self, _style: &egui::Style) -> f32 {
        self.tab_bar_height
    }

    fn gap_width(&self, _style: &egui::Style) -> f32 {
        self.gap_width
    }

    fn simplification_options(&self) -> SimplificationOptions {
        SimplificationOptions {
            prune_single_child_tabs: false,
            ..SimplificationOptions::default()
        }
    }

    fn is_tab_closable(&self, _tiles: &Tiles<DemoDockContent>, _tile_id: TileId) -> bool {
        false
    }
}

fn render_pane_header(
    ui: &mut Ui,
    area: DemoDockArea,
    _tile_id: TileId,
    pane: &mut DemoDockContent,
    close_requests: &mut Vec<String>,
    response: &mut UiResponse,
) {
    match pane.header_style {
        PaneHeaderStyle::Tabbed => {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{} {}", pane.badge, pane.title))
                        .small()
                        .strong(),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let drag_response = ui
                        .add(egui::Button::new(RichText::new("::").small()).sense(Sense::drag()))
                        .on_hover_text("Drag pane to reorder in this host or drop into another dock host");
                    drag_response.dnd_set_drag_payload(DemoDockDragPayload {
                        source_area: area,
                        title: pane.title.clone(),
                    });
                    if drag_response.drag_started() {
                        *response = UiResponse::DragStarted;
                    }

                    ui.menu_button("...", |ui| {
                        ui.label(RichText::new(&pane.summary).small());
                        ui.separator();
                        if pane.closable && ui.button("Close").clicked() {
                            close_requests.push(pane.title.clone());
                            ui.close();
                        }
                    });

                    let pin_label = if pane.pinned { "[P]" } else { "[ ]" };
                    if ui
                        .small_button(pin_label)
                        .on_hover_text(if pane.pinned { "Unpin pane" } else { "Pin pane" })
                        .clicked()
                    {
                        pane.pinned = !pane.pinned;
                    }

                    if pane.closable
                        && ui
                            .small_button("x")
                            .on_hover_text("Close pane")
                            .clicked()
                    {
                        close_requests.push(pane.title.clone());
                    }
                });
            });

            ui.label(RichText::new(&pane.summary).small());
        }
        PaneHeaderStyle::Standalone => {
            egui::Frame::default()
                .fill(pane.accent.gamma_multiply(0.16))
                .stroke(egui::Stroke::new(1.0, pane.accent.gamma_multiply(0.85)))
                .inner_margin(8)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(format!("{} {}", pane.badge, pane.title)).strong(),
                            );
                            ui.label(RichText::new(&pane.summary).small());
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let drag_response = ui
                                .add(
                                    egui::Button::new(RichText::new(":::").small())
                                        .sense(Sense::drag()),
                                )
                                .on_hover_text("Drag pane to reorder in this host or drop into another dock host");
                            drag_response.dnd_set_drag_payload(DemoDockDragPayload {
                                source_area: area,
                                title: pane.title.clone(),
                            });
                            if drag_response.drag_started() {
                                *response = UiResponse::DragStarted;
                            }

                            ui.menu_button("...", |ui| {
                                ui.label(RichText::new(&pane.summary).small());
                                ui.separator();
                                if pane.closable && ui.button("Close").clicked() {
                                    close_requests.push(pane.title.clone());
                                    ui.close();
                                }
                            });

                            let pin_label = if pane.pinned { "[P]" } else { "[ ]" };
                            if ui
                                .small_button(pin_label)
                                .on_hover_text(if pane.pinned { "Unpin pane" } else { "Pin pane" })
                                .clicked()
                            {
                                pane.pinned = !pane.pinned;
                            }

                            if pane.closable
                                && ui
                                    .small_button("x")
                                    .on_hover_text("Close pane")
                                    .clicked()
                            {
                                close_requests.push(pane.title.clone());
                            }
                        });
                    });
                });
        }
    }
}

fn build_tree_for_area(
    area: DemoDockArea,
    imported_panes: &[ImportedDockPane],
) -> (Tree<DemoDockContent>, Vec<DockPaneRecord>) {
    match area {
        DemoDockArea::Top => build_top_tree(imported_panes),
        DemoDockArea::Left => build_left_tree(imported_panes),
        DemoDockArea::Right => build_right_tree(imported_panes),
        DemoDockArea::Bottom => build_bottom_tree(imported_panes),
    }
}

fn build_top_tree(imported_panes: &[ImportedDockPane]) -> (Tree<DemoDockContent>, Vec<DockPaneRecord>) {
    let mut tiles = Tiles::default();
    let command_content = DemoDockContent::new(
            "Command Options",
            "CMD",
            "当前命令的即时参数和选择过滤器。",
            vec![
                "Operation: Sketch / Extrude / Measure".to_owned(),
                "Selection Filter: Bodies / Sketches / Datum".to_owned(),
            ],
            Color32::from_rgb(93, 135, 195),
        )
        .standalone()
        .closable(false);
    let command = tiles.insert_pane(command_content.clone());
    let snaps_content = DemoDockContent::new(
            "Snaps",
            "AID",
            "草图辅助和捕捉开关。",
            vec![
                "Grid Snap: On".to_owned(),
                "Constraint Hints: Visible".to_owned(),
            ],
            Color32::from_rgb(82, 161, 125),
        )
        .standalone();
    let snaps = tiles.insert_pane(snaps_content.clone());

    let base_root = tiles.insert_horizontal_tile(vec![command, snaps]);
    let mut pane_catalog = vec![
        DockPaneRecord {
            tile_id: command,
            title: "Command Options".to_owned(),
            content: command_content,
        },
        DockPaneRecord {
            tile_id: snaps,
            title: "Snaps".to_owned(),
            content: snaps_content,
        },
    ];
    let root = attach_imported_panes(
        DemoDockArea::Top,
        &mut tiles,
        base_root,
        None,
        imported_panes,
        &mut pane_catalog,
    );
    (
        Tree::new("dock_top_tree", root, tiles),
        pane_catalog,
    )
}

fn build_left_tree(imported_panes: &[ImportedDockPane]) -> (Tree<DemoDockContent>, Vec<DockPaneRecord>) {
    let mut tiles = Tiles::default();

    let model_content = DemoDockContent::new(
            "Model",
            "TAB",
            "装配结构和对象树。",
            vec![
                "Assembly".to_owned(),
                "Body-01".to_owned(),
                "Sketch-02".to_owned(),
            ],
            Color32::from_rgb(85, 133, 184),
        )
        .tabbed()
        .closable(false);
    let model = tiles.insert_pane(model_content.clone());
    let layers_content = DemoDockContent::new(
            "Layers",
            "TAB",
            "图层显隐和锁定控制。",
            vec![
                "Default".to_owned(),
                "Construction".to_owned(),
                "Dimensions".to_owned(),
            ],
            Color32::from_rgb(131, 110, 182),
        )
        .tabbed();
    let layers = tiles.insert_pane(layers_content.clone());
    let history_content = DemoDockContent::new(
            "History",
            "TAB",
            "参数化特征历史。",
            vec![
                "Sketch-01".to_owned(),
                "Extrude-01".to_owned(),
                "Fillet-02".to_owned(),
            ],
            Color32::from_rgb(189, 125, 67),
        )
        .tabbed();
    let history = tiles.insert_pane(history_content.clone());
    let selection_content = DemoDockContent::new(
            "Selection",
            "PANE",
            "当前选择集摘要。",
            vec![
                "Faces: 2".to_owned(),
                "Edges: 6".to_owned(),
                "Vertices: 0".to_owned(),
            ],
            Color32::from_rgb(82, 161, 125),
        )
        .standalone();
    let selection = tiles.insert_pane(selection_content.clone());

    let tab_group = tiles.insert_tab_tile(vec![model, layers, history]);
    let base_root = tiles.insert_vertical_tile(vec![tab_group, selection]);
    let mut pane_catalog = vec![
        DockPaneRecord {
            tile_id: model,
            title: "Model".to_owned(),
            content: model_content,
        },
        DockPaneRecord {
            tile_id: layers,
            title: "Layers".to_owned(),
            content: layers_content,
        },
        DockPaneRecord {
            tile_id: history,
            title: "History".to_owned(),
            content: history_content,
        },
        DockPaneRecord {
            tile_id: selection,
            title: "Selection".to_owned(),
            content: selection_content,
        },
    ];
    let root = attach_imported_panes(
        DemoDockArea::Left,
        &mut tiles,
        base_root,
        Some(tab_group),
        imported_panes,
        &mut pane_catalog,
    );
    (
        Tree::new("dock_left_tree", root, tiles),
        pane_catalog,
    )
}

fn build_right_tree(imported_panes: &[ImportedDockPane]) -> (Tree<DemoDockContent>, Vec<DockPaneRecord>) {
    let mut tiles = Tiles::default();

    let properties_content = DemoDockContent::new(
            "Properties",
            "TAB",
            "对象属性与参数编辑。",
            vec![
                "Name: Body-01".to_owned(),
                "Length: 120 mm".to_owned(),
                "Material: Aluminum 6061".to_owned(),
            ],
            Color32::from_rgb(74, 142, 169),
        )
        .tabbed()
        .closable(false);
    let properties = tiles.insert_pane(properties_content.clone());
    let inspector_content = DemoDockContent::new(
            "Inspector",
            "TAB",
            "几何检查与拓扑摘要。",
            vec![
                "Faces: 24".to_owned(),
                "Edges: 48".to_owned(),
                "Volume: 91.3 cm^3".to_owned(),
            ],
            Color32::from_rgb(159, 99, 145),
        )
        .tabbed();
    let inspector = tiles.insert_pane(inspector_content.clone());
    let constraints_content = DemoDockContent::new(
            "Constraints",
            "PANE",
            "草图约束与状态列表。",
            vec![
                "Horizontal".to_owned(),
                "Coincident".to_owned(),
                "Equal".to_owned(),
            ],
            Color32::from_rgb(191, 136, 71),
        )
        .standalone()
        .pinned(false);
    let constraints = tiles.insert_pane(constraints_content.clone());

    let top_tabs = tiles.insert_tab_tile(vec![properties, inspector]);
    let base_root = tiles.insert_vertical_tile(vec![top_tabs, constraints]);
    let mut pane_catalog = vec![
        DockPaneRecord {
            tile_id: properties,
            title: "Properties".to_owned(),
            content: properties_content,
        },
        DockPaneRecord {
            tile_id: inspector,
            title: "Inspector".to_owned(),
            content: inspector_content,
        },
        DockPaneRecord {
            tile_id: constraints,
            title: "Constraints".to_owned(),
            content: constraints_content,
        },
    ];
    let root = attach_imported_panes(
        DemoDockArea::Right,
        &mut tiles,
        base_root,
        Some(top_tabs),
        imported_panes,
        &mut pane_catalog,
    );
    (
        Tree::new("dock_right_tree", root, tiles),
        pane_catalog,
    )
}

fn build_bottom_tree(imported_panes: &[ImportedDockPane]) -> (Tree<DemoDockContent>, Vec<DockPaneRecord>) {
    let mut tiles = Tiles::default();

    let messages_content = DemoDockContent::new(
            "Messages",
            "TAB",
            "命令反馈和日志输出。",
            vec![
                "Ready".to_owned(),
                "Opened 3 document tabs".to_owned(),
                "Active tool: Select".to_owned(),
            ],
            Color32::from_rgb(75, 128, 189),
        )
        .tabbed()
        .closable(false);
    let messages = tiles.insert_pane(messages_content.clone());
    let tasks_content = DemoDockContent::new(
            "Tasks",
            "TAB",
            "后台任务和求解队列。",
            vec![
                "Meshing: Idle".to_owned(),
                "Analysis Queue: Empty".to_owned(),
            ],
            Color32::from_rgb(82, 161, 125),
        )
        .tabbed();
    let tasks = tiles.insert_pane(tasks_content.clone());
    let diagnostics_content = DemoDockContent::new(
            "Diagnostics",
            "PANE",
            "验证与问题列表。",
            vec![
                "0 Errors".to_owned(),
                "1 Warning".to_owned(),
                "Unsaved changes in Chassis-3D".to_owned(),
            ],
            Color32::from_rgb(187, 105, 86),
        )
        .standalone()
        .pinned(false);
    let diagnostics = tiles.insert_pane(diagnostics_content.clone());

    let left_tabs = tiles.insert_tab_tile(vec![messages, tasks]);
    let base_root = tiles.insert_horizontal_tile(vec![left_tabs, diagnostics]);
    let mut pane_catalog = vec![
        DockPaneRecord {
            tile_id: messages,
            title: "Messages".to_owned(),
            content: messages_content,
        },
        DockPaneRecord {
            tile_id: tasks,
            title: "Tasks".to_owned(),
            content: tasks_content,
        },
        DockPaneRecord {
            tile_id: diagnostics,
            title: "Diagnostics".to_owned(),
            content: diagnostics_content,
        },
    ];
    let root = attach_imported_panes(
        DemoDockArea::Bottom,
        &mut tiles,
        base_root,
        Some(left_tabs),
        imported_panes,
        &mut pane_catalog,
    );
    (
        Tree::new("dock_bottom_tree", root, tiles),
        pane_catalog,
    )
}

fn attach_imported_panes(
    area: DemoDockArea,
    tiles: &mut Tiles<DemoDockContent>,
    base_root: TileId,
    primary_tab_group: Option<TileId>,
    imported_panes: &[ImportedDockPane],
    pane_catalog: &mut Vec<DockPaneRecord>,
) -> TileId {
    if imported_panes.is_empty() {
        return base_root;
    }

    let mut root = base_root;
    let mut center_tiles = Vec::new();
    let mut left_tiles = Vec::new();
    let mut right_tiles = Vec::new();
    let mut top_tiles = Vec::new();
    let mut bottom_tiles = Vec::new();

    for imported in imported_panes {
        let tile_id = tiles.insert_pane(imported.content.clone());
        pane_catalog.push(DockPaneRecord {
            tile_id,
            title: imported.content.title.clone(),
            content: imported.content.clone(),
        });
        match imported.slot {
            DemoDockDropSlot::Center => center_tiles.push(tile_id),
            DemoDockDropSlot::LeftEdge => left_tiles.push(tile_id),
            DemoDockDropSlot::RightEdge => right_tiles.push(tile_id),
            DemoDockDropSlot::TopEdge => top_tiles.push(tile_id),
            DemoDockDropSlot::BottomEdge => bottom_tiles.push(tile_id),
        }
    }

    if !center_tiles.is_empty() {
        if let Some(tab_group_id) = primary_tab_group {
            if let Some(egui_tiles::Tile::Container(egui_tiles::Container::Tabs(tabs))) =
                tiles.get_mut(tab_group_id)
            {
                for tile_id in center_tiles {
                    tabs.add_child(tile_id);
                }
            }
        } else {
            let center_group = insert_group_tile(tiles, DemoDockDropSlot::Center, center_tiles);
            root = match area {
                DemoDockArea::Top | DemoDockArea::Bottom => {
                    tiles.insert_horizontal_tile(vec![root, center_group])
                }
                DemoDockArea::Left | DemoDockArea::Right => {
                    tiles.insert_vertical_tile(vec![root, center_group])
                }
            };
        }
    }

    if !left_tiles.is_empty() {
        let left_group = insert_group_tile(tiles, DemoDockDropSlot::LeftEdge, left_tiles);
        root = tiles.insert_horizontal_tile(vec![left_group, root]);
    }

    if !right_tiles.is_empty() {
        let right_group = insert_group_tile(tiles, DemoDockDropSlot::RightEdge, right_tiles);
        root = tiles.insert_horizontal_tile(vec![root, right_group]);
    }

    if !top_tiles.is_empty() {
        let top_group = insert_group_tile(tiles, DemoDockDropSlot::TopEdge, top_tiles);
        root = tiles.insert_vertical_tile(vec![top_group, root]);
    }

    if !bottom_tiles.is_empty() {
        let bottom_group = insert_group_tile(tiles, DemoDockDropSlot::BottomEdge, bottom_tiles);
        root = tiles.insert_vertical_tile(vec![root, bottom_group]);
    }

    root
}

fn insert_group_tile(
    tiles: &mut Tiles<DemoDockContent>,
    slot: DemoDockDropSlot,
    children: Vec<TileId>,
) -> TileId {
    match slot {
        DemoDockDropSlot::Center => tiles.insert_tab_tile(children),
        DemoDockDropSlot::LeftEdge | DemoDockDropSlot::RightEdge => {
            let tabs = tiles.insert_tab_tile(children);
            tiles.insert_vertical_tile(vec![tabs])
        }
        DemoDockDropSlot::TopEdge | DemoDockDropSlot::BottomEdge => {
            let tabs = tiles.insert_tab_tile(children);
            tiles.insert_horizontal_tile(vec![tabs])
        }
    }
}

fn classify_drop_slot(rect: egui::Rect, pointer: egui::Pos2) -> DemoDockDropSlot {
    if !rect.contains(pointer) {
        return DemoDockDropSlot::Center;
    }

    let edge_band = (rect.width().min(rect.height()) * 0.24).clamp(24.0, 72.0);
    let left = pointer.x - rect.left();
    let right = rect.right() - pointer.x;
    let top = pointer.y - rect.top();
    let bottom = rect.bottom() - pointer.y;

    let mut nearest = (DemoDockDropSlot::Center, edge_band);
    for candidate in [
        (DemoDockDropSlot::LeftEdge, left),
        (DemoDockDropSlot::RightEdge, right),
        (DemoDockDropSlot::TopEdge, top),
        (DemoDockDropSlot::BottomEdge, bottom),
    ] {
        if candidate.1 <= nearest.1 {
            nearest = candidate;
        }
    }

    nearest.0
}

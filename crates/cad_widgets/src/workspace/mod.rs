use egui::{Align, Button, Color32, Layout, RichText, Sense, Ui};
use egui_tiles::{Behavior, SimplificationOptions, Tile, TileId, Tiles, Tree, UiResponse};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentKind {
    Model3d,
    Plot2d,
    Table,
    Report,
}

impl DocumentKind {
    pub fn badge_text(self) -> &'static str {
        match self {
            DocumentKind::Model3d => "3D",
            DocumentKind::Plot2d => "2D",
            DocumentKind::Table => "TAB",
            DocumentKind::Report => "DOC",
        }
    }
}

#[derive(Clone, Debug)]
pub struct DocumentTabsState {
    pub active_id: String,
}

impl DocumentTabsState {
    pub fn new(active_id: impl Into<String>) -> Self {
        Self {
            active_id: active_id.into(),
        }
    }

    pub fn activate(&mut self, id: impl Into<String>) {
        self.active_id = id.into();
    }

    pub fn is_active(&self, id: &str) -> bool {
        self.active_id == id
    }
}

pub struct DocumentTab<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub kind: DocumentKind,
    pub dirty: bool,
    pub closable: bool,
}

impl<'a> DocumentTab<'a> {
    pub fn new(id: &'a str, title: &'a str, kind: DocumentKind) -> Self {
        Self {
            id,
            title,
            kind,
            dirty: false,
            closable: true,
        }
    }

    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

#[derive(Default)]
pub struct DocumentTabsResponse {
    pub activated: Option<String>,
    pub close_requested: Option<String>,
}

pub struct DocumentTabs<'a> {
    id_source: &'a str,
    tabs: &'a [DocumentTab<'a>],
}

impl<'a> DocumentTabs<'a> {
    pub fn new(id_source: &'a str, tabs: &'a [DocumentTab<'a>]) -> Self {
        Self { id_source, tabs }
    }

    pub fn show(self, ui: &mut Ui, state: &mut DocumentTabsState) -> DocumentTabsResponse {
        let mut response = DocumentTabsResponse::default();

        ui.push_id(self.id_source, |ui| {
            ui.horizontal_wrapped(|ui| {
                for tab in self.tabs {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            let button_text = if tab.dirty {
                                format!("{} {} *", tab.kind.badge_text(), tab.title)
                            } else {
                                format!("{} {}", tab.kind.badge_text(), tab.title)
                            };

                            let tab_clicked = ui
                                .add(Button::new(button_text).selected(state.is_active(tab.id)))
                                .clicked();

                            if tab_clicked {
                                state.activate(tab.id);
                                response.activated = Some(tab.id.to_owned());
                            }

                            if tab.closable && ui.small_button("x").clicked() {
                                response.close_requested = Some(tab.id.to_owned());
                            }
                        });
                    });
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(RichText::new("Central Workspace").small());
                });
            });
        });

        response
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentPaneHeaderStyle {
    Tabbed,
    Standalone,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkspaceSplitAxis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub enum DocumentWorkspaceNode {
    Document(String),
    Tabs(Vec<DocumentWorkspaceNode>),
    Split {
        axis: WorkspaceSplitAxis,
        children: Vec<DocumentWorkspaceNode>,
    },
}

impl DocumentWorkspaceNode {
    pub fn document(id: impl Into<String>) -> Self {
        Self::Document(id.into())
    }

    pub fn single_tab(id: impl Into<String>) -> Self {
        Self::Tabs(vec![Self::Document(id.into())])
    }

    pub fn tabs(children: Vec<DocumentWorkspaceNode>) -> Self {
        Self::Tabs(children)
    }

    pub fn horizontal(children: Vec<DocumentWorkspaceNode>) -> Self {
        Self::Split {
            axis: WorkspaceSplitAxis::Horizontal,
            children,
        }
    }

    pub fn vertical(children: Vec<DocumentWorkspaceNode>) -> Self {
        Self::Split {
            axis: WorkspaceSplitAxis::Vertical,
            children,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorkspaceDocument {
    pub id: String,
    pub title: String,
    pub kind: DocumentKind,
    pub summary: String,
    pub lines: Vec<String>,
    pub accent: Color32,
    pub header_style: DocumentPaneHeaderStyle,
    pub dirty: bool,
    pub closable: bool,
}

impl WorkspaceDocument {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        kind: DocumentKind,
        summary: impl Into<String>,
        lines: Vec<String>,
        accent: Color32,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            kind,
            summary: summary.into(),
            lines,
            accent,
            header_style: DocumentPaneHeaderStyle::Tabbed,
            dirty: false,
            closable: true,
        }
    }

    pub fn standalone(mut self) -> Self {
        self.header_style = DocumentPaneHeaderStyle::Standalone;
        self
    }

    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

#[derive(Clone, Debug)]
pub struct DocumentWorkspaceBuilder {
    id_source: String,
    documents: Vec<WorkspaceDocument>,
    root: DocumentWorkspaceNode,
    tab_bar_height: f32,
    gap_width: f32,
}

impl DocumentWorkspaceBuilder {
    pub fn new(id_source: impl Into<String>, root: DocumentWorkspaceNode) -> Self {
        Self {
            id_source: id_source.into(),
            documents: Vec::new(),
            root,
            tab_bar_height: 26.0,
            gap_width: 2.0,
        }
    }

    pub fn document(mut self, document: WorkspaceDocument) -> Self {
        self.documents.push(document);
        self
    }

    pub fn documents(mut self, documents: impl IntoIterator<Item = WorkspaceDocument>) -> Self {
        self.documents.extend(documents);
        self
    }

    pub fn tab_bar_height(mut self, tab_bar_height: f32) -> Self {
        self.tab_bar_height = tab_bar_height;
        self
    }

    pub fn gap_width(mut self, gap_width: f32) -> Self {
        self.gap_width = gap_width;
        self
    }

    pub fn build(self) -> DocumentWorkspaceLayout {
        DocumentWorkspaceLayout::from_builder(self)
    }
}

#[derive(Clone, Debug)]
pub struct DocumentWorkspaceSnapshot {
    pub builder: DocumentWorkspaceBuilder,
    pub closed_document_ids: Vec<String>,
}

#[derive(Clone)]
pub struct DocumentWorkspaceLayout {
    builder: DocumentWorkspaceBuilder,
    tree: Tree<WorkspaceDocument>,
    document_tile_ids: HashMap<String, TileId>,
    closed_document_ids: Vec<String>,
}

impl DocumentWorkspaceLayout {
    pub fn from_builder(builder: DocumentWorkspaceBuilder) -> Self {
        let (tree, document_tile_ids) = build_tree(&builder);
        Self {
            builder,
            tree,
            document_tile_ids,
            closed_document_ids: Vec::new(),
        }
    }

    pub fn demo() -> Self {
        demo_workspace_builder().build()
    }

    pub fn snapshot(&self) -> DocumentWorkspaceSnapshot {
        DocumentWorkspaceSnapshot {
            builder: self.builder.clone(),
            closed_document_ids: self.closed_document_ids.clone(),
        }
    }

    pub fn restore(&mut self, snapshot: DocumentWorkspaceSnapshot) {
        self.builder = snapshot.builder;
        let (tree, document_tile_ids) = build_tree(&self.builder);
        self.tree = tree;
        self.document_tile_ids = document_tile_ids;
        self.closed_document_ids.clear();

        for document_id in snapshot.closed_document_ids {
            let _ = self.close_document(&document_id);
        }
    }

    pub fn restore_document(&mut self, document_id: &str) -> bool {
        if let Some(index) = self
            .closed_document_ids
            .iter()
            .position(|id| id == document_id)
        {
            self.closed_document_ids.remove(index);
            if let Some(tile_id) = self.document_tile_ids.get(document_id) {
                self.tree.tiles.set_visible(*tile_id, true);
            }
            return true;
        }

        false
    }

    pub fn restore_all_documents(&mut self) {
        let document_ids = self.closed_document_ids.clone();
        for document_id in document_ids {
            let _ = self.restore_document(&document_id);
        }
    }

    pub fn closed_documents(&self) -> Vec<WorkspaceDocument> {
        self.closed_document_ids
            .iter()
            .filter_map(|document_id| self.find_document(document_id).cloned())
            .collect()
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let mut behavior = DocumentWorkspaceBehavior {
            tab_bar_height: self.builder.tab_bar_height,
            gap_width: self.builder.gap_width,
            close_requests: Vec::new(),
        };
        self.tree.ui(&mut behavior, ui);

        for document_id in behavior.close_requests {
            let _ = self.close_document(&document_id);
        }
    }

    fn close_document(&mut self, document_id: &str) -> bool {
        if self.closed_document_ids.iter().any(|id| id == document_id) {
            return false;
        }

        let Some(tile_id) = self.document_tile_ids.get(document_id).copied() else {
            return false;
        };

        self.tree.tiles.set_visible(tile_id, false);
        self.closed_document_ids.push(document_id.to_owned());
        true
    }

    fn find_document(&self, document_id: &str) -> Option<&WorkspaceDocument> {
        self.builder
            .documents
            .iter()
            .find(|document| document.id == document_id)
    }
}

struct DocumentWorkspaceBehavior {
    tab_bar_height: f32,
    gap_width: f32,
    close_requests: Vec<String>,
}

impl Behavior<WorkspaceDocument> for DocumentWorkspaceBehavior {
    fn tab_title_for_pane(&mut self, pane: &WorkspaceDocument) -> egui::WidgetText {
        let suffix = if pane.dirty { " *" } else { "" };
        format!("{} {}{}", pane.kind.badge_text(), pane.title, suffix).into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut Ui,
        _tile_id: TileId,
        pane: &mut WorkspaceDocument,
    ) -> UiResponse {
        let mut response = UiResponse::None;

        egui::Frame::default()
            .fill(pane.accent.gamma_multiply(0.10))
            .stroke(egui::Stroke::new(1.0, pane.accent.gamma_multiply(0.7)))
            .inner_margin(10)
            .show(ui, |ui| {
                render_document_header(ui, pane, &mut self.close_requests, &mut response);

                ui.add_space(8.0);
                for line in &pane.lines {
                    ui.label(line);
                }
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

    fn is_tab_closable(&self, tiles: &Tiles<WorkspaceDocument>, tile_id: TileId) -> bool {
        match tiles.get(tile_id) {
            Some(Tile::Pane(pane)) => pane.closable,
            _ => false,
        }
    }

    fn on_tab_close(&mut self, tiles: &mut Tiles<WorkspaceDocument>, tile_id: TileId) -> bool {
        if let Some(Tile::Pane(pane)) = tiles.get(tile_id) {
            self.close_requests.push(pane.id.clone());
        }
        false
    }
}

fn render_document_header(
    ui: &mut Ui,
    pane: &mut WorkspaceDocument,
    close_requests: &mut Vec<String>,
    response: &mut UiResponse,
) {
    match pane.header_style {
        DocumentPaneHeaderStyle::Tabbed => {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{} {}", pane.kind.badge_text(), pane.title))
                        .small()
                        .strong(),
                );

                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    let drag_response = ui
                        .add(egui::Button::new(RichText::new("::").small()).sense(Sense::drag()))
                        .on_hover_text("Drag document pane within the central workspace");
                    if drag_response.drag_started() {
                        *response = UiResponse::DragStarted;
                    }

                    ui.menu_button("...", |ui| {
                        ui.label(RichText::new(&pane.summary).small());
                        ui.separator();
                        if pane.closable && ui.button("Close").clicked() {
                            close_requests.push(pane.id.clone());
                            ui.close();
                        }
                    });

                    if pane.closable
                        && ui
                            .small_button("x")
                            .on_hover_text("Close document")
                            .clicked()
                    {
                        close_requests.push(pane.id.clone());
                    }
                });
            });

            ui.label(RichText::new(&pane.summary).small());
        }
        DocumentPaneHeaderStyle::Standalone => {
            egui::Frame::default()
                .fill(pane.accent.gamma_multiply(0.16))
                .stroke(egui::Stroke::new(1.0, pane.accent.gamma_multiply(0.85)))
                .inner_margin(8)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(format!("{} {}", pane.kind.badge_text(), pane.title))
                                    .strong(),
                            );
                            ui.label(RichText::new(&pane.summary).small());
                        });

                        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                            let drag_response = ui
                                .add(
                                    egui::Button::new(RichText::new(":::").small())
                                        .sense(Sense::drag()),
                                )
                                .on_hover_text("Drag document pane within the central workspace");
                            if drag_response.drag_started() {
                                *response = UiResponse::DragStarted;
                            }

                            ui.menu_button("...", |ui| {
                                ui.label(RichText::new(&pane.summary).small());
                                ui.separator();
                                if pane.closable && ui.button("Close").clicked() {
                                    close_requests.push(pane.id.clone());
                                    ui.close();
                                }
                            });

                            if pane.closable
                                && ui
                                    .small_button("x")
                                    .on_hover_text("Close document")
                                    .clicked()
                            {
                                close_requests.push(pane.id.clone());
                            }
                        });
                    });
                });
        }
    }
}

fn build_tree(builder: &DocumentWorkspaceBuilder) -> (Tree<WorkspaceDocument>, HashMap<String, TileId>) {
    let mut tiles = Tiles::default();
    let mut document_tile_ids = HashMap::new();
    let root = build_node(
        &mut tiles,
        &builder.root,
        &builder.documents,
        &mut document_tile_ids,
    );
    (
        Tree::new(builder.id_source.clone(), root, tiles),
        document_tile_ids,
    )
}

fn build_node(
    tiles: &mut Tiles<WorkspaceDocument>,
    node: &DocumentWorkspaceNode,
    documents: &[WorkspaceDocument],
    document_tile_ids: &mut HashMap<String, TileId>,
) -> TileId {
    match node {
        DocumentWorkspaceNode::Document(document_id) => {
            let Some(document) = documents.iter().find(|document| &document.id == document_id) else {
                panic!("missing workspace document: {document_id}");
            };
            let tile_id = tiles.insert_pane(document.clone());
            document_tile_ids.insert(document.id.clone(), tile_id);
            tile_id
        }
        DocumentWorkspaceNode::Tabs(children) => {
            let children = children
                .iter()
                .map(|child| build_node(tiles, child, documents, document_tile_ids))
                .collect();
            tiles.insert_tab_tile(children)
        }
        DocumentWorkspaceNode::Split { axis, children } => {
            let children = children
                .iter()
                .map(|child| build_node(tiles, child, documents, document_tile_ids))
                .collect();
            match axis {
                WorkspaceSplitAxis::Horizontal => tiles.insert_horizontal_tile(children),
                WorkspaceSplitAxis::Vertical => tiles.insert_vertical_tile(children),
            }
        }
    }
}

fn demo_workspace_builder() -> DocumentWorkspaceBuilder {
    let root = DocumentWorkspaceNode::vertical(vec![
        DocumentWorkspaceNode::horizontal(vec![
            DocumentWorkspaceNode::single_tab("doc:model_3d"),
            DocumentWorkspaceNode::tabs(vec![
                DocumentWorkspaceNode::document("doc:force_plot"),
                DocumentWorkspaceNode::document("doc:bom_table"),
                DocumentWorkspaceNode::document("doc:inspection_report"),
            ]),
        ]),
        DocumentWorkspaceNode::single_tab("doc:notes"),
    ]);

    DocumentWorkspaceBuilder::new("center_document_tree", root)
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
                Color32::from_rgb(77, 126, 189),
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
                Color32::from_rgb(82, 161, 125),
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
                Color32::from_rgb(190, 132, 76),
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
                Color32::from_rgb(146, 101, 170),
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
                Color32::from_rgb(116, 121, 138),
            )
            .standalone()
            .dirty(false),
        ])
        .tab_bar_height(26.0)
        .gap_width(2.0)
}

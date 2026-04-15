use egui::{Align, Layout, RichText, Ui};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default)]
pub struct TreeViewState {
    pub expanded: BTreeSet<String>,
    pub selected: BTreeSet<String>,
    pub active: Option<String>,
}

impl TreeViewState {
    pub fn is_expanded(&self, id: &str) -> bool {
        self.expanded.contains(id)
    }

    pub fn is_selected(&self, id: &str) -> bool {
        self.selected.contains(id)
    }

    pub fn toggle_expanded(&mut self, id: &str) {
        if !self.expanded.remove(id) {
            self.expanded.insert(id.to_owned());
        }
    }

    pub fn select_single(&mut self, id: &str) {
        self.selected.clear();
        self.selected.insert(id.to_owned());
        self.active = Some(id.to_owned());
    }

    pub fn toggle_selected(&mut self, id: &str) {
        if !self.selected.remove(id) {
            self.selected.insert(id.to_owned());
        }
        self.active = Some(id.to_owned());
    }
}

#[derive(Clone, Debug)]
pub struct TreeNode<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub icon: Option<&'a str>,
    pub detail: Option<&'a str>,
    pub children: Vec<TreeNode<'a>>,
}

impl<'a> TreeNode<'a> {
    pub fn leaf(id: &'a str, label: &'a str) -> Self {
        Self {
            id,
            label,
            icon: None,
            detail: None,
            children: Vec::new(),
        }
    }

    pub fn branch(id: &'a str, label: &'a str, children: Vec<TreeNode<'a>>) -> Self {
        Self {
            id,
            label,
            icon: None,
            detail: None,
            children,
        }
    }

    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn detail(mut self, detail: &'a str) -> Self {
        self.detail = Some(detail);
        self
    }
}

#[derive(Default)]
pub struct TreeViewResponse {
    pub clicked: Option<String>,
    pub context_requested: Option<String>,
}

pub struct TreeView<'a> {
    id_source: &'a str,
    indent: f32,
}

impl<'a> TreeView<'a> {
    pub fn new(id_source: &'a str) -> Self {
        Self {
            id_source,
            indent: 18.0,
        }
    }

    pub fn indent(mut self, indent: f32) -> Self {
        self.indent = indent;
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        nodes: &[TreeNode<'a>],
        state: &mut TreeViewState,
    ) -> TreeViewResponse {
        let mut response = TreeViewResponse::default();

        ui.push_id(self.id_source, |ui| {
            for node in nodes {
                self.show_node(ui, node, 0, state, &mut response);
            }
        });

        response
    }

    fn show_node(
        &self,
        ui: &mut Ui,
        node: &TreeNode<'a>,
        depth: usize,
        state: &mut TreeViewState,
        response: &mut TreeViewResponse,
    ) {
        let has_children = !node.children.is_empty();
        let is_expanded = state.is_expanded(node.id);
        let is_selected = state.is_selected(node.id);

        ui.horizontal(|ui| {
            ui.add_space(depth as f32 * self.indent);

            if has_children {
                let symbol = if is_expanded { "▾" } else { "▸" };
                if ui.small_button(symbol).clicked() {
                    state.toggle_expanded(node.id);
                }
            } else {
                ui.add_space(18.0);
            }

            let mut label = String::new();
            if let Some(icon) = node.icon {
                label.push_str(icon);
                label.push(' ');
            }
            label.push_str(node.label);

            let row_response = ui.selectable_label(is_selected, label);

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some(detail) = node.detail {
                    ui.label(RichText::new(detail).small());
                }
            });

            if row_response.clicked() {
                let additive = ui.input(|input| input.modifiers.command || input.modifiers.ctrl);
                if additive {
                    state.toggle_selected(node.id);
                } else {
                    state.select_single(node.id);
                }
                response.clicked = Some(node.id.to_owned());
            }

            if row_response.double_clicked() && has_children {
                state.toggle_expanded(node.id);
            }

            if row_response.secondary_clicked() {
                if !state.is_selected(node.id) {
                    state.select_single(node.id);
                }
                response.context_requested = Some(node.id.to_owned());
            }
        });

        if has_children && state.is_expanded(node.id) {
            for child in &node.children {
                self.show_node(ui, child, depth + 1, state, response);
            }
        }
    }
}

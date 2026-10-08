//! Timeline layer stack widget: switches, labels, and layer rows.

#[derive(Clone, Debug, PartialEq)]
pub struct LayerItemDef {
    pub id: u64,
    pub name: String,
    pub video: bool,
    pub audio: bool,
    pub solo: bool,
    pub locked: bool,
    pub shy: bool,
    pub is_adjustment: bool,
    pub expanded: bool,
    pub children: Vec<LayerItemDef>,
}

pub struct LayerTreeWidget {
    pub layers: Vec<LayerItemDef>,
    pub selected_layer_id: Option<u64>,
    pub time: u64,
    pub work_area_start: u64,
    pub work_area_end: u64,
    pub shy_guy_enabled: bool,
}

impl LayerTreeWidget {
    pub fn new() -> Self {
        Self { layers: Vec::new(), selected_layer_id: None, time: 0, work_area_start: 0, work_area_end: 300, shy_guy_enabled: false }
    }

    pub fn select_layer(&mut self, id: u64) {
        self.selected_layer_id = Some(id);
    }

    pub fn toggle_video(&mut self, id: u64) {
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.video = !item.video;
        }
    }

    pub fn toggle_solo(&mut self, id: u64) {
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.solo = !item.solo;
        }
    }

    pub fn toggle_lock(&mut self, id: u64) {
        if let Some(item) = find_layer_mut(&mut self.layers, id) {
            item.locked = !item.locked;
        }
    }

    /// Layers hidden while the "shy guy" switch is on.
    pub fn visible_layers(&self) -> Vec<u64> {
        self.layers.iter().filter(|item| !(self.shy_guy_enabled && item.shy)).map(|item| item.id).collect()
    }
}

fn find_layer_mut(items: &mut [LayerItemDef], id: u64) -> Option<&mut LayerItemDef> {
    for item in items.iter_mut() {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = find_layer_mut(&mut item.children, id) {
            return Some(found);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer(id: u64, name: &str) -> LayerItemDef {
        LayerItemDef {
            id,
            name: name.to_string(),
            video: true,
            audio: false,
            solo: false,
            locked: false,
            shy: false,
            is_adjustment: false,
            expanded: false,
            children: vec![],
        }
    }

    #[test]
    fn test_layer_tree_mutation() {
        let mut tree = LayerTreeWidget::new();
        tree.layers.push(layer(1, "Background"));
        tree.layers.push(layer(2, "Title"));

        tree.select_layer(2);
        assert_eq!(tree.selected_layer_id, Some(2));

        tree.toggle_video(2);
        assert!(!tree.layers[1].video);
        tree.toggle_video(2);
        assert!(tree.layers[1].video);

        tree.toggle_solo(2);
        assert!(tree.layers[1].solo);

        tree.toggle_lock(2);
        assert!(tree.layers[1].locked);
    }

    #[test]
    fn test_shy_layers_hidden() {
        let mut tree = LayerTreeWidget::new();
        tree.layers.push(layer(1, "Visible"));
        tree.layers.push(layer(2, "Shy"));
        tree.layers[1].shy = true;

        assert_eq!(tree.visible_layers(), vec![1, 2]);
        tree.shy_guy_enabled = true;
        assert_eq!(tree.visible_layers(), vec![1]);
    }
}

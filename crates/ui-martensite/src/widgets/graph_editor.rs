//! Graph editor widget: keyframe velocity curves with split in/out ease handles.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityHandle {
    /// Incoming influence (0–100 %): how strongly the curve eases into the keyframe.
    pub incoming: f32,
    /// Outgoing influence (0–100 %): how strongly the curve eases out of the keyframe.
    pub outgoing: f32,
    /// Incoming and outgoing handles move together until Alt-drag splits them.
    pub linked: bool,
}

impl VelocityHandle {
    pub const fn linear() -> Self {
        Self { incoming: 0.0, outgoing: 0.0, linked: true }
    }

    pub const fn easy_ease() -> Self {
        Self { incoming: 33.33, outgoing: 33.33, linked: true }
    }

    pub fn is_split(&self) -> bool {
        !self.linked || (self.incoming - self.outgoing).abs() > f32::EPSILON
    }

    /// Drag one side; while linked the other side follows. Alt-drag passes
    /// `split: true` to move only that side.
    pub fn set_incoming(&mut self, influence: f32, split: bool) {
        self.incoming = influence.clamp(0.0, 100.0);
        if split {
            self.linked = false;
        } else if self.linked {
            self.outgoing = self.incoming;
        }
    }

    pub fn set_outgoing(&mut self, influence: f32, split: bool) {
        self.outgoing = influence.clamp(0.0, 100.0);
        if split {
            self.linked = false;
        } else if self.linked {
            self.incoming = self.outgoing;
        }
    }

    pub fn relink(&mut self) {
        let avg = (self.incoming + self.outgoing) / 2.0;
        self.incoming = avg;
        self.outgoing = avg;
        self.linked = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GraphMode {
    ValueGraph,
    SpeedGraph,
}

pub struct GraphEditorWidget {
    pub mode: GraphMode,
    pub selected_keyframe: Option<u64>,
    pub handle: VelocityHandle,
    pub show_reference_lines: bool,
}

impl GraphEditorWidget {
    pub fn new() -> Self {
        Self { mode: GraphMode::ValueGraph, selected_keyframe: None, handle: VelocityHandle::linear(), show_reference_lines: true }
    }

    pub fn select_keyframe(&mut self, id: u64) {
        self.selected_keyframe = Some(id);
    }

    pub fn set_mode(&mut self, mode: GraphMode) {
        self.mode = mode;
    }

    pub fn apply_easy_ease(&mut self) {
        self.handle = VelocityHandle::easy_ease();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_handles_move_together() {
        let mut handle = VelocityHandle::linear();
        handle.set_incoming(45.0, false);
        assert_eq!(handle.incoming, 45.0);
        assert_eq!(handle.outgoing, 45.0);
        assert!(!handle.is_split());
    }

    #[test]
    fn test_split_handles() {
        let mut handle = VelocityHandle::easy_ease();
        handle.set_outgoing(80.0, true);
        assert_eq!(handle.incoming, 33.33);
        assert_eq!(handle.outgoing, 80.0);
        assert!(handle.is_split());

        handle.relink();
        assert!(!handle.is_split());
        assert!((handle.incoming - 56.665).abs() < 0.01);
    }

    #[test]
    fn test_influence_clamped() {
        let mut handle = VelocityHandle::linear();
        handle.set_incoming(250.0, false);
        assert_eq!(handle.incoming, 100.0);
        handle.set_outgoing(-10.0, true);
        assert_eq!(handle.outgoing, 0.0);
    }

    #[test]
    fn test_graph_editor_state() {
        let mut editor = GraphEditorWidget::new();
        assert_eq!(editor.mode, GraphMode::ValueGraph);

        editor.set_mode(GraphMode::SpeedGraph);
        assert_eq!(editor.mode, GraphMode::SpeedGraph);

        editor.select_keyframe(7);
        assert_eq!(editor.selected_keyframe, Some(7));

        editor.apply_easy_ease();
        assert_eq!(editor.handle, VelocityHandle::easy_ease());
    }
}

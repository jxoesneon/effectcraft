//! Tool strip widget: horizontal After Effects toolbar with tool flyouts.

use effectcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolSlot {
    pub primary: Tool,
    pub alternatives: &'static [Tool],
}

pub const TOOL_SLOTS: &[ToolSlot] = &[
    ToolSlot { primary: Tool::Selection, alternatives: &[] },
    ToolSlot { primary: Tool::Hand, alternatives: &[] },
    ToolSlot { primary: Tool::Zoom, alternatives: &[] },
    ToolSlot { primary: Tool::Rotation, alternatives: &[] },
    ToolSlot { primary: Tool::PanBehind, alternatives: &[] },
    ToolSlot { primary: Tool::Shape, alternatives: &[] },
    ToolSlot { primary: Tool::Pen, alternatives: &[] },
    ToolSlot { primary: Tool::Type, alternatives: &[] },
    ToolSlot { primary: Tool::Brush, alternatives: &[Tool::CloneStamp, Tool::Eraser] },
    ToolSlot { primary: Tool::RotoBrush, alternatives: &[] },
];

pub struct ToolStripWidget {
    pub active_tool: Tool,
    pub workspace_label: String,
    pub open_flyout: Option<usize>,
}

impl ToolStripWidget {
    pub fn new() -> Self {
        Self { active_tool: Tool::Selection, workspace_label: "Standard".to_string(), open_flyout: None }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn open_flyout_at(&mut self, slot: usize) {
        if slot < TOOL_SLOTS.len() {
            self.open_flyout = Some(slot);
        }
    }

    pub fn close_flyout(&mut self) {
        self.open_flyout = None;
    }

    /// Pick a tool from the open flyout (a slot's primary or one of its alternatives).
    pub fn pick_from_flyout(&mut self, tool: Tool) -> bool {
        let Some(slot) = self.open_flyout else { return false };
        let Some(def) = TOOL_SLOTS.get(slot) else { return false };
        if def.primary == tool || def.alternatives.contains(&tool) {
            self.active_tool = tool;
            self.open_flyout = None;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_strip_state() {
        let mut strip = ToolStripWidget::new();
        assert_eq!(strip.active_tool, Tool::Selection);
        assert_eq!(strip.workspace_label, "Standard");
        assert_eq!(strip.open_flyout, None);

        strip.set_tool(Tool::Pen);
        assert_eq!(strip.active_tool, Tool::Pen);
    }

    #[test]
    fn test_tool_flyout() {
        let mut strip = ToolStripWidget::new();

        // Brush slot (index 8) flys out to Clone Stamp and Eraser.
        strip.open_flyout_at(8);
        assert_eq!(strip.open_flyout, Some(8));
        assert!(strip.pick_from_flyout(Tool::CloneStamp));
        assert_eq!(strip.active_tool, Tool::CloneStamp);
        assert_eq!(strip.open_flyout, None);

        // Out-of-range slots and wrong tools do nothing.
        strip.open_flyout_at(99);
        assert_eq!(strip.open_flyout, None);
        strip.open_flyout_at(0);
        assert!(!strip.pick_from_flyout(Tool::Eraser));
    }
}

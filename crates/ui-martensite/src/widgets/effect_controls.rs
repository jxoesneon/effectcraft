//! Effect Controls widget: scrubby parameters for the selected layer's effect stack.

use crate::widgets::scrubby_input::ScrubbyInputWidget;
use effectcraft_engine::Tool;

pub struct EffectControlsWidget {
    pub active_tool: Tool,
    pub effect_name: String,
    pub blurriness: ScrubbyInputWidget,
    pub opacity: ScrubbyInputWidget,
    pub evolution: ScrubbyInputWidget,
    pub effect_enabled: bool,
    pub keyframes_armed: bool,
}

impl EffectControlsWidget {
    pub fn new(effect_name: &str) -> Self {
        Self {
            active_tool: Tool::Selection,
            effect_name: effect_name.to_string(),
            blurriness: ScrubbyInputWidget::new("Blurriness", 0.0, 0.0, 1000.0, "px"),
            opacity: ScrubbyInputWidget::new("Opacity", 100.0, 0.0, 100.0, "%"),
            evolution: ScrubbyInputWidget::new("Evolution", 0.0, 0.0, 360.0, "°"),
            effect_enabled: true,
            keyframes_armed: false,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.active_tool = tool;
    }

    pub fn toggle_effect(&mut self) -> bool {
        self.effect_enabled = !self.effect_enabled;
        self.effect_enabled
    }

    pub fn toggle_keyframes(&mut self) -> bool {
        self.keyframes_armed = !self.keyframes_armed;
        self.keyframes_armed
    }

    pub fn reset(&mut self) {
        self.blurriness.set_direct_value(0.0);
        self.opacity.set_direct_value(100.0);
        self.evolution.set_direct_value(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_controls_defaults() {
        let mut controls = EffectControlsWidget::new("Gaussian Blur");
        assert_eq!(controls.effect_name, "Gaussian Blur");
        assert_eq!(controls.active_tool, Tool::Selection);
        assert_eq!(controls.blurriness.value, 0.0);
        assert_eq!(controls.opacity.value, 100.0);
        assert!(controls.effect_enabled);

        controls.set_tool(Tool::RotoBrush);
        assert_eq!(controls.active_tool, Tool::RotoBrush);

        assert!(!controls.toggle_effect());
        assert!(!controls.effect_enabled);
        assert!(controls.toggle_keyframes());
        assert!(controls.keyframes_armed);

        controls.blurriness.set_direct_value(42.0);
        controls.reset();
        assert_eq!(controls.blurriness.value, 0.0);
    }
}

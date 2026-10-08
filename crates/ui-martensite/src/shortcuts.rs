//! Keystroke state machine providing After Effects keyboard ergonomics.

use effectcraft_engine::Tool;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub cmd: bool,
}

impl KeyModifiers {
    pub const fn empty() -> Self {
        Self { shift: false, ctrl: false, alt: false, cmd: false }
    }
}

pub struct KeyboardEngine {
    pub prior_tool: Option<Tool>,
    pub space_held: bool,
    pub z_held: bool,
    pub alt_held: bool,
}

impl KeyboardEngine {
    pub fn new() -> Self {
        Self { prior_tool: None, space_held: false, z_held: false, alt_held: false }
    }

    pub fn on_key_down(&mut self, key: &str, current: Tool) -> Option<Tool> {
        match key {
            "Space" if !self.space_held => {
                self.space_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Hand)
            }
            "z" | "Z" if !self.z_held => {
                self.z_held = true;
                self.prior_tool = Some(current);
                Some(Tool::Zoom)
            }
            "Alt" => {
                self.alt_held = true;
                None
            }
            // After Effects single-key tool shortcuts
            "v" | "V" => Some(Tool::Selection),
            "h" | "H" => Some(Tool::Hand),
            "w" | "W" => Some(Tool::Rotation),
            "y" | "Y" => Some(Tool::PanBehind),
            "q" | "Q" => Some(Tool::Shape),
            "g" | "G" => Some(Tool::Pen),
            "t" | "T" => Some(Tool::Type),
            "b" | "B" => Some(Tool::Brush),
            "e" | "E" => Some(Tool::Eraser),
            _ => None,
        }
    }

    pub fn on_key_up(&mut self, key: &str) -> Option<Tool> {
        match key {
            "Space" if self.space_held => {
                self.space_held = false;
                self.prior_tool.take()
            }
            "z" | "Z" if self.z_held => {
                self.z_held = false;
                self.prior_tool.take()
            }
            "Alt" => {
                self.alt_held = false;
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_key_tool_switching() {
        let mut k = KeyboardEngine::new();
        assert_eq!(k.on_key_down("v", Tool::Brush), Some(Tool::Selection));
        assert_eq!(k.on_key_down("H", Tool::Selection), Some(Tool::Hand));
        assert_eq!(k.on_key_down("w", Tool::Hand), Some(Tool::Rotation));
        assert_eq!(k.on_key_down("y", Tool::Rotation), Some(Tool::PanBehind));
        assert_eq!(k.on_key_down("q", Tool::PanBehind), Some(Tool::Shape));
        assert_eq!(k.on_key_down("g", Tool::Shape), Some(Tool::Pen));
        assert_eq!(k.on_key_down("t", Tool::Pen), Some(Tool::Type));
        assert_eq!(k.on_key_down("b", Tool::Type), Some(Tool::Brush));
        assert_eq!(k.on_key_down("e", Tool::Brush), Some(Tool::Eraser));
    }

    #[test]
    fn test_spring_loaded_hand_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Brush;

        // Press Space: temporary Hand
        assert_eq!(k.on_key_down("Space", initial), Some(Tool::Hand));
        assert!(k.space_held);

        // Multiple down events shouldn't overwrite prior tool
        assert_eq!(k.on_key_down("Space", Tool::Hand), None);

        // Release Space: restores initial tool
        assert_eq!(k.on_key_up("Space"), Some(initial));
        assert!(!k.space_held);
    }

    #[test]
    fn test_spring_loaded_zoom_tool() {
        let mut k = KeyboardEngine::new();
        let initial = Tool::Pen;

        assert_eq!(k.on_key_down("z", initial), Some(Tool::Zoom));
        assert!(k.z_held);

        assert_eq!(k.on_key_up("z"), Some(initial));
        assert!(!k.z_held);
    }
}

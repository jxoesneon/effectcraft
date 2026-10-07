//! Sovereign retained-mode motion graphics interface for EffectCraft built on Martensite.

pub mod command_reg;
pub mod keyframes;
pub mod menus;
pub mod theme;

pub struct EffectcraftApp {
    pub current_frame: u64,
    pub comp_duration: u64,
    pub is_rendering: bool,
}

impl EffectcraftApp {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            comp_duration: 300,
            is_rendering: false,
        }
    }

    pub fn seek_to(&mut self, frame: u64) {
        self.current_frame = frame.min(self.comp_duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effectcraft_app() {
        let mut app = EffectcraftApp::new();
        assert_eq!(app.current_frame, 0);
        app.seek_to(150);
        assert_eq!(app.current_frame, 150);
    }
}

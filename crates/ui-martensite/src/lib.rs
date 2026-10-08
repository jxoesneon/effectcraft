//! Sovereign retained-mode interface for EffectCraft built on the Martensite GUI engine.

pub mod command_reg;
pub mod keyframes;
pub mod menus;
pub mod shortcuts;
pub mod theme;
pub mod widgets;

use effectcraft_engine::Engine;
use std::sync::{Arc, Mutex};

/// Application state container managing the Martensite GUI pipeline.
pub struct EffectcraftApp {
    pub engine: Arc<Mutex<Engine>>,
    pub theme: theme::CraftTheme,
    pub keyboard: shortcuts::KeyboardEngine,
    pub active_tool: effectcraft_engine::Tool,
    pub current_frame: u64,
    pub comp_duration: u64,
    pub is_rendering: bool,
    pub zoom_level: f32,
    pub pan_offset: [f32; 2],
    pub rulers_visible: bool,
    pub snap_to_guides: bool,
    pub is_dirty: bool,
}

impl EffectcraftApp {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            theme: theme::CraftTheme::vfx_dark(),
            keyboard: shortcuts::KeyboardEngine::new(),
            active_tool: effectcraft_engine::Tool::Selection,
            current_frame: 0,
            comp_duration: 300,
            is_rendering: false,
            zoom_level: 1.0,
            pan_offset: [0.0, 0.0],
            rulers_visible: true,
            snap_to_guides: true,
            is_dirty: false,
        }
    }

    pub fn set_tool(&mut self, tool: effectcraft_engine::Tool) {
        self.active_tool = tool;
    }

    pub fn seek_to(&mut self, frame: u64) {
        self.current_frame = frame.min(self.comp_duration);
    }

    pub fn step_frames(&mut self, delta: i64) {
        let next = self.current_frame as i128 + delta as i128;
        self.current_frame = next.clamp(0, self.comp_duration as i128) as u64;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.clamp(0.01, 64.0);
    }

    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan_offset[0] += dx;
        self.pan_offset[1] += dy;
    }

    pub fn reset_view(&mut self) {
        self.zoom_level = 1.0;
        self.pan_offset = [0.0, 0.0];
    }

    pub fn toggle_rulers(&mut self) -> bool {
        self.rulers_visible = !self.rulers_visible;
        self.rulers_visible
    }

    pub fn toggle_snapping(&mut self) -> bool {
        self.snap_to_guides = !self.snap_to_guides;
        self.snap_to_guides
    }

    pub fn set_rendering(&mut self, rendering: bool) {
        self.is_rendering = rendering;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let engine = Engine::new();
        let app = EffectcraftApp::new(engine);
        assert_eq!(app.active_tool, effectcraft_engine::Tool::Selection);
        assert_eq!(app.current_frame, 0);
        assert_eq!(app.comp_duration, 300);
        assert!(!app.is_rendering);
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
        assert!(app.rulers_visible);
        assert!(app.snap_to_guides);
        assert!(!app.is_dirty);
    }

    #[test]
    fn test_seek_and_step() {
        let engine = Engine::new();
        let mut app = EffectcraftApp::new(engine);

        app.seek_to(150);
        assert_eq!(app.current_frame, 150);

        app.seek_to(10_000);
        assert_eq!(app.current_frame, 300); // Clamped to comp duration

        app.seek_to(10);
        app.step_frames(5);
        assert_eq!(app.current_frame, 15);
        app.step_frames(-100);
        assert_eq!(app.current_frame, 0);
    }

    #[test]
    fn test_zoom_clamping() {
        let engine = Engine::new();
        let mut app = EffectcraftApp::new(engine);

        app.set_zoom(2.5);
        assert_eq!(app.zoom_level, 2.5);

        app.set_zoom(0.0001);
        assert_eq!(app.zoom_level, 0.01);

        app.set_zoom(1000.0);
        assert_eq!(app.zoom_level, 64.0);
    }

    #[test]
    fn test_pan_and_reset() {
        let engine = Engine::new();
        let mut app = EffectcraftApp::new(engine);

        app.pan_by(120.0, -45.0);
        assert_eq!(app.pan_offset, [120.0, -45.0]);

        app.set_zoom(3.0);
        app.reset_view();
        assert_eq!(app.zoom_level, 1.0);
        assert_eq!(app.pan_offset, [0.0, 0.0]);
    }

    #[test]
    fn test_toggles() {
        let engine = Engine::new();
        let mut app = EffectcraftApp::new(engine);

        assert!(app.rulers_visible);
        assert!(!app.toggle_rulers());
        assert!(!app.rulers_visible);
        assert!(app.toggle_rulers());

        assert!(app.snap_to_guides);
        assert!(!app.toggle_snapping());
        assert!(!app.snap_to_guides);
        assert!(app.toggle_snapping());

        app.set_rendering(true);
        assert!(app.is_rendering);
        app.set_rendering(false);
        assert!(!app.is_rendering);
    }
}

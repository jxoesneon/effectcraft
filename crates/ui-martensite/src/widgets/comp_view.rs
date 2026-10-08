//! Composition view widget with subpixel pan/zoom, rulers, and safe-title guides.

pub struct CompViewWidget {
    pub zoom: f32,
    pub pan_offset: [f32; 2],
    pub comp_size: [u32; 2],
    pub rulers_visible: bool,
    pub safe_margins_visible: bool,
    pub checkerboard_opacity: f32,
}

impl CompViewWidget {
    pub fn new(width: u32, height: u32) -> Self {
        Self { zoom: 1.0, pan_offset: [0.0, 0.0], comp_size: [width, height], rulers_visible: true, safe_margins_visible: false, checkerboard_opacity: 0.0 }
    }

    pub fn zoom_at(&mut self, factor: f32, cursor: [f32; 2]) {
        let old_zoom = self.zoom;
        let new_zoom = (self.zoom * factor).clamp(0.01, 64.0);
        let ratio = new_zoom / old_zoom;

        self.pan_offset[0] = cursor[0] - (cursor[0] - self.pan_offset[0]) * ratio;
        self.pan_offset[1] = cursor[1] - (cursor[1] - self.pan_offset[1]) * ratio;
        self.zoom = new_zoom;
    }

    pub fn set_checkerboard_opacity(&mut self, opacity: f32) {
        self.checkerboard_opacity = opacity.clamp(0.0, 1.0);
    }

    pub fn screen_to_comp(&self, screen: [f32; 2]) -> [f32; 2] {
        [(screen[0] - self.pan_offset[0]) / self.zoom, (screen[1] - self.pan_offset[1]) / self.zoom]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comp_coordinates_and_zoom() {
        let mut view = CompViewWidget::new(1920, 1080);
        assert_eq!(view.screen_to_comp([100.0, 100.0]), [100.0, 100.0]);

        view.zoom_at(2.0, [0.0, 0.0]);
        assert_eq!(view.zoom, 2.0);
        assert_eq!(view.screen_to_comp([100.0, 100.0]), [50.0, 50.0]);

        view.set_checkerboard_opacity(2.0);
        assert_eq!(view.checkerboard_opacity, 1.0);
    }
}

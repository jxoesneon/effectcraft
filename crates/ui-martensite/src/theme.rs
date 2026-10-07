//! VFX dark theme tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub comp_bg: Color,
    pub keyframe_diamond: Color,
}

impl Theme {
    pub fn vfx_studio() -> Self {
        Self {
            comp_bg: Color(18, 20, 24),
            keyframe_diamond: Color(255, 200, 0),
        }
    }
}

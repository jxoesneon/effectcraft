//! Keyframe Bézier interpolation state.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InterpolationType {
    Linear,
    Bezier,
    Hold,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Keyframe {
    pub frame: u64,
    pub value: f32,
    pub interpolation: InterpolationType,
}

pub struct KeyframeTrack {
    pub property_name: String,
    pub keyframes: Vec<Keyframe>,
}

impl KeyframeTrack {
    pub fn new(property: &str) -> Self {
        Self { property_name: property.to_string(), keyframes: Vec::new() }
    }

    pub fn insert_keyframe(&mut self, frame: u64, value: f32) {
        self.keyframes.push(Keyframe { frame, value, interpolation: InterpolationType::Bezier });
        self.keyframes.sort_by_key(|k| k.frame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframes() {
        let mut track = KeyframeTrack::new("Position.X");
        track.insert_keyframe(10, 100.0);
        track.insert_keyframe(0, 0.0);
        assert_eq!(track.keyframes[0].frame, 0);
        assert_eq!(track.keyframes[1].frame, 10);
    }
}

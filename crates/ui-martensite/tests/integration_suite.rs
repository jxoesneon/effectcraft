//! VFX integration tests.

use effectcraft_ui_martensite::{EffectcraftApp, keyframes::KeyframeTrack};

#[test]
fn test_vfx_workflow() {
    let mut app = EffectcraftApp::new();
    let mut track = KeyframeTrack::new("Opacity");
    track.insert_keyframe(0, 0.0);
    track.insert_keyframe(60, 100.0);
    assert_eq!(track.keyframes.len(), 2);
    app.seek_to(30);
    assert_eq!(app.current_frame, 30);
}

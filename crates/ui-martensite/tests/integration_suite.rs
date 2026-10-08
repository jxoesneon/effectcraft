//! Comprehensive integration test suite for EffectCraft Martensite UI.
//!
//! Validates end-to-end integration across commands, menus, shortcuts,
//! scrubby inputs, graph editor velocity handles, keyframe tracks, and
//! composition-viewer coordinates.

use effectcraft_engine::{Engine, Tool};
use effectcraft_ui_martensite::{
    EffectcraftApp,
    command_reg::{COMMAND_REGISTRY, find_command},
    keyframes::KeyframeTrack,
    menus::generate_main_menu,
    theme::CraftTheme,
    widgets::{
        CompViewWidget, DockPanelGroup, EffectControlsWidget, GraphEditorWidget, GraphMode, LayerItemDef, LayerTreeWidget, ScrubbyInputWidget, ToolStripWidget,
        VelocityHandle,
    },
};

#[test]
fn test_end_to_end_workspace_interaction() {
    let engine = Engine::new();
    let mut app = EffectcraftApp::new(engine);

    // 1. Initial State Verification
    assert_eq!(app.active_tool, Tool::Selection);
    assert_eq!(app.zoom_level, 1.0);
    assert!(app.rulers_visible);
    assert!(!app.is_rendering);

    // 2. Keystroke Workflow: Switch to Pen, zoom in, hold Space to pan
    let new_tool = app.keyboard.on_key_down("g", app.active_tool);
    assert_eq!(new_tool, Some(Tool::Pen));
    app.set_tool(Tool::Pen);

    app.set_zoom(2.0);
    assert_eq!(app.zoom_level, 2.0);

    // Spring-loaded Hand tool
    let hand_tool = app.keyboard.on_key_down("Space", app.active_tool);
    assert_eq!(hand_tool, Some(Tool::Hand));
    app.set_tool(Tool::Hand);

    app.pan_by(50.0, 100.0);
    assert_eq!(app.pan_offset, [50.0, 100.0]);

    // Release Space restores Pen
    let restored_tool = app.keyboard.on_key_up("Space");
    assert_eq!(restored_tool, Some(Tool::Pen));
    app.set_tool(Tool::Pen);

    // 3. Playhead Navigation over the Composition
    app.seek_to(120);
    assert_eq!(app.current_frame, 120);
    app.step_frames(-20);
    assert_eq!(app.current_frame, 100);

    // 4. Effect Controls Interaction for an applied blur
    let mut controls = EffectControlsWidget::new("Gaussian Blur");
    controls.set_tool(Tool::Selection);
    controls.blurriness.on_pointer_down(0.0);
    controls.blurriness.on_pointer_move(20.0, false, false);
    controls.blurriness.on_pointer_up();
    assert_eq!(controls.blurriness.value, 20.0);
    assert!(controls.toggle_keyframes());

    // 5. Timeline Layer Stack & Switches
    let mut layers = LayerTreeWidget::new();
    layers.layers.push(LayerItemDef {
        id: 1,
        name: "Background Solid".to_string(),
        video: true,
        audio: false,
        solo: false,
        locked: true,
        shy: false,
        is_adjustment: false,
        expanded: false,
        children: vec![],
    });
    layers.layers.push(LayerItemDef {
        id: 2,
        name: "Title Text".to_string(),
        video: true,
        audio: false,
        solo: false,
        locked: false,
        shy: true,
        is_adjustment: false,
        expanded: false,
        children: vec![],
    });
    layers.select_layer(2);
    assert_eq!(layers.selected_layer_id, Some(2));
    layers.toggle_video(2);
    assert!(!layers.layers[1].video);
    layers.shy_guy_enabled = true;
    assert_eq!(layers.visible_layers(), vec![1]);

    // 6. Keyframe Track & Graph Editor Ease Handles
    let mut track = KeyframeTrack::new("Opacity");
    track.insert_keyframe(0, 0.0);
    track.insert_keyframe(60, 100.0);
    assert_eq!(track.keyframes.len(), 2);
    assert_eq!(track.keyframes[1].frame, 60);

    let mut graph = GraphEditorWidget::new();
    graph.select_keyframe(1);
    graph.apply_easy_ease();
    assert!(graph.handle.linked);
    graph.handle.set_outgoing(75.0, true);
    assert!(graph.handle.is_split());
    graph.set_mode(GraphMode::SpeedGraph);
    assert_eq!(graph.mode, GraphMode::SpeedGraph);

    let mut scrubby = ScrubbyInputWidget::new("Influence", 33.0, 0.0, 100.0, "%");
    scrubby.on_pointer_down(0.0);
    scrubby.on_pointer_move(10.0, false, false);
    assert_eq!(scrubby.value, 43.0);

    let mut handle = VelocityHandle::linear();
    handle.set_incoming(50.0, false);
    assert_eq!(handle.outgoing, 50.0);

    // 7. Tool Strip Flyouts
    let mut strip = ToolStripWidget::new();
    assert_eq!(strip.active_tool, Tool::Selection);
    strip.open_flyout_at(8);
    assert!(strip.pick_from_flyout(Tool::CloneStamp));
    assert_eq!(strip.active_tool, Tool::CloneStamp);

    // 8. Docking System Validation
    let mut dock = DockPanelGroup::new(&["Project", "Effect Controls", "Render Queue"]);
    dock.select_tab(2);
    assert_eq!(dock.active_tab, 2);
    dock.toggle_collapsed();
    assert!(dock.collapsed_to_icons);

    // 8. Composition Viewer Coordinates
    let mut viewer = CompViewWidget::new(1920, 1080);
    viewer.zoom_at(2.0, [0.0, 0.0]);
    assert_eq!(viewer.screen_to_comp([100.0, 100.0]), [50.0, 50.0]);

    // 9. Menu Generation Consistency
    let menus = generate_main_menu();
    assert!(!menus.is_empty());
    for menu in &menus {
        for item in &menu.items {
            if let Some(cmd_id) = item.command_id {
                assert!(find_command(cmd_id).is_some(), "Unknown command in menu: {}", cmd_id);
            }
        }
    }

    // 10. Command Registry Sanity
    assert!(!COMMAND_REGISTRY.is_empty());
    assert!(find_command("comp.new").is_some());
    assert!(find_command("bogus.command").is_none());

    // 11. Theme Color Space Consistency
    let theme = CraftTheme::vfx_dark();
    let obsidian = CraftTheme::studio_obsidian();
    assert_ne!(theme.surface_app_bg, obsidian.surface_app_bg);
    assert!(theme.text_primary.luminance() > theme.surface_panel.luminance());
}

//! Decoupled command catalog and taxonomy for EffectCraft.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    File,
    Edit,
    Composition,
    Layer,
    Effect,
    Animation,
    View,
    Window,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub category: CommandCategory,
    pub default_shortcut: Option<&'static str>,
    pub secondary_shortcut: Option<&'static str>,
}

pub const COMMAND_REGISTRY: &[CommandSpec] = &[
    // File
    CommandSpec {
        id: "file.newProject",
        label: "New Project",
        category: CommandCategory::File,
        default_shortcut: Some("Ctrl+Alt+N"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "file.open", label: "Open Project…", category: CommandCategory::File, default_shortcut: Some("Ctrl+O"), secondary_shortcut: None },
    CommandSpec { id: "file.save", label: "Save", category: CommandCategory::File, default_shortcut: Some("Ctrl+S"), secondary_shortcut: None },
    CommandSpec { id: "file.saveAs", label: "Save As…", category: CommandCategory::File, default_shortcut: Some("Ctrl+Shift+S"), secondary_shortcut: None },
    CommandSpec { id: "file.import", label: "Import File…", category: CommandCategory::File, default_shortcut: Some("Ctrl+I"), secondary_shortcut: None },
    // Edit
    CommandSpec { id: "edit.undo", label: "Undo", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.redo", label: "Redo", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+Shift+Z"), secondary_shortcut: None },
    CommandSpec { id: "edit.cut", label: "Cut", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+X"), secondary_shortcut: None },
    CommandSpec { id: "edit.copy", label: "Copy", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+C"), secondary_shortcut: None },
    CommandSpec { id: "edit.paste", label: "Paste", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+V"), secondary_shortcut: None },
    CommandSpec { id: "edit.duplicate", label: "Duplicate", category: CommandCategory::Edit, default_shortcut: Some("Ctrl+D"), secondary_shortcut: None },
    CommandSpec {
        id: "edit.splitLayer",
        label: "Split Layer",
        category: CommandCategory::Edit,
        default_shortcut: Some("Ctrl+Shift+D"),
        secondary_shortcut: None,
    },
    // Composition
    CommandSpec {
        id: "comp.new",
        label: "New Composition…",
        category: CommandCategory::Composition,
        default_shortcut: Some("Ctrl+N"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "comp.settings",
        label: "Composition Settings…",
        category: CommandCategory::Composition,
        default_shortcut: Some("Ctrl+K"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "comp.trimToWorkArea",
        label: "Trim Comp to Work Area",
        category: CommandCategory::Composition,
        default_shortcut: Some("Ctrl+Shift+X"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "comp.saveFrameAs",
        label: "Save Frame As",
        category: CommandCategory::Composition,
        default_shortcut: Some("Ctrl+Alt+S"),
        secondary_shortcut: None,
    },
    // Layer
    CommandSpec { id: "layer.newSolid", label: "New Solid…", category: CommandCategory::Layer, default_shortcut: Some("Ctrl+Y"), secondary_shortcut: None },
    CommandSpec {
        id: "layer.newText",
        label: "New Text Layer",
        category: CommandCategory::Layer,
        default_shortcut: Some("Ctrl+Alt+Shift+T"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "layer.newShape", label: "New Shape Layer", category: CommandCategory::Layer, default_shortcut: None, secondary_shortcut: None },
    CommandSpec {
        id: "layer.newNull",
        label: "New Null Object",
        category: CommandCategory::Layer,
        default_shortcut: Some("Ctrl+Alt+Shift+Y"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "layer.newAdjustment",
        label: "New Adjustment Layer",
        category: CommandCategory::Layer,
        default_shortcut: Some("Ctrl+Alt+Y"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "layer.precompose",
        label: "Pre-compose…",
        category: CommandCategory::Layer,
        default_shortcut: Some("Ctrl+Shift+C"),
        secondary_shortcut: None,
    },
    // Effect
    CommandSpec {
        id: "effect.apply",
        label: "Apply Effect…",
        category: CommandCategory::Effect,
        default_shortcut: Some("Ctrl+Alt+Shift+E"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "effect.removeAll",
        label: "Remove All Effects",
        category: CommandCategory::Effect,
        default_shortcut: Some("Ctrl+Shift+E"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "effect.toggle", label: "Toggle Effect", category: CommandCategory::Effect, default_shortcut: None, secondary_shortcut: None },
    // Animation
    CommandSpec { id: "time.nextKey", label: "Next Keyframe", category: CommandCategory::Animation, default_shortcut: Some("K"), secondary_shortcut: None },
    CommandSpec {
        id: "time.previousKey",
        label: "Previous Keyframe",
        category: CommandCategory::Animation,
        default_shortcut: Some("J"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "layer.timeStretch",
        label: "Time Stretch…",
        category: CommandCategory::Animation,
        default_shortcut: Some("Ctrl+Alt+R"),
        secondary_shortcut: None,
    },
    CommandSpec {
        id: "layer.enableTimeRemap",
        label: "Enable Time Remapping",
        category: CommandCategory::Animation,
        default_shortcut: Some("Ctrl+Alt+T"),
        secondary_shortcut: None,
    },
    // View
    CommandSpec { id: "view.zoomIn", label: "Zoom In", category: CommandCategory::View, default_shortcut: Some("."), secondary_shortcut: None },
    CommandSpec { id: "view.zoomOut", label: "Zoom Out", category: CommandCategory::View, default_shortcut: Some(","), secondary_shortcut: None },
    CommandSpec {
        id: "view.rulers",
        label: "Show Rulers",
        category: CommandCategory::View,
        default_shortcut: Some("Ctrl+Shift+Alt+R"),
        secondary_shortcut: None,
    },
    CommandSpec { id: "view.guides", label: "Show Guides", category: CommandCategory::View, default_shortcut: Some("Ctrl+;"), secondary_shortcut: None },
    CommandSpec { id: "view.fullScreen", label: "Full Screen", category: CommandCategory::View, default_shortcut: Some("Ctrl+\\"), secondary_shortcut: None },
];

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().find(|cmd| cmd.id == id)
}

pub fn commands_by_category(category: CommandCategory) -> Vec<&'static CommandSpec> {
    COMMAND_REGISTRY.iter().filter(|cmd| cmd.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_command_ids_are_unique() {
        let mut ids = HashSet::new();
        for cmd in COMMAND_REGISTRY {
            assert!(ids.insert(cmd.id), "Duplicate command ID detected: {}", cmd.id);
        }
    }

    #[test]
    fn test_lookup_finds_all_commands() {
        for cmd in COMMAND_REGISTRY {
            let found = find_command(cmd.id);
            assert!(found.is_some());
            assert_eq!(found.unwrap().label, cmd.label);
        }
    }

    #[test]
    fn test_categories_populated() {
        assert!(!commands_by_category(CommandCategory::File).is_empty());
        assert!(!commands_by_category(CommandCategory::Edit).is_empty());
        assert!(!commands_by_category(CommandCategory::Composition).is_empty());
        assert!(!commands_by_category(CommandCategory::Layer).is_empty());
        assert!(!commands_by_category(CommandCategory::Effect).is_empty());
        assert!(!commands_by_category(CommandCategory::Animation).is_empty());
        assert!(!commands_by_category(CommandCategory::View).is_empty());
    }
}

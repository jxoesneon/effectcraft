//! VFX command registry.

pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
}

pub const COMMANDS: &[Command] = &[
    Command { id: "comp.new", label: "New Composition…" },
    Command { id: "render.add_queue", label: "Add to Render Queue" },
];

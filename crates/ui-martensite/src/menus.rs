//! VFX menus.

pub struct MenuCategory {
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub const MENUS: &[MenuCategory] = &[
    MenuCategory { title: "Composition", items: &["comp.new", "render.add_queue"] },
];

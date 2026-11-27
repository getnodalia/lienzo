//! Every screen draws its text in the interface language. Panels and the dialog behind every menu
//! command are drawn in Spanish; a text that is still the English key of a Spanish catalog row
//! was drawn without going through the catalog.
use std::collections::{BTreeMap, BTreeSet};

use egui::epaint::Shape;
use egui_kittest::{Harness, kittest::NodeT as _};
use lienzo_ui_egui::{LienzoApp, i18n, menus, theme::ThemeKind};
use serde_json::json;

/// Names of the things the app ships or makes, drawn as stored: a document, a layer, a swatch, a
/// gradient, a pattern, a shape, a style, a brush, an action, a tool preset, a channel or a
/// character style is data the person can rename or replace, so it is not interface text.
const DATA: &[&str] = &[
    "Background",
    "Untitled",
    "Quick Mask",
    "None",
    // Swatches
    "50% Gray",
    "Black",
    "Blue",
    "Cyan",
    "Dark",
    "Green",
    "Orange",
    "Red",
    "Violet",
    "White",
    "Yellow",
    // Brushes, gradients, patterns, shapes, styles and tool presets
    "Black, White",
    "Blues",
    "Charcoal",
    "Cross",
    "Diamond",
    "Dots",
    "Drop Shadow",
    "Emboss",
    "Foreground to Background",
    "Greens",
    "Grid",
    "Heart",
    "Inner Shadow",
    "Outer Glow",
    "Radial Gradient",
    "Reds",
    "Ring",
    "Sparkle",
    "Spatter",
    "Star",
    "Symbols",
];

/// The default actions the Actions panel lists: each is the name of an action, which is data.
const ACTIONS: &[&str] = &[
    "Add Layer Mask",
    "Add Vector Mask",
    "Arrange",
    "Clear Guides",
    "Convert to Smart Object",
    "Create Clipping Mask",
    "Delete Layer Mask",
    "Delete Vector Mask",
    "Deselect",
    "Flatten Image",
    "Focus Area",
    "Group Layers",
    "Hide All Effects",
    "Lock Layers",
    "Merge Visible",
    "New Blank Video Layer",
    "New Type Layer",
    "Open",
    "Rasterize Fill Content",
    "Rasterize Type",
    "Remove Black Matte",
    "Remove White Matte",
    "Select All",
    "Update All Modified Content",
];

/// Texts whose draw site is still to be found. Each is named for assistive technology in English
/// while the visible text is translated (or is drawn by a site this walk could not trace). They are
/// bugs, never a place to add to: remove one when its site goes through `tl!`.
const KNOWN: &[&str] = &["Close", "H", "Proof Setup", "Selection", "Type Layer", "W"];

fn harness() -> Harness<'static, LienzoApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1440.0, 1000.0)).build_eframe(|cc| {
        LienzoApp::setup_context(&cc.egui_ctx, ThemeKind::Pro);
        let mut app = LienzoApp::new(lienzo_engine::Session::new(), Default::default());
        app.background_jobs = false;
        app.run("file.new", json!({"width": 64, "height": 48})).ok();
        app.run("layer.new.layer", json!({"name": "Plate A"})).ok();
        app.run("edit.fill", json!({"color": "#80ff00"})).ok();
        app.run("layer.new.layer", json!({"name": "Plate B"})).ok();
        app
    });
    h.state_mut().session.prefs.edit(|p| p.interface.language = "es".into());
    h.run_steps(4);
    h
}

fn collect_shape(shape: &Shape, out: &mut BTreeMap<String, String>) {
    match shape {
        Shape::Text(t) => {
            for line in t.galley.text().split('\n') {
                let line = line.trim();
                if !line.is_empty() {
                    out.entry(line.to_owned()).or_insert_with(|| "painted".to_owned());
                }
            }
        }
        Shape::Vec(v) => v.iter().for_each(|s| collect_shape(s, out)),
        _ => {}
    }
}

/// Everything the frame names for assistive technology or paints as text, with how it is shown.
fn shown(h: &Harness<'_, LienzoApp>) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for node in h.root().children_recursive() {
        let n = node.accesskit_node();
        // What a person typed into a field is theirs, not the interface's.
        let is_field = |role| matches!(role, egui::accesskit::Role::TextInput | egui::accesskit::Role::MultilineTextInput);
        if n.parent().is_some_and(|p| is_field(p.role())) {
            continue;
        }
        let value = if is_field(n.role()) { None } else { n.value() };
        for text in [n.label(), value].into_iter().flatten() {
            let text = text.trim();
            if !text.is_empty() {
                out.entry(text.to_owned()).or_insert_with(|| {
                    let parent = n.parent().map(|p| format!(" in {:?} {:?}", p.role(), p.label().unwrap_or_default())).unwrap_or_default();
                    format!("{:?}{parent}", n.role())
                });
            }
        }
    }
    for clipped in &h.output().shapes {
        collect_shape(&clipped.shape, &mut out);
    }
    out
}

/// Texts of the frame that are English keys of a Spanish row with another reading.
fn untranslated(frame: &BTreeMap<String, String>) -> Vec<String> {
    let es = i18n::Lang::from_code("es").expect("Spanish is registered");
    frame
        .iter()
        .filter(|(t, _)| !DATA.contains(&t.as_str()) && !ACTIONS.contains(&t.as_str()) && !KNOWN.contains(&t.as_str()))
        .filter(|(t, _)| i18n::has(es, t) && i18n::tr(es, t) != t.as_str())
        .map(|(t, kind)| format!("{t:?} ({kind})"))
        .collect()
}

fn dismiss(h: &mut Harness<'static, LienzoApp>) {
    for _ in 0..3 {
        let ids: Vec<u64> = h.state().ui.dialogs.iter().map(|d| d.id).collect();
        for id in ids {
            let _ = lienzo_ui_egui::dialogs::cancel(h.state_mut(), id);
        }
        h.key_press(egui::Key::Escape);
        h.run_steps(2);
    }
}

/// Commands that leave the window or reshape it: they open no dialog of their own, and floating a
/// document into a window of its own trips egui's layer assertion in a test harness.
fn skipped(id: &str) -> bool {
    id.starts_with("window.arrange") || id.contains("quit") || id.contains("exit") || id == "file.close" || id == "file.closeAll"
}

fn walk() -> Vec<String> {
    let _gpu = lienzo_testkit::gpu_lock();
    #[cfg(target_os = "linux")]
    assert!(lienzo_ui_egui::cjk_fonts::wait_for_fontconfig_cjk_scan(), "Fontconfig CJK lookup did not finish");
    let mut failures: Vec<String> = Vec::new();
    let mut note = |what: &str, frame: &BTreeMap<String, String>, seen: &mut BTreeSet<String>| {
        for line in untranslated(frame) {
            if seen.insert(line.clone()) {
                failures.push(format!("{what}: {line}"));
            }
        }
    };
    let mut seen = BTreeSet::new();
    let mut h = harness();
    note("main window", &shown(&h), &mut seen);
    let ids: Vec<(String, String)> = menus::menu_items(h.state()).into_iter().filter(|i| i.enabled).map(|i| (i.id, i.label)).collect();
    for (id, _) in ids {
        if skipped(&id) {
            continue;
        }
        let ctx = h.ctx.clone();
        let _ = menus::invoke(h.state_mut(), &ctx, &id, json!({}));
        h.run_steps(3);
        note(&format!("command {id}"), &shown(&h), &mut seen);
        dismiss(&mut h);
    }
    failures
}

/// A screen that draws a text the catalog translates, without going through the catalog, shows
/// English to everyone who runs Lienzo in Spanish.
#[test]
fn spanish_interface_has_no_untranslated_text() {
    let failures = walk();
    assert!(failures.is_empty(), "texts drawn in English although Spanish has them:\n{}", failures.join("\n"));
}

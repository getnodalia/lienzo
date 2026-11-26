//! Starter templates for File › New: finished, layered documents of our own design (posters,
//! postcards, wallpapers, covers and more) that open ready to edit.
//!
//! A template is a short script of ordinary engine commands (`shape.create`, `type.create`,
//! `layer.layerStyle.dropShadow`, …) run on a scratch session, so everything a template makes is
//! a real, editable layer: gradient and shape layers, live text, named layers. Nothing here is
//! copied from another product's templates: the compositions, colours and words are ours, set in
//! the two typefaces Lienzo bundles (Inter and Sora).
//!
//! Every template is written twice, in English and in Spanish, so one opened from a Spanish
//! interface arrives in Spanish. `id` is the English name in lower case with dashes: it is what
//! scripts pass to `file.newFromTemplate` whatever language the text is written in.
//!
//! The design is drawn in design pixels and multiplied by a scale, so the Home screen builds the
//! same template at thumbnail size without a second code path.

use lienzo_doc::Document;
use serde_json::{Value, json};

use crate::commands::CommandSpec;
use crate::presets::{always, bad};
use crate::{Result, Session};

mod designs;
mod names;

use designs::TEMPLATES;

const CMD: &str = "file.newFromTemplate";
/// The largest scale a template is built at (2 = twice its design size).
const MAX_SCALE: f64 = 2.0;
/// The smallest scale, so a thumbnail never degenerates into a point.
const MIN_SCALE: f64 = 0.02;

/// The language a template's text is written in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Lang {
    #[default]
    En,
    Es,
}

impl Lang {
    /// `"es"`, `"es-ES"` and `"es_MX.UTF-8"` are Spanish; anything else is English.
    pub fn parse(code: &str) -> Lang {
        let code = code.trim().to_ascii_lowercase();
        if code == "es" || code.starts_with("es-") || code.starts_with("es_") { Lang::Es } else { Lang::En }
    }
}

/// A group of templates in the gallery.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Category {
    pub id: &'static str,
    /// The English label; the interface translates it.
    pub label: &'static str,
}

/// The gallery's categories, in order.
pub const CATEGORIES: &[Category] =
    &[Category { id: "social", label: "Social" }, Category { id: "print", label: "Print & paper" }, Category { id: "art", label: "Web & art" }];

/// One starter template.
pub struct Template {
    /// The English name in lower case with dashes.
    pub id: &'static str,
    pub category: &'static str,
    /// The name in English (the interface translates it) and in Spanish.
    pub name: &'static str,
    pub name_es: &'static str,
    /// What it is for, in one sentence, in English and in Spanish.
    pub summary: &'static str,
    pub summary_es: &'static str,
    /// Canvas size in pixels at the design scale, and its resolution in pixels per inch.
    pub size: (u32, u32),
    pub ppi: f32,
    /// The colour under everything (what shows if the design's layers are removed).
    base: &'static str,
    build: fn(&mut Canvas) -> Result<()>,
}

impl Template {
    /// The template's name in `lang`.
    pub fn name(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => self.name,
            Lang::Es => self.name_es,
        }
    }

    /// The template's summary in `lang`.
    pub fn summary(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => self.summary,
            Lang::Es => self.summary_es,
        }
    }
}

/// Every starter template, in gallery order.
pub fn templates() -> &'static [Template] {
    TEMPLATES
}

/// The template called `name`: its id, or its name in either language, in any case.
pub fn find(name: &str) -> Option<&'static Template> {
    let key = name.trim().to_lowercase();
    let dashed = key.replace(' ', "-");
    TEMPLATES.iter().find(|t| t.id == dashed || t.name.to_lowercase() == key || t.name_es.to_lowercase() == key)
}

/// Builds the template `name` as a document of its own, at `scale` times its design size (1 is the
/// real thing, a small scale is a thumbnail). The document is unsaved and has no history, so Undo
/// does not take the template apart.
pub fn build(name: &str, lang: Lang, scale: f64) -> Result<Document> {
    let template = find(name).ok_or_else(|| bad(CMD, format!("no template called `{name}`; file.templates lists them")))?;
    if !scale.is_finite() || !(MIN_SCALE..=MAX_SCALE).contains(&scale) {
        return Err(bad(CMD, format!("`scale` must be between {MIN_SCALE} and {MAX_SCALE}")));
    }
    let k = scale as f32;
    let px = |v: u32| ((v as f32 * k).round() as u32).max(1);
    let mut session = Session::new();
    session.execute(
        "file.new",
        json!({
            "width": px(template.size.0),
            "height": px(template.size.1),
            "resolution": template.ppi,
            "background": template.base,
            "name": template.name(lang),
        }),
    )?;
    let mut canvas = Canvas { s: &mut session, k, pt: 72.0 / template.ppi.max(1.0), lang };
    (template.build)(&mut canvas)?;
    let state = session.active().ok_or_else(|| bad(CMD, "the template made no document"))?;
    let mut doc = state.doc.as_ref().clone();
    if lang == Lang::Es {
        // The Background `file.new` made is the one layer the template did not name itself.
        let background: Vec<_> = doc.layers.iter().filter(|l| doc.is_background(l.id)).map(|l| l.id).collect();
        for layer in doc.layers.iter_mut().filter(|l| background.contains(&l.id)) {
            layer.name = "Fondo".to_string();
        }
    }
    Ok(doc)
}

// ---------- the command ----------

fn new_from_template(s: &mut Session, p: &Value) -> Result<Value> {
    let name = p.get("template").and_then(Value::as_str).filter(|t| !t.trim().is_empty()).ok_or_else(|| bad(CMD, "`template` is required"))?;
    let lang = p.get("lang").and_then(Value::as_str).map(Lang::parse).unwrap_or_default();
    let scale = match p.get("scale") {
        None | Some(Value::Null) => 1.0,
        Some(v) => v.as_f64().ok_or_else(|| bad(CMD, "`scale` must be a number"))?,
    };
    let mut doc = build(name, lang, scale)?;
    if let Some(custom) = p.get("name").and_then(Value::as_str).filter(|n| !n.trim().is_empty()) {
        doc.name = custom.to_string();
    }
    let template = find(name).map(|t| t.id).unwrap_or_default();
    let index = s.add_document(doc, None);
    Ok(json!({ "document": index, "template": template }))
}

fn list_templates(_: &mut Session, p: &Value) -> Result<Value> {
    let lang = p.get("lang").and_then(Value::as_str).map(Lang::parse).unwrap_or_default();
    Ok(json!({
        "categories": CATEGORIES.iter().map(|c| json!({"id": c.id, "label": c.label})).collect::<Vec<_>>(),
        "templates": TEMPLATES.iter().map(|t| json!({
            "id": t.id,
            "category": t.category,
            "name": t.name(lang),
            "summary": t.summary(lang),
            "width": t.size.0,
            "height": t.size.1,
            "resolution": t.ppi,
        })).collect::<Vec<_>>(),
    }))
}

/// The `file.newFromTemplate` and `file.templates` command specs.
pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec {
            id: CMD,
            label: "New from Template…",
            menu: &["File"],
            shortcut: None,
            params: r##"{"template":id|name (see file.templates),"lang":"en"|"es"="en" (the language the template's text is written in),"scale":0.02..2=1 (a fraction builds a thumbnail-sized copy),"name":str?} → {document,template}"##,
            enabled: always,
            run: new_from_template,
            journal: true,
        },
        CommandSpec {
            id: "file.templates",
            label: "Starter Templates",
            menu: &[],
            shortcut: None,
            params: r##"{"lang":"en"|"es"="en"} → {categories:[{id,label}],templates:[{id,category,name,summary,width,height,resolution}]}"##,
            enabled: always,
            run: list_templates,
            journal: false,
        },
    ]
}

// ---------- the drawing kit ----------

/// A scratch document being drawn on, in design pixels.
struct Canvas<'a> {
    s: &'a mut Session,
    k: f32,
    /// Points per design pixel before the scale: type sizes are points, converted with the
    /// document's resolution, so a print template's pixels need this to land where drawn.
    pt: f32,
    lang: Lang,
}

/// How a shape is filled.
enum Paint {
    Solid(&'static str),
    /// A colour with an alpha in 0..=1.
    Alpha(&'static str, f32),
    /// Linear gradient between colour stops, at an angle in degrees.
    Linear(&'static [(f32, &'static str)], f32),
    None,
}

/// A line of text and how it is set.
struct Type {
    text: &'static str,
    /// Where the text box sits, in design pixels: x, y, width, height.
    at: [f32; 4],
    font: &'static str,
    weight: u16,
    size: f32,
    color: &'static str,
    align: &'static str,
    /// Letter-spacing in thousandths of an em.
    tracking: f32,
    /// Line spacing in points; 0 is automatic.
    leading: f32,
    caps: bool,
}

impl Type {
    const fn new(text: &'static str, at: [f32; 4], size: f32, color: &'static str) -> Type {
        Type { text, at, font: "Inter", weight: 500, size, color, align: "left", tracking: 0.0, leading: 0.0, caps: false }
    }
    const fn display(mut self) -> Type {
        self.font = "Sora";
        self.weight = 800;
        self
    }
    const fn weight(mut self, w: u16) -> Type {
        self.weight = w;
        self
    }
    const fn center(mut self) -> Type {
        self.align = "center";
        self
    }
    const fn tracked(mut self, t: f32) -> Type {
        self.tracking = t;
        self
    }
    const fn leading(mut self, l: f32) -> Type {
        self.leading = l;
        self
    }
    const fn caps(mut self) -> Type {
        self.caps = true;
        self
    }
}

fn rgb(hex: &str) -> [f32; 3] {
    let channel = |i: usize| hex.get(1 + i * 2..3 + i * 2).and_then(|h| u8::from_str_radix(h, 16).ok()).map_or(0.0, |v| f32::from(v) / 255.0);
    [channel(0), channel(1), channel(2)]
}

fn rgba(hex: &str, alpha: f32) -> Value {
    let c = rgb(hex);
    json!([c[0], c[1], c[2], alpha])
}

impl Paint {
    fn json(&self) -> Value {
        let stops = |list: &[(f32, &str)]| list.iter().map(|(t, c)| json!([t, c])).collect::<Vec<_>>();
        match self {
            Paint::Solid(c) => json!(c),
            Paint::Alpha(c, a) => rgba(c, *a),
            Paint::Linear(list, angle) => json!({"gradient": {"stops": stops(list), "angle": angle, "style": "linear"}}),
            Paint::None => Value::Null,
        }
    }
}

impl Canvas<'_> {
    /// Picks the English or the Spanish words.
    fn t(&self, en: &'static str, es: &'static str) -> &'static str {
        match self.lang {
            Lang::En => en,
            Lang::Es => es,
        }
    }

    /// A layer's name in the template's language.
    fn name(&self, english: &str) -> String {
        match self.lang {
            Lang::En => english.to_string(),
            Lang::Es => names::spanish(english).unwrap_or_else(|| english.to_string()),
        }
    }

    fn scaled(&self, r: [f32; 4]) -> Vec<f32> {
        r.iter().map(|v| v * self.k).collect()
    }

    /// Creates a shape layer and returns its id.
    fn shape(&mut self, name: &str, mut params: Value, paint: &Paint, stroke: Option<(f32, &'static str, f32)>) -> Result<u64> {
        params["name"] = json!(self.name(name));
        params["fill"] = paint.json();
        params["stroke"] = match stroke {
            Some((width, color, alpha)) => json!({"width": (width * self.k).max(0.5), "color": color, "opacity": (alpha * 100.0).round(), "align": "center"}),
            None => Value::Null,
        };
        let made = self.s.execute("shape.create", params)?;
        made.get("layer").and_then(Value::as_u64).ok_or_else(|| bad(CMD, "a template shape made no layer"))
    }

    fn rect(&mut self, name: &str, r: [f32; 4], paint: Paint) -> Result<u64> {
        let rect = self.scaled(r);
        self.shape(name, json!({"kind": "rect", "rect": rect}), &paint, None)
    }

    fn rounded(&mut self, name: &str, r: [f32; 4], radius: f32, paint: Paint) -> Result<u64> {
        let rect = self.scaled(r);
        self.shape(name, json!({"kind": "roundedRect", "rect": rect, "radii": radius * self.k}), &paint, None)
    }

    fn ellipse(&mut self, name: &str, r: [f32; 4], paint: Paint) -> Result<u64> {
        let rect = self.scaled(r);
        self.shape(name, json!({"kind": "ellipse", "rect": rect}), &paint, None)
    }

    /// A circle from its centre and radius.
    fn circle(&mut self, name: &str, cx: f32, cy: f32, r: f32, paint: Paint) -> Result<u64> {
        self.ellipse(name, [cx - r, cy - r, r * 2.0, r * 2.0], paint)
    }

    /// An outline circle from its centre and radius (`[x, y, r]`).
    fn ring(&mut self, name: &str, [cx, cy, r]: [f32; 3], width: f32, color: &'static str, alpha: f32) -> Result<u64> {
        let rect = self.scaled([cx - r, cy - r, r * 2.0, r * 2.0]);
        self.shape(name, json!({"kind": "ellipse", "rect": rect}), &Paint::None, Some((width, color, alpha)))
    }

    /// An outline rectangle.
    fn frame(&mut self, name: &str, r: [f32; 4], radius: f32, width: f32, color: &'static str, alpha: f32) -> Result<u64> {
        let rect = self.scaled(r);
        let params = json!({"kind": "roundedRect", "rect": rect, "radii": radius * self.k});
        self.shape(name, params, &Paint::None, Some((width, color, alpha)))
    }

    /// A soft shadow under a layer.
    fn shadow(&mut self, layer: u64, distance: f32, size: f32, opacity: u32, color: &'static str) -> Result<()> {
        self.s.execute("layer.select", json!({"layer": layer}))?;
        self.s.execute(
            "layer.layerStyle.dropShadow",
            json!({"color": color, "opacity": opacity, "blend": "multiply", "angle": 90, "useGlobalLight": false, "distance": distance * self.k, "size": size * self.k}),
        )?;
        Ok(())
    }

    /// A text layer in a box.
    fn text(&mut self, name: &str, t: Type) -> Result<u64> {
        let params = json!({
            "name": self.name(name),
            "text": t.text,
            "box": self.scaled(t.at),
            "font": t.font,
            "weight": t.weight,
            "size": t.size * self.k * self.pt,
            "color": t.color,
            "align": t.align,
            "tracking": t.tracking,
            "leading": if t.leading > 0.0 { json!(t.leading * self.k * self.pt) } else { json!("auto") },
            "caps": if t.caps { "all" } else { "normal" },
        });
        let made = self.s.execute("type.create", params)?;
        made.get("layer").and_then(Value::as_u64).ok_or_else(|| bad(CMD, "a template text made no layer"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer_names(doc: &Document) -> Vec<String> {
        doc.layers.iter().map(|l| l.name.clone()).collect()
    }

    #[test]
    fn every_template_builds_in_both_languages_with_the_same_layers() {
        for t in templates() {
            let en = build(t.id, Lang::En, 0.25).unwrap();
            let es = build(t.id, Lang::Es, 0.25).unwrap();
            assert_eq!(en.layers.len(), es.layers.len(), "{} differs between languages", t.id);
            assert!(en.layers.len() >= 6, "{} has only {} layers", t.id, en.layers.len());
            assert_eq!(en.name, t.name);
            assert_eq!(es.name, t.name_es);
            let want = (t.size.0 as f32 * 0.25).round() as u32;
            assert_eq!(en.size.width, want, "{} width", t.id);
            assert!((en.resolution_dpi - t.ppi).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn spanish_templates_name_every_layer_in_spanish() {
        for t in templates() {
            let en = build(t.id, Lang::En, 0.2).unwrap();
            let es = build(t.id, Lang::Es, 0.2).unwrap();
            for (e, s) in layer_names(&en).iter().zip(layer_names(&es)) {
                if e == "Background" {
                    assert_eq!(s, "Fondo", "{}: the Background layer", t.id);
                } else {
                    assert_eq!(Some(s), names::spanish(e), "{}: layer {e} has no Spanish name", t.id);
                }
            }
            assert!(layer_names(&en).iter().any(|n| n == "Background"), "{} lost its Background", t.id);
        }
    }

    #[test]
    fn templates_have_unique_ids_known_categories_and_text_in_both_languages() {
        let mut seen = std::collections::HashSet::new();
        for t in templates() {
            assert!(seen.insert(t.id), "duplicate id {}", t.id);
            assert!(CATEGORIES.iter().any(|c| c.id == t.category), "{} has an unknown category", t.id);
            assert_eq!(t.id, t.name.to_lowercase().replace(' ', "-"), "the id is the English name");
            assert!(!t.name_es.is_empty() && !t.summary_es.is_empty());
            assert_ne!(t.summary, t.summary_es);
        }
        for c in CATEGORIES {
            assert!(templates().iter().any(|t| t.category == c.id), "{} is empty", c.id);
        }
    }

    #[test]
    fn a_template_opens_as_an_unsaved_document_without_history() {
        let mut s = Session::new();
        let r = s.execute(CMD, json!({"template": "dusk-hills", "scale": 0.1, "lang": "es-ES"})).unwrap();
        assert_eq!(r["template"], "dusk-hills");
        let active = s.active().unwrap();
        assert_eq!(active.doc.name, "Colinas al atardecer");
        assert!(!s.execute("edit.undo", json!({})).is_ok_and(|v| v == json!(true)), "Undo must not take the template apart");
        // Found by its Spanish name too, in any case.
        s.execute(CMD, json!({"template": "tarjeta de VISITA", "scale": 0.2})).unwrap();
        assert_eq!(s.documents().len(), 2);
    }

    #[test]
    fn text_is_live_text_in_the_chosen_language() {
        let en = build("quiet-quote", Lang::En, 0.3).unwrap();
        let es = build("quiet-quote", Lang::Es, 0.3).unwrap();
        let text = |d: &Document, layer: &str| {
            d.layers
                .iter()
                .find_map(|l| match &l.content {
                    lienzo_doc::LayerContent::Text(t) if l.name == layer => Some(t.text.clone()),
                    _ => None,
                })
                .unwrap_or_default()
        };
        assert!(text(&en, "Quote").starts_with("Slow down"));
        assert!(text(&es, "Cita").starts_with("Ve despacio"));
    }

    #[test]
    fn hostile_params_are_errors_not_panics() {
        let mut s = Session::new();
        for params in [
            json!({}),
            json!({"template": ""}),
            json!({"template": 7}),
            json!({"template": "no-such-template"}),
            json!({"template": "dusk-hills", "scale": 0}),
            json!({"template": "dusk-hills", "scale": -1}),
            json!({"template": "dusk-hills", "scale": 1e30}),
            json!({"template": "dusk-hills", "scale": "big"}),
            json!({"template": "dusk-hills", "scale": 2.5}),
            Value::Null,
        ] {
            assert!(s.execute(CMD, params.clone()).is_err(), "{params}");
        }
        assert!(s.documents().is_empty(), "a refused template leaves no document behind");
    }

    #[test]
    fn the_listing_names_every_template() {
        let mut s = Session::new();
        let r = s.execute("file.templates", json!({"lang": "es"})).unwrap();
        assert_eq!(r["templates"].as_array().unwrap().len(), templates().len());
        assert_eq!(r["templates"][0]["name"], "Cita tranquila");
        assert_eq!(r["categories"].as_array().unwrap().len(), CATEGORIES.len());
    }

    /// Writes every template as a PPM into `$LIENZO_TEMPLATE_DUMP` (a design aid, not a test):
    /// `LIENZO_TEMPLATE_DUMP=/tmp/t cargo test -p lienzo-engine dump_templates -- --ignored`.
    #[test]
    #[ignore = "writes files for a person to look at"]
    fn dump_templates() {
        let Some(dir) = std::env::var_os("LIENZO_TEMPLATE_DUMP") else { return };
        let dir = std::path::PathBuf::from(dir);
        std::fs::create_dir_all(&dir).unwrap();
        let scale: f64 = std::env::var("LIENZO_TEMPLATE_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(0.5);
        for lang in [Lang::En, Lang::Es] {
            for t in templates() {
                let doc = build(t.id, lang, scale).unwrap();
                let img = lienzo_compose::thumbnail(&doc, 4000);
                let mut out = format!("P6\n{} {}\n255\n", img.width, img.height).into_bytes();
                for px in img.pixels.as_chunks::<4>().0 {
                    // Over white, so a transparent edge is visible as such.
                    let a = u16::from(px[3]);
                    out.extend(px[..3].iter().map(|c| ((u16::from(*c) * a + 255 * (255 - a)) / 255) as u8));
                }
                let tag = if lang == Lang::En { "en" } else { "es" };
                std::fs::write(dir.join(format!("{}-{tag}.ppm", t.id)), out).unwrap();
            }
        }
    }
}

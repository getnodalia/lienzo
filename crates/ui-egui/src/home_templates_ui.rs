//! The Home screen's template gallery: finished, layered starting points (a poster, a postcard,
//! a wallpaper…) as a grid of cards that wraps to the width of the window. A card shows the
//! template drawn small, its name and its size; clicking it runs `file.newFromTemplate`.
//!
//! The templates themselves, their text in English and Spanish, and their thumbnails' source are
//! the engine's (`lienzo_engine::template_cmds`); this file only lays them out. Each thumbnail is
//! the template built at a small scale and composited once, one per frame, so opening Home never
//! stalls on the whole set.

use std::collections::HashMap;

use egui::{Color32, CornerRadius, FontId, Rect, Role, Sense, Stroke, StrokeKind, TextureHandle, TextureOptions, Ui, WidgetInfo, pos2, vec2};
use lienzo_engine::template_cmds::{self, Template};
use nodalia_theme::creator::{Look, home, layout};
use nodalia_theme::for_egui::color;
use serde_json::json;

use crate::LienzoApp;

/// The longest side of a cached thumbnail, in pixels.
const THUMB_SIDE: f32 = 384.0;
/// The narrowest a card gets before the grid drops a column.
const MIN_CARD: f32 = 188.0;
/// The widest a card gets when there is only room for one or two.
const MAX_CARD: f32 = 292.0;
const GAP: f32 = 16.0;
/// The preview well's height as a share of the card's width.
const WELL: f32 = 0.78;
/// Under the well: the name and the size.
const CAPTION: f32 = 58.0;

/// Thumbnails built so far, by template and by whether its text is Spanish. `None` is a template
/// that would not build: it is not tried again.
#[derive(Clone, Default)]
struct Thumbs(HashMap<(&'static str, bool), Option<TextureHandle>>);

fn spanish() -> bool {
    template_cmds::Lang::parse(crate::i18n::current().code()) == template_cmds::Lang::Es
}

fn lang() -> template_cmds::Lang {
    if spanish() { template_cmds::Lang::Es } else { template_cmds::Lang::En }
}

/// How many columns of cards fit `width`, and how wide each card is then.
pub(crate) fn columns(width: f32) -> (usize, f32) {
    let cols = (((width + GAP) / (MIN_CARD + GAP)).floor().max(1.0)) as usize;
    let card = ((width - GAP * (cols as f32 - 1.0)) / cols as f32).clamp(MIN_CARD.min(width.max(1.0)), MAX_CARD);
    (cols, card)
}

/// "6 × 4 in" for print sizes, "1080 × 1080 px" for screens.
pub(crate) fn size_label(t: &Template) -> String {
    let number = |v: f32| {
        let rounded = (v * 10.0).round() / 10.0;
        if (rounded - rounded.round()).abs() < f32::EPSILON { format!("{}", rounded.round() as i64) } else { format!("{rounded:.1}") }
    };
    let (w, h) = (t.size.0 as f32, t.size.1 as f32);
    if t.ppi >= 300.0 { format!("{} × {} in", number(w / t.ppi), number(h / t.ppi)) } else { format!("{} × {} px", t.size.0, t.size.1) }
}

/// The template's thumbnail, building one more per call when it is not cached yet.
fn thumbnail(ctx: &egui::Context, t: &'static Template, budget: &mut bool) -> Option<TextureHandle> {
    let key = (t.id, spanish());
    let id = egui::Id::unique("lienzo_template_thumbs");
    let mut thumbs: Thumbs = ctx.data(|d| d.get_temp(id)).unwrap_or_default();
    if let Some(known) = thumbs.0.get(&key) {
        return known.clone();
    }
    if !*budget {
        ctx.request_repaint();
        return None;
    }
    *budget = false;
    let scale = f64::from(THUMB_SIDE) / f64::from(t.size.0.max(t.size.1).max(1));
    let texture = template_cmds::build(t.id, lang(), scale).ok().map(|doc| {
        let img = lienzo_compose::thumbnail(&doc, THUMB_SIDE as u32);
        let image = egui::ColorImage::from_rgba_unmultiplied([img.width as usize, img.height as usize], &img.pixels);
        let opts = TextureOptions { magnification: egui::TextureFilter::Linear, minification: egui::TextureFilter::Linear, ..Default::default() };
        ctx.load_texture(format!("template-{}-{}", t.id, key.1), image, opts)
    });
    thumbs.0.insert(key, texture.clone());
    ctx.data_mut(|d| d.insert_temp(id, thumbs));
    ctx.request_repaint();
    texture
}

/// The category chips and the cards, under their heading. Returns nothing: a click runs the
/// command itself.
pub(crate) fn show(app: &mut LienzoApp, ui: &mut Ui, look: &Look) {
    home::section_heading(ui, look, tl!("Start from a template"));
    ui.add_space(10.0);
    category_chips(app, ui, look);
    ui.add_space(14.0);

    let chosen = app.ui.chrome.home_template_category.clone();
    let list: Vec<&'static Template> = template_cmds::templates().iter().filter(|t| chosen.is_empty() || t.category == chosen).collect();
    let (cols, card_w) = columns(ui.available_width());
    let mut budget = true;
    let mut picked = None;
    for row in list.chunks(cols) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = GAP;
            for t in row {
                if card(ui, look, t, card_w, &mut budget) {
                    picked = Some(t.id);
                }
            }
        });
        ui.add_space(GAP);
    }
    if let Some(id) = picked {
        // `run` reports a failure in the status bar; a template that built in the tests builds here.
        let _ = app.run("file.newFromTemplate", json!({"template": id, "lang": crate::i18n::current().code()}));
    }
}

fn category_chips(app: &mut LienzoApp, ui: &mut Ui, look: &Look) {
    let p = &look.palette;
    let mut chosen = app.ui.chrome.home_template_category.clone();
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
        let all = std::iter::once(("", tl!("All"))).chain(template_cmds::CATEGORIES.iter().map(|c| (c.id, tl!(c.label))));
        for (id, label) in all {
            let selected = chosen == id;
            let font = FontId::new(12.5, look.strong.clone());
            let ink = if selected { color(p.surface_panel) } else { color(p.text_secondary) };
            let galley = ui.painter().layout_no_wrap(label.to_owned(), font, ink);
            let (rect, resp) = ui.allocate_exact_size(vec2(galley.size().x + 26.0, layout::CONTROL_HEIGHT), Sense::click());
            resp.widget_info(|| WidgetInfo::selected(Role::Tab, true, selected, label));
            let fill = if selected {
                color(p.text_primary)
            } else if resp.hovered() {
                color(p.control_bg_hover)
            } else {
                color(p.control_field_bg)
            };
            ui.painter().rect_filled(rect, CornerRadius::same(layout::CONTROL_RADIUS), fill);
            ui.painter().galley(pos2(rect.left() + 13.0, rect.center().y - galley.size().y / 2.0), galley, ink);
            if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                chosen = id.to_string();
            }
        }
    });
    app.ui.chrome.home_template_category = chosen;
}

/// One card; true when it was clicked.
fn card(ui: &mut Ui, look: &Look, t: &'static Template, width: f32, budget: &mut bool) -> bool {
    let p = &look.palette;
    let well_h = (width * WELL).round();
    let (rect, resp) = ui.allocate_exact_size(vec2(width, well_h + CAPTION), Sense::click());
    let hovered = resp.hovered();
    let r = layout::CONTROL_RADIUS;
    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(r), color(p.surface_raised));
    let well = Rect::from_min_size(rect.min, vec2(width, well_h));
    painter.rect_filled(well, CornerRadius { nw: r, ne: r, sw: 0, se: 0 }, color(p.surface_sunken));
    let inner = well.shrink(14.0);
    match thumbnail(ui.ctx(), t, budget) {
        Some(tex) => {
            let size = tex.size_vec2();
            let fit = (inner.width() / size.x).min(inner.height() / size.y);
            let shown = Rect::from_center_size(inner.center(), size * fit);
            // A soft edge so a pale template does not dissolve into the well.
            painter.rect_stroke(shown.expand(0.5), 2.0, Stroke::new(1.0, color(p.border_subtle)), StrokeKind::Outside);
            painter.image(tex.id(), shown, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
        }
        None => {
            painter.rect_filled(Rect::from_center_size(inner.center(), inner.size() * 0.5), 6.0, color(p.control_field_bg));
        }
    }
    let edge = if hovered { Stroke::new(1.5, color(p.text_primary)) } else { Stroke::new(1.0, color(p.border_subtle)) };
    painter.rect_stroke(rect, CornerRadius::same(r), edge, StrokeKind::Inside);

    let x = rect.left() + 14.0;
    let room = rect.right() - x - 12.0;
    let ink = color(p.text_primary);
    let name = tl!(t.name);
    let name_galley = elided(ui, name, FontId::new(13.0, look.strong.clone()), ink, room);
    ui.painter().galley(pos2(x, well.bottom() + 10.0), name_galley, ink);
    let dim = color(p.text_secondary);
    let size_galley = elided(ui, &size_label(t), FontId::proportional(11.5), dim, room);
    ui.painter().galley(pos2(x, well.bottom() + 32.0), size_galley, dim);

    resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, name));
    resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked()
}

fn elided(ui: &Ui, text: &str, font: FontId, ink: Color32, width: f32) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_owned(), font, ink);
    job.wrap = egui::text::TextWrapping::truncate_at_width(width.max(1.0));
    ui.painter().layout_job(job)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grid_wraps_to_the_width_and_never_overflows_it() {
        for width in [120.0_f32, 188.0, 300.0, 420.0, 640.0, 900.0, 1180.0, 1700.0, 2600.0] {
            let (cols, card) = columns(width);
            assert!(cols >= 1);
            let used = card * cols as f32 + GAP * (cols as f32 - 1.0);
            assert!(used <= width.max(MIN_CARD.min(width)) + 0.5, "{cols} cards of {card} overflow {width} ({used})");
            assert!(card <= MAX_CARD);
        }
        assert_eq!(columns(188.0).0, 1);
        assert_eq!(columns(392.0).0, 2);
        assert!(columns(1180.0).0 >= 5);
    }

    #[test]
    fn sizes_read_in_inches_for_print_and_pixels_for_screens() {
        let find = |id: &str| template_cmds::find(id).unwrap();
        assert_eq!(size_label(find("coast-postcard")), "6 × 4 in");
        assert_eq!(size_label(find("business-card")), "3.5 × 2 in");
        assert_eq!(size_label(find("dusk-hills")), "2560 × 1440 px");
    }

    #[test]
    fn every_template_name_has_a_spanish_catalog_entry_that_matches_the_engine() {
        let es = crate::i18n::Lang::from_code("es").unwrap();
        for t in template_cmds::templates() {
            assert_eq!(crate::i18n::tr(es, t.name), t.name_es, "{} is not translated to Spanish", t.id);
        }
        for c in template_cmds::CATEGORIES {
            assert_ne!(crate::i18n::tr(es, c.label), c.label, "{} is not translated to Spanish", c.label);
        }
    }
}

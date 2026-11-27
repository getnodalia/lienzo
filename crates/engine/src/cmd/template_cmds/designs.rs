//! The twelve starter templates. Each function draws one composition, back to front, in design
//! pixels; the words come in English and Spanish through [`Canvas::t`].

use super::{Canvas, Paint, Template, Type};
use crate::Result;

/// Top to bottom: the first stop at the top edge.
const DOWN: f32 = -90.0;

pub(super) const TEMPLATES: &[Template] = &[
    Template {
        id: "quiet-quote",
        category: "social",
        name: "Quiet Quote",
        name_es: "Cita tranquila",
        summary: "A square post for one good sentence, on a warm wash with a low sun",
        summary_es: "Una publicación cuadrada para una buena frase, sobre un fondo cálido con un sol bajo",
        size: (1080, 1080),
        ppi: 72.0,
        base: "#F3E7DB",
        build: quiet_quote,
    },
    Template {
        id: "weekend-notes",
        category: "social",
        name: "Weekend Notes",
        name_es: "Notas del fin de semana",
        summary: "A tall story cover with a moon over layered hills",
        summary_es: "Una portada de historia vertical con una luna sobre colinas en capas",
        size: (1080, 1920),
        ppi: 72.0,
        base: "#1E3B45",
        build: weekend_notes,
    },
    Template {
        id: "little-things",
        category: "social",
        name: "Little Things",
        name_es: "Pequeñas cosas",
        summary: "Three photo tiles and a caption, ready for your own pictures",
        summary_es: "Tres recuadros de foto y un pie, listos para tus imágenes",
        size: (1080, 1350),
        ppi: 72.0,
        base: "#F6F2EC",
        build: little_things,
    },
    Template {
        id: "video-thumbnail",
        category: "social",
        name: "Video Thumbnail",
        name_es: "Miniatura de vídeo",
        summary: "A 16:9 cover with a big title, a tag and a warm sun",
        summary_es: "Una portada 16:9 con un título grande, una etiqueta y un sol cálido",
        size: (1280, 720),
        ppi: 72.0,
        base: "#1F2A44",
        build: video_thumbnail,
    },
    Template {
        id: "evening-poster",
        category: "print",
        name: "Evening Poster",
        name_es: "Cartel de noche",
        summary: "A 6 × 9 in poster with a full moon over deep indigo",
        summary_es: "Un cartel de 6 × 9 in con una luna llena sobre un índigo profundo",
        size: (1800, 2700),
        ppi: 300.0,
        base: "#1D2145",
        build: evening_poster,
    },
    Template {
        id: "coast-postcard",
        category: "print",
        name: "Coast Postcard",
        name_es: "Postal de costa",
        summary: "A 6 × 4 in postcard with a sunset over the sea and a greeting",
        summary_es: "Una postal de 6 × 4 in con un atardecer sobre el mar y un saludo",
        size: (1800, 1200),
        ppi: 300.0,
        base: "#F8E2C8",
        build: coast_postcard,
    },
    Template {
        id: "garden-invitation",
        category: "print",
        name: "Garden Invitation",
        name_es: "Invitación de jardín",
        summary: "A 5 × 7 in invitation with a fine double border",
        summary_es: "Una invitación de 5 × 7 in con un fino doble marco",
        size: (1500, 2100),
        ppi: 300.0,
        base: "#FBF6EC",
        build: garden_invitation,
    },
    Template {
        id: "business-card",
        category: "print",
        name: "Business Card",
        name_es: "Tarjeta de visita",
        summary: "A 3.5 × 2 in card in deep green with a small sun",
        summary_es: "Una tarjeta de 3,5 × 2 in en verde profundo con un pequeño sol",
        size: (1050, 600),
        ppi: 300.0,
        base: "#1F3B34",
        build: business_card,
    },
    Template {
        id: "dusk-hills",
        category: "art",
        name: "Dusk Hills",
        name_es: "Colinas al atardecer",
        summary: "A 2560 × 1440 wallpaper: a low sun and five quiet ridges",
        summary_es: "Un fondo de pantalla de 2560 × 1440: un sol bajo y cinco lomas serenas",
        size: (2560, 1440),
        ppi: 72.0,
        base: "#2C2F5B",
        build: dusk_hills,
    },
    Template {
        id: "web-banner",
        category: "art",
        name: "Web Banner",
        name_es: "Cabecera web",
        summary: "A 1500 × 500 header with a headline, a button and soft circles",
        summary_es: "Una cabecera de 1500 × 500 con un titular, un botón y círculos suaves",
        size: (1500, 500),
        ppi: 72.0,
        base: "#DDEEF0",
        build: web_banner,
    },
    Template {
        id: "album-cover",
        category: "art",
        name: "Album Cover",
        name_es: "Portada de disco",
        summary: "A square cover of warm concentric circles with a title",
        summary_es: "Una portada cuadrada de círculos concéntricos cálidos con un título",
        size: (1500, 1500),
        ppi: 72.0,
        base: "#2A1F2D",
        build: album_cover,
    },
    Template {
        id: "colour-palette",
        category: "art",
        name: "Colour Palette",
        name_es: "Paleta de color",
        summary: "Five calm swatches with their names and codes",
        summary_es: "Cinco muestras serenas con sus nombres y códigos",
        size: (1600, 1000),
        ppi: 72.0,
        base: "#F7F4EE",
        build: colour_palette,
    },
];

fn quiet_quote(c: &mut Canvas) -> Result<()> {
    c.rect("Wash", [0.0, 0.0, 1080.0, 1080.0], Paint::Linear(&[(0.0, "#F7ECDF"), (1.0, "#EBCFC1")], DOWN))?;
    c.circle("Sun", 800.0, 250.0, 150.0, Paint::Alpha("#E7AE97", 0.75))?;
    c.ring("Halo", [800.0, 250.0, 225.0], 2.0, "#D9A58F", 0.55)?;
    c.ring("Outer halo", [800.0, 250.0, 310.0], 2.0, "#D9A58F", 0.3)?;
    c.ellipse("Far hill", [-260.0, 900.0, 1100.0, 560.0], Paint::Alpha("#DDB09B", 0.75))?;
    c.ellipse("Near hill", [260.0, 960.0, 1200.0, 640.0], Paint::Alpha("#C98F7A", 0.7))?;
    c.rect("Rule", [120.0, 470.0, 72.0, 5.0], Paint::Solid("#B9826E"))?;
    let quote = c.t("Slow down. The good light lasts longer than you think.", "Ve despacio. La buena luz dura más de lo que crees.");
    c.text("Quote", Type::new(quote, [120.0, 520.0, 800.0, 280.0], 66.0, "#4A3328").leading(84.0))?;
    let by = c.t("Your name", "Tu nombre");
    c.text("Author", Type::new(by, [120.0, 830.0, 600.0, 40.0], 24.0, "#8C6655").weight(600).tracked(180.0).caps())?;
    Ok(())
}

fn weekend_notes(c: &mut Canvas) -> Result<()> {
    c.rect("Sky", [0.0, 0.0, 1080.0, 1920.0], Paint::Linear(&[(0.0, "#1B3640"), (0.55, "#3F6F73"), (1.0, "#E3CFAF")], DOWN))?;
    c.circle("Moon glow", 540.0, 520.0, 330.0, Paint::Alpha("#F4EBD8", 0.1))?;
    c.circle("Moon glow near", 540.0, 520.0, 230.0, Paint::Alpha("#F4EBD8", 0.16))?;
    c.circle("Moon", 540.0, 520.0, 150.0, Paint::Alpha("#F4EBD8", 0.95))?;
    c.ellipse("Far hill", [-420.0, 1420.0, 1500.0, 800.0], Paint::Alpha("#3B6468", 0.95))?;
    c.ellipse("Middle hill", [-80.0, 1530.0, 1500.0, 800.0], Paint::Solid("#2B5256"))?;
    c.ellipse("Near hill", [-520.0, 1650.0, 1560.0, 800.0], Paint::Solid("#1D3E44"))?;
    let kicker = c.t("A small journal", "Un pequeño diario");
    c.text("Kicker", Type::new(kicker, [90.0, 930.0, 900.0, 40.0], 26.0, "#CFE0DA").weight(600).tracked(300.0).caps())?;
    let title = c.t("Weekend\nnotes", "Notas del\nfin de semana");
    c.text("Title", Type::new(title, [90.0, 990.0, 900.0, 340.0], 118.0, "#F6EFE2").display().leading(128.0))?;
    let sub = c.t("Slow mornings, long walks, good light.", "Mañanas lentas, paseos largos, buena luz.");
    c.text("Subtitle", Type::new(sub, [90.0, 1290.0, 860.0, 120.0], 40.0, "#D9E6E0").leading(54.0))?;
    Ok(())
}

fn little_things(c: &mut Canvas) -> Result<()> {
    let hint = c.t("Your photo", "Tu foto");
    let one = c.rounded("Photo 1", [90.0, 90.0, 900.0, 640.0], 28.0, Paint::Linear(&[(0.0, "#D3DECB"), (1.0, "#9FB8A6")], 135.0))?;
    c.shadow(one, 10.0, 28.0, 14, "#3A4A3E")?;
    let two = c.rounded("Photo 2", [90.0, 770.0, 436.0, 300.0], 28.0, Paint::Linear(&[(0.0, "#F5D8C4"), (1.0, "#E0A98F")], 135.0))?;
    c.shadow(two, 10.0, 28.0, 14, "#5A3A2E")?;
    let three = c.rounded("Photo 3", [554.0, 770.0, 436.0, 300.0], 28.0, Paint::Linear(&[(0.0, "#D2E3EE"), (1.0, "#9DBBD2")], 135.0))?;
    c.shadow(three, 10.0, 28.0, 14, "#2E4458")?;
    c.text("Photo 1 hint", Type::new(hint, [90.0, 392.0, 900.0, 40.0], 30.0, "#5F7867").center().weight(600))?;
    c.text("Photo 2 hint", Type::new(hint, [90.0, 904.0, 436.0, 40.0], 26.0, "#8E5E49").center().weight(600))?;
    c.text("Photo 3 hint", Type::new(hint, [554.0, 904.0, 436.0, 40.0], 26.0, "#476580").center().weight(600))?;
    let title = c.t("Little things", "Pequeñas cosas");
    c.text("Title", Type::new(title, [90.0, 1118.0, 900.0, 100.0], 78.0, "#3A3A36").display())?;
    let sub = c.t("Small moments worth keeping", "Pequeños momentos que merece la pena guardar");
    c.text("Caption", Type::new(sub, [90.0, 1218.0, 900.0, 50.0], 32.0, "#7B766D"))?;
    Ok(())
}

fn video_thumbnail(c: &mut Canvas) -> Result<()> {
    c.rect("Backdrop", [0.0, 0.0, 1280.0, 720.0], Paint::Linear(&[(0.0, "#1F2A44"), (1.0, "#46688A")], -35.0))?;
    c.circle("Sun glow", 1090.0, 440.0, 330.0, Paint::Alpha("#F2C879", 0.14))?;
    c.circle("Sun", 1090.0, 440.0, 200.0, Paint::Solid("#F2C879"))?;
    c.circle("Small circle", 1180.0, 110.0, 48.0, Paint::Alpha("#F6EFE2", 0.85))?;
    c.circle("Dot", 800.0, 600.0, 22.0, Paint::Alpha("#F6EFE2", 0.5))?;
    c.rounded("Tag", [70.0, 96.0, 250.0, 56.0], 28.0, Paint::Solid("#F2C879"))?;
    let tag = c.t("Tutorial", "Tutorial");
    c.text("Tag text", Type::new(tag, [70.0, 108.0, 250.0, 40.0], 24.0, "#1F2A44").weight(700).center().tracked(220.0).caps())?;
    let title = c.t("How I edit\ncalm photos", "Cómo edito\nfotos serenas");
    c.text("Title", Type::new(title, [70.0, 190.0, 800.0, 330.0], 112.0, "#FFFFFF").display().leading(122.0))?;
    c.rect("Rule", [70.0, 572.0, 72.0, 5.0], Paint::Solid("#F2C879"))?;
    let sub = c.t("A five minute workflow", "Un flujo de cinco minutos");
    c.text("Subtitle", Type::new(sub, [70.0, 596.0, 700.0, 50.0], 36.0, "#DCE6EE"))?;
    Ok(())
}

fn evening_poster(c: &mut Canvas) -> Result<()> {
    c.rect("Night", [0.0, 0.0, 1800.0, 2700.0], Paint::Linear(&[(0.0, "#1A1D3F"), (1.0, "#3E4380")], DOWN))?;
    c.ring("Ring wide", [900.0, 1010.0, 790.0], 4.0, "#F3E9D2", 0.12)?;
    c.ring("Ring", [900.0, 1010.0, 650.0], 4.0, "#F3E9D2", 0.22)?;
    c.circle("Moon glow", 900.0, 1010.0, 560.0, Paint::Alpha("#F3E9D2", 0.1))?;
    c.circle("Moon", 900.0, 1010.0, 470.0, Paint::Solid("#F3E9D2"))?;
    c.circle("Crater", 760.0, 880.0, 90.0, Paint::Alpha("#D9CDB1", 0.55))?;
    c.circle("Crater small", 1010.0, 1130.0, 56.0, Paint::Alpha("#D9CDB1", 0.5))?;
    c.circle("Crater tiny", 1090.0, 860.0, 34.0, Paint::Alpha("#D9CDB1", 0.45))?;
    for (i, (x, y, r)) in [(280.0, 380.0, 9.0), (1480.0, 300.0, 12.0), (1560.0, 760.0, 8.0), (230.0, 1180.0, 10.0), (1420.0, 1560.0, 9.0), (360.0, 1580.0, 7.0)]
        .into_iter()
        .enumerate()
    {
        c.circle(&format!("Star {}", i + 1), x, y, r, Paint::Alpha("#F3E9D2", 0.8))?;
    }
    let kicker = c.t("Open air concert", "Concierto al aire libre");
    c.text("Kicker", Type::new(kicker, [150.0, 1860.0, 1500.0, 70.0], 46.0, "#BFC4EC").weight(600).center().tracked(380.0).caps())?;
    let title = c.t("Under the\nmoon", "Bajo la\nluna");
    c.text("Title", Type::new(title, [150.0, 1950.0, 1500.0, 440.0], 210.0, "#F6EFE2").display().center().leading(214.0))?;
    let when = c.t("Saturday · 8 pm · Riverside Park", "Sábado · 20:00 · Parque del Río");
    c.text("Details", Type::new(when, [150.0, 2470.0, 1500.0, 80.0], 56.0, "#D6DAF5").center())?;
    Ok(())
}

fn coast_postcard(c: &mut Canvas) -> Result<()> {
    c.rect("Sky", [0.0, 0.0, 1800.0, 1200.0], Paint::Linear(&[(0.0, "#F9E5C9"), (0.6, "#F4BFA3"), (1.0, "#E9A28F")], DOWN))?;
    c.circle("Sun glow", 1230.0, 640.0, 330.0, Paint::Alpha("#FFF1D6", 0.3))?;
    c.circle("Sun", 1230.0, 640.0, 190.0, Paint::Solid("#FFF1D6"))?;
    c.rect("Sea", [0.0, 700.0, 1800.0, 500.0], Paint::Linear(&[(0.0, "#E58E86"), (1.0, "#9B5E77")], DOWN))?;
    c.rect("Sun path", [1090.0, 700.0, 280.0, 14.0], Paint::Alpha("#FFF1D6", 0.8))?;
    c.rect("Sun path 2", [1130.0, 760.0, 200.0, 12.0], Paint::Alpha("#FFF1D6", 0.6))?;
    c.rect("Sun path 3", [1170.0, 820.0, 120.0, 10.0], Paint::Alpha("#FFF1D6", 0.45))?;
    c.ellipse("Headland", [1100.0, 760.0, 1100.0, 640.0], Paint::Solid("#7B4D63"))?;
    c.ellipse("Cliff", [-300.0, 880.0, 1100.0, 600.0], Paint::Solid("#5C3A55"))?;
    let card = c.rounded("Label", [110.0, 790.0, 780.0, 290.0], 26.0, Paint::Alpha("#FFF8EE", 0.94))?;
    c.shadow(card, 8.0, 24.0, 18, "#4A2A3A")?;
    let hello = c.t("Hello from\nthe coast", "Saludos desde\nla costa");
    c.text("Greeting", Type::new(hello, [160.0, 830.0, 700.0, 170.0], 72.0, "#5A3A3F").display().leading(80.0))?;
    let wish = c.t("Wish you were here", "Ojalá estuvieras aquí");
    c.text("Line", Type::new(wish, [160.0, 1000.0, 700.0, 50.0], 38.0, "#8A6A64"))?;
    Ok(())
}

fn garden_invitation(c: &mut Canvas) -> Result<()> {
    c.circle("Leaf cluster top left", 150.0, 150.0, 240.0, Paint::Alpha("#C9D7C0", 0.45))?;
    c.circle("Leaf cluster top left small", 330.0, 100.0, 120.0, Paint::Alpha("#B9CCB0", 0.4))?;
    c.circle("Leaf cluster bottom right", 1350.0, 1950.0, 260.0, Paint::Alpha("#C9D7C0", 0.45))?;
    c.circle("Leaf cluster bottom right small", 1170.0, 2040.0, 130.0, Paint::Alpha("#B9CCB0", 0.4))?;
    c.frame("Border", [90.0, 90.0, 1320.0, 1920.0], 12.0, 4.0, "#C3A58A", 1.0)?;
    c.frame("Inner border", [126.0, 126.0, 1248.0, 1848.0], 6.0, 1.6, "#C3A58A", 0.7)?;
    c.ring("Monogram ring", [750.0, 420.0, 90.0], 3.0, "#B58E72", 1.0)?;
    c.circle("Monogram dot", 750.0, 420.0, 18.0, Paint::Solid("#B58E72"))?;
    let kicker = c.t("You are invited", "Estás invitado");
    c.text("Kicker", Type::new(kicker, [150.0, 590.0, 1200.0, 60.0], 38.0, "#A07F66").weight(600).center().tracked(420.0).caps())?;
    let names = c.t("Ana\n& Pablo", "Ana\ny Pablo");
    c.text("Names", Type::new(names, [150.0, 720.0, 1200.0, 470.0], 160.0, "#4B3B34").display().center().leading(176.0))?;
    c.rect("Rule", [690.0, 1260.0, 120.0, 4.0], Paint::Solid("#B58E72"))?;
    let date = c.t("Saturday, 14 June\nat five in the afternoon", "Sábado, 14 de junio\na las cinco de la tarde");
    c.text("Date", Type::new(date, [150.0, 1330.0, 1200.0, 200.0], 56.0, "#5E4A40").center().leading(76.0))?;
    let place = c.t("The Garden House", "La Casa del Jardín");
    c.text("Place", Type::new(place, [150.0, 1560.0, 1200.0, 80.0], 56.0, "#4B3B34").display().center())?;
    let rsvp = c.t("Please reply by 1 June", "Confirma antes del 1 de junio");
    c.text("Reply", Type::new(rsvp, [150.0, 1730.0, 1200.0, 60.0], 38.0, "#8C7364").center())?;
    Ok(())
}

fn business_card(c: &mut Canvas) -> Result<()> {
    c.rect("Card", [0.0, 0.0, 1050.0, 600.0], Paint::Linear(&[(0.0, "#1F3B34"), (1.0, "#27483F")], -20.0))?;
    c.circle("Mark circle", 880.0, 290.0, 340.0, Paint::Alpha("#35605A", 0.55))?;
    c.circle("Mark circle inner", 880.0, 290.0, 220.0, Paint::Alpha("#3F7068", 0.55))?;
    c.circle("Sun", 880.0, 290.0, 70.0, Paint::Solid("#E6B96F"))?;
    let name = c.t("Your Name", "Tu Nombre");
    c.text("Name", Type::new(name, [70.0, 118.0, 680.0, 110.0], 74.0, "#F4EBDD").display())?;
    let role = c.t("Photographer", "Fotografía");
    c.text("Role", Type::new(role, [70.0, 232.0, 680.0, 50.0], 30.0, "#A9CBB9").weight(600).tracked(200.0).caps())?;
    c.rect("Rule", [70.0, 318.0, 64.0, 4.0], Paint::Solid("#E6B96F"))?;
    c.text("Email", Type::new("hello@yourname.com", [70.0, 400.0, 600.0, 44.0], 30.0, "#DCE8E0"))?;
    c.text("Phone", Type::new("+00 000 000 000", [70.0, 446.0, 600.0, 44.0], 30.0, "#DCE8E0"))?;
    c.text("Web", Type::new("yourname.com", [70.0, 492.0, 600.0, 44.0], 30.0, "#DCE8E0"))?;
    Ok(())
}

fn dusk_hills(c: &mut Canvas) -> Result<()> {
    c.rect("Sky", [0.0, 0.0, 2560.0, 1440.0], Paint::Linear(&[(0.0, "#2A2D58"), (0.5, "#B4688B"), (1.0, "#F6BC93")], DOWN))?;
    c.circle("Sun glow wide", 1280.0, 800.0, 640.0, Paint::Alpha("#FFD9B0", 0.08))?;
    c.circle("Sun glow", 1280.0, 800.0, 440.0, Paint::Alpha("#FFD9B0", 0.14))?;
    c.circle("Sun", 1280.0, 800.0, 210.0, Paint::Solid("#FFE5BF"))?;
    c.ellipse("Ridge 1", [-500.0, 880.0, 2300.0, 900.0], Paint::Alpha("#7C5A86", 0.9))?;
    c.ellipse("Ridge 2", [900.0, 940.0, 2400.0, 900.0], Paint::Alpha("#634C7E", 0.95))?;
    c.ellipse("Ridge 3", [-700.0, 1040.0, 2500.0, 900.0], Paint::Solid("#4A3E70"))?;
    c.ellipse("Ridge 4", [700.0, 1130.0, 2500.0, 900.0], Paint::Solid("#35305F"))?;
    c.ellipse("Ridge 5", [-400.0, 1240.0, 2300.0, 900.0], Paint::Solid("#211E48"))?;
    Ok(())
}

fn web_banner(c: &mut Canvas) -> Result<()> {
    c.rect("Backdrop", [0.0, 0.0, 1500.0, 500.0], Paint::Linear(&[(0.0, "#E6F3F1"), (1.0, "#BDDFE0")], 0.0))?;
    c.circle("Large circle", 1190.0, 260.0, 230.0, Paint::Alpha("#9CCBCF", 0.65))?;
    c.circle("Medium circle", 1010.0, 430.0, 110.0, Paint::Alpha("#7DB4BA", 0.6))?;
    c.circle("Sun", 1330.0, 120.0, 82.0, Paint::Solid("#F4D9A8"))?;
    c.ring("Ring", [1190.0, 260.0, 300.0], 2.0, "#7DB4BA", 0.5)?;
    let title = c.t("Your headline\ngoes right here", "Tu titular\nva justo aquí");
    c.text("Headline", Type::new(title, [90.0, 92.0, 760.0, 200.0], 66.0, "#1F4B52").display().leading(76.0))?;
    let sub = c.t("One calm sentence about what you do.", "Una frase serena sobre lo que haces.");
    c.text("Subline", Type::new(sub, [90.0, 290.0, 760.0, 50.0], 30.0, "#3C6B72"))?;
    c.rounded("Button", [90.0, 372.0, 240.0, 66.0], 33.0, Paint::Solid("#1F4B52"))?;
    let go = c.t("Learn more", "Saber más");
    c.text("Button label", Type::new(go, [90.0, 388.0, 240.0, 40.0], 26.0, "#FFFFFF").weight(600).center())?;
    Ok(())
}

fn album_cover(c: &mut Canvas) -> Result<()> {
    c.rect("Backdrop", [0.0, 0.0, 1500.0, 1500.0], Paint::Linear(&[(0.0, "#2F2233"), (1.0, "#231A28")], DOWN))?;
    c.circle("Disc 1", 750.0, 640.0, 640.0, Paint::Solid("#3C2A40"))?;
    c.circle("Disc 2", 750.0, 640.0, 520.0, Paint::Solid("#6E3A50"))?;
    c.circle("Disc 3", 750.0, 640.0, 400.0, Paint::Solid("#B65A4E"))?;
    c.circle("Disc 4", 750.0, 640.0, 280.0, Paint::Solid("#E89A5C"))?;
    c.circle("Disc 5", 750.0, 640.0, 150.0, Paint::Solid("#FBDFA6"))?;
    c.circle("Hole", 750.0, 640.0, 26.0, Paint::Solid("#2F2233"))?;
    let title = c.t("Golden hour", "Hora dorada");
    c.text("Title", Type::new(title, [110.0, 1250.0, 1280.0, 120.0], 100.0, "#F8E6C4").display())?;
    let by = c.t("Your name", "Tu nombre");
    c.text("Artist", Type::new(by, [110.0, 1372.0, 1280.0, 50.0], 32.0, "#C9A88F").weight(600).tracked(260.0).caps())?;
    Ok(())
}

fn colour_palette(c: &mut Canvas) -> Result<()> {
    let title = c.t("Morning palette", "Paleta de mañana");
    c.text("Title", Type::new(title, [120.0, 96.0, 1300.0, 100.0], 76.0, "#34302B").display())?;
    let sub = c.t("Five calm colours for your next project", "Cinco colores serenos para tu próximo proyecto");
    c.text("Subtitle", Type::new(sub, [120.0, 200.0, 1300.0, 50.0], 34.0, "#7C766C"))?;
    let swatches = [
        (c.t("Sand", "Arena"), "#E9D5C3", "#E9D5C3"),
        (c.t("Linen", "Lino"), "#CDB99C", "#CDB99C"),
        (c.t("Sage", "Salvia"), "#9DB4A0", "#9DB4A0"),
        (c.t("Lagoon", "Laguna"), "#6E8F8B", "#6E8F8B"),
        (c.t("Pine", "Pino"), "#3D5A5E", "#3D5A5E"),
    ];
    for (i, (name, hex, _)) in swatches.into_iter().enumerate() {
        let x = 120.0 + i as f32 * 276.0;
        let tile = c.rounded(&format!("Swatch {}", i + 1), [x, 300.0, 256.0, 420.0], 24.0, Paint::Solid(hex))?;
        c.shadow(tile, 8.0, 24.0, 12, "#3A3A36")?;
        c.text(&format!("Name {}", i + 1), Type::new(name, [x, 756.0, 256.0, 44.0], 32.0, "#34302B").weight(600))?;
        c.text(&format!("Code {}", i + 1), Type::new(hex, [x, 800.0, 256.0, 40.0], 26.0, "#7C766C"))?;
    }
    Ok(())
}

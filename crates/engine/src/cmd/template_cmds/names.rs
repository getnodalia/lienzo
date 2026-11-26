//! The layer names of the starter templates in Spanish.
//!
//! A template is drawn with English layer names; opened from a Spanish interface, its layers
//! (Sun, Near hill, Quote…) arrive named in Spanish too, so nothing in the Layers panel stays
//! untranslated. [`spanish`] looks one name up; a name that ends in a number (`Ridge 3`) is
//! translated without it and the number put back, so the table lists `Ridge` once.

/// English layer name → Spanish, sorted by the English name.
const NAMES_ES: &[(&str, &str)] = &[
    ("Artist", "Artista"),
    ("Author", "Autor"),
    ("Backdrop", "Telón de fondo"),
    ("Border", "Borde"),
    ("Button", "Botón"),
    ("Button label", "Texto del botón"),
    ("Caption", "Pie de foto"),
    ("Card", "Tarjeta"),
    ("Cliff", "Acantilado"),
    ("Code", "Código"),
    ("Crater", "Cráter"),
    ("Crater small", "Cráter pequeño"),
    ("Crater tiny", "Cráter diminuto"),
    ("Date", "Fecha"),
    ("Details", "Detalles"),
    ("Disc", "Disco"),
    ("Dot", "Punto"),
    ("Email", "Correo electrónico"),
    ("Far hill", "Colina lejana"),
    ("Greeting", "Saludo"),
    ("Halo", "Halo"),
    ("Headland", "Cabo"),
    ("Headline", "Titular"),
    ("Hole", "Agujero"),
    ("Inner border", "Borde interior"),
    ("Kicker", "Antetítulo"),
    ("Label", "Rótulo"),
    ("Large circle", "Círculo grande"),
    ("Leaf cluster bottom right", "Hojas abajo a la derecha"),
    ("Leaf cluster bottom right small", "Hojas pequeñas abajo a la derecha"),
    ("Leaf cluster top left", "Hojas arriba a la izquierda"),
    ("Leaf cluster top left small", "Hojas pequeñas arriba a la izquierda"),
    ("Line", "Línea"),
    ("Mark circle", "Círculo de la marca"),
    ("Mark circle inner", "Círculo interior de la marca"),
    ("Medium circle", "Círculo mediano"),
    ("Middle hill", "Colina intermedia"),
    ("Monogram dot", "Punto del monograma"),
    ("Monogram ring", "Anillo del monograma"),
    ("Moon", "Luna"),
    ("Moon glow", "Resplandor de la luna"),
    ("Moon glow near", "Resplandor cercano de la luna"),
    ("Name", "Nombre"),
    ("Names", "Nombres"),
    ("Near hill", "Colina cercana"),
    ("Night", "Noche"),
    ("Outer halo", "Halo exterior"),
    ("Phone", "Teléfono"),
    ("Photo", "Foto"),
    ("Photo 1 hint", "Indicación de la foto 1"),
    ("Photo 2 hint", "Indicación de la foto 2"),
    ("Photo 3 hint", "Indicación de la foto 3"),
    ("Place", "Lugar"),
    ("Quote", "Cita"),
    ("Reply", "Respuesta"),
    ("Ridge", "Cresta"),
    ("Ring", "Anillo"),
    ("Ring wide", "Anillo amplio"),
    ("Role", "Cargo"),
    ("Rule", "Filete"),
    ("Sea", "Mar"),
    ("Sky", "Cielo"),
    ("Small circle", "Círculo pequeño"),
    ("Star", "Estrella"),
    ("Subline", "Línea secundaria"),
    ("Subtitle", "Subtítulo"),
    ("Sun", "Sol"),
    ("Sun glow", "Resplandor del sol"),
    ("Sun glow wide", "Resplandor amplio del sol"),
    ("Sun path", "Trayectoria del sol"),
    ("Swatch", "Muestra"),
    ("Tag", "Etiqueta"),
    ("Tag text", "Texto de la etiqueta"),
    ("Title", "Título"),
    ("Wash", "Fondo suave"),
    ("Web", "Web"),
];

/// The Spanish for the template layer name `english`, or `None` when the table has no entry.
pub(super) fn spanish(english: &str) -> Option<String> {
    let exact = |name: &str| NAMES_ES.binary_search_by_key(&name, |(en, _)| en).ok().and_then(|i| NAMES_ES.get(i)).map(|(_, es)| *es);
    if let Some(es) = exact(english) {
        return Some(es.to_string());
    }
    let (stem, number) = english.rsplit_once(' ')?;
    if number.is_empty() || !number.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    exact(stem).map(|es| format!("{es} {number}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_is_sorted_for_the_lookup() {
        assert!(NAMES_ES.windows(2).all(|w| w[0].0 < w[1].0), "NAMES_ES must stay sorted by the English name");
    }

    #[test]
    fn numbered_names_keep_their_number() {
        assert_eq!(spanish("Ridge 3").as_deref(), Some("Cresta 3"));
        assert_eq!(spanish("Sun path 2").as_deref(), Some("Trayectoria del sol 2"));
        assert_eq!(spanish("Photo 1 hint").as_deref(), Some("Indicación de la foto 1"));
        assert_eq!(spanish("Near hill").as_deref(), Some("Colina cercana"));
    }

    #[test]
    fn unknown_names_are_not_invented() {
        assert_eq!(spanish("Mystery"), None);
        assert_eq!(spanish("Ridge x"), None);
        assert_eq!(spanish(""), None);
    }
}

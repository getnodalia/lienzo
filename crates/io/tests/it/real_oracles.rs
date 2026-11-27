//! Real Photoshop- and GIMP-authored files are optional oracles: they are not pinned or fetched,
//! and a test that reads them skips with a message when `corpus/photoshop` or `corpus/gimp` is
//! absent. Set `LIENZO_ORACLES_REQUIRED=1` to make a missing one fail instead (for a run that
//! must not pass by skipping).

/// True when `corpus/<rel>` is missing, after printing why the test is skipped.
#[allow(dead_code)]
pub fn missing(rel: &str) -> bool {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join("corpus").join(rel);
    if dir.is_dir() {
        return false;
    }
    assert!(
        std::env::var_os("LIENZO_ORACLES_REQUIRED").is_none_or(|v| v.is_empty()),
        "corpus/{rel} is missing and LIENZO_ORACLES_REQUIRED is set ({})",
        dir.display()
    );
    eprintln!("SKIPPED: corpus/{rel} is not present (real-file oracles are optional; see docs/development.md > Test corpora)");
    true
}

#[allow(unused_macros)]
/// Returns from the test when `corpus/<rel>` is missing.
macro_rules! skip_without_oracles {
    ($rel:expr) => {
        if crate::real_oracles::missing($rel) {
            return;
        }
    };
}

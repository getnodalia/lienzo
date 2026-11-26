# nodalia-ui (vendored)

Copy of `crates/ui` from the nodalia-libs generated tree at `6db9b8e114917c0f9c046e431efa0f9f7eb9beb1` (getnodalia/foundation, branch nodalia/libs)
(MIT OR Apache-2.0), used through `[patch]` in the workspace `Cargo.toml`.

Changes: `DockStyle::floating_radius` (and `ChromeStyle::radius`), the corner radius of a
floating window's fill and outline. It is 0 by default, so nothing changes unless an app sets it;
Lienzo sets it for the Nodalia themes so the floating Properties card is round like every other
island. Files touched: `src/docking/chrome.rs`, `src/docking/surface.rs`.

`DockStyle::dock_radius` does the same for the docked groups (cards in a column), and a tab that
sits on a corner of a rounded card (a dock group or a floating window) is filled with that
corner's radius, so the active tab no longer squares the card's top corner
(`src/docking/render/group.rs`, `tab`). A group of the split tree (a panel docked above, below
or beside the canvas) is a card too, except a lone protected panel such as the canvas
(`group`, `GroupKind::Frame`).

To update: copy the new `crates/ui/src` over this one, re-apply the change above, and move the
revision in the workspace `Cargo.toml`, in this file and in the header of `Cargo.toml`.

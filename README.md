<div align="center">
  <a href="https://getnodalia.com">
    <img src="https://raw.githubusercontent.com/danielmigueltejedor/nodalia-web/main/assets/brand/nodalia-wordmark-badge.svg" width="150" alt="Nodalia">
  </a>
  <br><br>
  <img src="https://raw.githubusercontent.com/danielmigueltejedor/lienzo/nodalia/assets/app-icon/hicolor/512x512/apps/dev.danielmiguelt.Lienzo.png" width="192" alt="Lienzo">
  <h1>Lienzo</h1>
  <p><strong>The image editor of the Nodalia suite.</strong></p>

  <p>
    <img src="https://img.shields.io/badge/status-early%20stage-orange" alt="Early stage">
    <img src="https://img.shields.io/badge/Rust-native-dea584?logo=rust&logoColor=black" alt="Native Rust">
    <img src="https://img.shields.io/badge/design-Nodalia-EE3B6B" alt="Nodalia Design">
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2ea44f" alt="MIT OR Apache-2.0">
  </p>

  <p>
    <a href="#overview">Overview</a> ·
    <a href="#what-it-does">What it does</a> ·
    <a href="#build-and-run">Build</a> ·
    <a href="#project-status">Status</a> ·
    <a href="#the-nodalia-suite">Suite</a> ·
    <a href="#license">License</a>
  </p>
</div>

---

## Overview

**Lienzo** is the image editor of [Nodalia](https://getnodalia.com): a native app written in Rust
for working in layers, with no account and no cloud. It runs on Linux, macOS and Windows.

<p align="center">
  <img src="https://raw.githubusercontent.com/getnodalia/nodalia-web/main/assets/screens/lienzo/lienzo-editor-dark.jpg" width="900" alt="Lienzo editing The Great Wave off Kanagawa in the dark theme, with adjustment layers in the Layers panel">
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/getnodalia/nodalia-web/main/assets/screens/lienzo/lienzo-editor-light.jpg" width="440" alt="The same image in the light theme, with the Curves properties">
  <img src="https://raw.githubusercontent.com/getnodalia/nodalia-web/main/assets/screens/lienzo/lienzo-home-dark.jpg" width="440" alt="Lienzo's start screen in the dark theme">
</p>

> [!IMPORTANT]
> **Lienzo is at an early stage.** Version 0.4.2 is out for Linux, macOS and Windows:
> [download it from the releases page](https://github.com/getnodalia/lienzo/releases/latest)
> (AppImage, .deb, .rpm and .tar.gz; a DMG; a Windows installer and portable zip).

## What it does

- Layers, masks, adjustment layers, smart objects, selections, paths, type and brushes.
- A template gallery on Home with twelve calm starting points (social posts, a poster, a postcard,
  a business card, a web banner and more), each with named layers and live text.
- Opens and saves Photoshop documents (PSD and PSB), plus its own `.pcraft` format and the usual
  image formats.
- The Nodalia interface: rounded islands, Lienzo's accent, Sora and Inter, round window controls
  (or your desktop's own, on Linux), light and dark following the system.
- Every action is a command, so it can be scripted and driven from the command line.

<details>
<summary>Every tool in the toolbox</summary>

<h4>🧰 69 tools</h4>
Move · Artboard · Rectangular and Elliptical Marquee · Single Row Marquee · Single Column Marquee · Lasso · Polygonal Lasso · Magnetic Lasso · Object Selection · Quick Selection · Magic Wand · Crop · Perspective Crop · Slice · Slice Select · Frame · Eyedropper · Color Sampler · Ruler · Note · Count · Remove · Spot Healing Brush · Healing Brush · Patch · Content-Aware Move · Red Eye · Brush · Pencil · Mixer Brush · Color Replacement · Clone Stamp · Pattern Stamp · History Brush · Art History Brush · Eraser · Background Eraser · Magic Eraser · Gradient · Paint Bucket · Blur · Sharpen · Smudge · Dodge · Burn · Sponge · Pen · Freeform Pen · Curvature Pen · Add Anchor Point · Delete Anchor Point · Convert Point · Horizontal and Vertical Type · Horizontal and Vertical Type Mask · Path Selection · Direct Selection · Rectangle · Ellipse · Triangle · Polygon · Star · Line · Custom Shape · Hand · Rotate View · Zoom
</details>

## Build and run

You need a recent stable Rust toolchain.

```bash
git clone https://github.com/danielmigueltejedor/lienzo.git
cd lienzo
cargo run --release -p lienzo
```


## Project status

| Area | State |
|---|---|
| Editor | Working |
| App icon | Done: portrait of Diego Velázquez |
| Nodalia interface | Done: theme, islands, window controls, main menu |
| Global menu in the top bar (D-Bus) | Planned |
| Package and binary names | Pending |
| Installable packages | None published |

## App icon

Nodalia app icons are black-and-white portraits of historical figures over the app colour.
Lienzo's is Diego Velázquez, the painter. Provenance and licence are in [`assets/app-icon/`](assets/app-icon/).

## The Nodalia suite

Lienzo is one of the applications of [Nodalia](https://getnodalia.com), an independent software
project developed in public by [Daniel Miguel Tejedor](https://danielmigueltejedor.com).

## License

`MIT OR Apache-2.0`. See [`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE) and
[`NOTICE`](NOTICE).

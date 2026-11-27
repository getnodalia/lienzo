---
title: Install Lienzo
description: Download Lienzo from its official GitHub releases and choose an installer for your operating system.
order: 10
verified: 0.8.2
---

Download Lienzo from the [official releases page](https://github.com/getnodalia/lienzo/releases/latest). Choose an asset listed for your operating system and architecture. In the file names below, `<version>` stands for the release version. Available formats can vary by release.

## Windows

Lienzo needs Windows 10 or later. On Windows 7 and 8 the installer stops with "Lienzo needs Windows 10 or later."

1. Open the [latest release](https://github.com/getnodalia/lienzo/releases/latest).
2. Download `lienzo-<version>-windows-x64.msi`. It fits most PCs (64-bit Intel and AMD).
3. Run the file and follow the installer.

To run without installing, download `lienzo-<version>-windows-x64-portable.zip` instead and unzip it.

| Your PC                            | Installer                                | Portable                                          |
|------------------------------------|------------------------------------------|---------------------------------------------------|
| 64-bit Intel or AMD                | `lienzo-<version>-windows-x64.msi`   | `lienzo-<version>-windows-x64-portable.zip`   |
| Arm (Snapdragon and other Arm PCs) | `lienzo-<version>-windows-arm64.msi` | `lienzo-<version>-windows-arm64-portable.zip` |
| 32-bit                             | `lienzo-<version>-windows-x86.msi`   | `lienzo-<version>-windows-x86-portable.zip`   |

Check the release notes for signing and installation details.

## macOS

1. Download `lienzo-<version>-macos-universal.dmg`. It works on Apple silicon and Intel Macs,.
2. Open it and drag Lienzo to Applications.

## Linux

Pick one file from the release. Use `x86_64` for Intel and AMD, or `aarch64` for Arm.

| Format              | File                                         | Notes                                                                              |
|---------------------|----------------------------------------------|------------------------------------------------------------------------------------|
| AppImage            | `lienzo-<version>-linux-x86_64.AppImage` | Runs anywhere and needs no install.                                                |
| Debian and Ubuntu   | `lienzo-<version>-linux-x86_64.deb`      |                                                                                    |
| Fedora and openSUSE | `lienzo-<version>-linux-x86_64.rpm`      |                                                                                    |
| Tarball             | `lienzo-<version>-linux-x86_64.tar.gz`   | Unpack it anywhere.                                                                |

The first time you run the AppImage, it adds its icon and menu entry under `~/.local/share`. Set `LIENZO_NO_DESKTOP_INTEGRATION=1` to skip that. For RISC-V, take the `riscv64` file: `lienzo-<version>-linux-riscv64.tar.gz`, `.deb` or `.rpm`. The RISC-V tarball needs glibc 2.39 or newer.

On a Wayland session, files dropped on the Lienzo window do not open yet. Use **File › Open…** instead, or copy the image in your file manager and paste it with Ctrl+V. To drag and drop, start Lienzo under XWayland with `WAYLAND_DISPLAY= lienzo`. Lienzo shows the right command for your install in a notice until you dismiss it.

## FreeBSD and the web

FreeBSD 14 (x86_64) has a tarball, `lienzo-<version>-freebsd-x86_64.tar.gz`, laid out like `/usr/local`. The web version comes as `lienzo-web-<version>.zip`, a static site you host on any web server. It runs in a modern browser.

## Check your download

`SHA256SUMS.txt` on the release lists a checksum for every file.

## Next step

Open an image: [Open and save files](../guides/open-and-save-files.md).

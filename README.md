# HDL Register Wizard

This is both a webapp and a desktop application that can generate VHDL code and documentation to create hardware registers accessible on a memory mapped bus. It can load and save files in the Model Description Format developped by Bitvis for its (now defunct) [Register Wizard](https://bitvis.no/dev-tools/register-wizard/). Files saved by this webapp should be usable by Bitvis' tool.

## Trial

The webapp compiled from the most current sources can be accessed [here](https://daixiwen.github.io/hdl-register-wizard/).

## Project Status

The project is under development and is not currently usable. The aim for the first release is to be able to load and save MDF files, as the [Bitvis Register Wizard](https://bitvis.no/dev-tools/register-wizard/) didn't have a GUI. It can also generate documentation, but no code yet.

## Project License

The project uses an MIT license.

## Dependencies

The application requires Webview2 on Windows and WebKitGTK on Linux. Webview2 should be installed on an up to date Windows 10/11, and most Linux distributions install WebKitGTK.

## Installation

### Windows

Two msi installers are available. The one with the `-user` suffix will install the application for the local user only, and doesn't need administrative rights. The other one will install it for all users.

The installer will automatically detect if Webview2 is not installed and will download it automatically.

### Linux

Currently for Linux the application needs to be built from source

## Libraries

This application is written in Rust and built upon several components, and among them:
- GUI engine: [Dioxus](https://dioxuslabs.com/), which is itself build over [Wry](https://github.com/tauri-apps/wry)
- Template engine: [Tera](https://keats.github.io/tera/)
- GUI CSS framework: [Bulma](https://bulma.io/)
- Symbols: [Fontawesome](https://fontawesome.com/)

## Building

See the [building](BUILDING.md) page for instructions

# License

```
Copyright © 2020-2024 Sylvain Tertois
This work is free. You can redistribute it and/or modify it under the
terms of the Do What The Fuck You Want To Public License, Version 2,
as published by Sam Hocevar. See the COPYING file for more details.
```

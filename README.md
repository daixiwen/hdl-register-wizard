# HDL Register Wizard

This is both a webapp and a desktop application that can generate VHDL code and documentation to create hardware registers accessible on a memory mapped bus. It can load and save files in the Model Description Format developped by Bitvis for its (now defunct) [Register Wizard](https://bitvis.no/dev-tools/register-wizard/). Files saved by this webapp should be usable by Bitvis' tool.

## Trial

The webapp compiled from the most current sources can be accessed [here](https://daixiwen.github.io/hdl-register-wizard/).

## Project Status

The project is under development and is not currently usable. The aim for the first release is to be able to load and save MDF files, as the [Bitvis Register Wizard](https://bitvis.no/dev-tools/register-wizard/) didn't have a GUI. It can also generate documentation, but no code yet.

## Project License

The project uses an MIT license.

## Dependencies

### Windows

The application requires Webview2, that should be installed on an up to date Windows 10/11.

### Linux

The application requires WebKitGTK and the xdo library. Most Linux distributions install WebKitGTK by default, and the xdo library is usually installed with a package called xdo-tool.

## Installation

### Windows

Two msi installers are available. The one with the `-user` suffix will install the application for the local user only, and doesn't need administrative rights. The other one will install it for all users.

The installer will automatically detect if Webview2 is not installed and will download it automatically.

### Linux

Binaries are distributed as flatpaks. Install flatpak first and download the flatpack in the releases page.

To install the application natively currently you will have to build it from source.

## Libraries

This application is written in Rust and built upon several components, and among them:
- GUI engine: [Dioxus](https://dioxuslabs.com/), which is itself build over [Wry](https://github.com/tauri-apps/wry)
- Template engine: [Tera](https://keats.github.io/tera/)
- GUI CSS framework: [Bulma](https://bulma.io/)
- Symbols: [Fontawesome](https://fontawesome.com/)

## Building

See the [building](BUILDING.md) page for instructions

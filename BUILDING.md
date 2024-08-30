(work in progress)
# Desktop

## Windows

- Install rust, thenWindows SDK and the WiX toolset version 3
- `cargo build --release --target x86_64-pc-windows-msvc`
- (Windows) install the WiX toolset and run the `createmsi` batch file in `install/` to create the installers

## Linux

- Install rust
- Install the XDO developper library (`xdo-dev` or `xdo-tool`, depending on your distribution)
- `cargo build --release`
- Use the `install-linux.sh` script in `install/` to install the application and the files it needs

# Web
- Install rust
- Install the Dioxus CLI tool: `cargo install dioxus-cli`
- `dx build --platform=web`
- if you want to run the app on a local server, use `dx serve`

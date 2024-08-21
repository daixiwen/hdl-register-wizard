(work in progress)
# Desktop
- Install rust
- Install the Dioxus CLI tool: `cargo install dioxus-cli`
- (Linux) install the XDO developper library (`xdo-dev` or `xdo-tool`, depending on your distribution)
- `cargo build --release`
- (Linux) use the `install-linux.sh` script in `install/` to install the application and the files it needs
- (Windows) install the WiX toolset and run the `createmsi` batch file in `install/` to create the installers

# Web
- Install rust
- Install the Dioxus CLI tool: `cargo install dioxus-cli`
- `dx build --platform=web`
- if you want to run the app on a local server, use `dx serve`

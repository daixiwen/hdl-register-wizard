#!/bin/sh
flatpak remote-add --user --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak-builder --force-clean --user --install-deps-from=flathub --repo=repo --install builddir install/flatpak/org.becoz.hdlregisterwizard.yml
flatpak build-bundle repo hdlregisterwizard-x64-$1.flatpak org.becoz.hdlregisterwizard

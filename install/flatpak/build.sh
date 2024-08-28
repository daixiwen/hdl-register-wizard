#!/bin/sh
flatpak remote-add --user --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak-builder --force-clean --user --install-deps-from=flathub --repo=repo --install builddir install/flatpak/org.becoz.hdlregisterwizard.yml
flatpak build-bundle repo hdlregisterwizard.flatpak org.becoz.hdlregisterwizard

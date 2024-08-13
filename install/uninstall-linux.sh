#!/bin/sh

APP_NAME="hdlregisterwizard"
EXE_NAME="hdl_register_wizard"
SRC_EXEC_PATH="../target/release"/$EXE_NAME
SRC_ASSETS="../src"

APP_PATH=$(systemd-path system-binaries)/$EXE_NAME
SHARED_PATH=$(systemd-path system-shared)
DATA_PATH=$SHARED_PATH/$APP_NAME
ICON_PATH=$SHARED_PATH/icons/hicolor/512x512

DESKTOP_FILE=$SHARED_PATH/applications/${APP_NAME}.desktop

rm $DESKTOP_FILE
rm $APP_PATH
rm $ICON_PATH/${APP_NAME}.png
rm -r $DATA_PATH

echo HDL Register Wizard uninstalled!

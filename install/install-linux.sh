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

#create .desktop file
echo [Desktop Entry] > $DESKTOP_FILE
echo Encoding=UTF-8 >> $DESKTOP_FILE
echo Version=0.1 >> $DESKTOP_FILE
echo Type=Application >> $DESKTOP_FILE
echo Terminal=false >> $DESKTOP_FILE
echo Exec=$APP_PATH >> $DESKTOP_FILE
echo Name=HDL Register Wizard >> $DESKTOP_FILE
echo Icon=$ICON_PATH/${APP_NAME}.png >> $DESKTOP_FILE

# copy binary
cp $SRC_EXEC_PATH $APP_PATH

# copy icon
mkdir -p $ICON_PATH
cp $SRC_ASSETS/icon.png $ICON_PATH/${APP_NAME}.png

# copy assets
mkdir -p $DATA_PATH
cp -r $SRC_ASSETS/css $SRC_ASSETS/templates $SRC_ASSETS/icon.png $DATA_PATH

echo HDL Register Wizard installed!

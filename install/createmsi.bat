@echo creating MSI file using the WiX Toolset...

candle -arch x64 hdlregisterwizard.wxs
light -ext WixUIExtension -cultures:en-us hdlregisterwizard.wixobj
candle hdlregisterwizard-user.wxs
light -ext WixUIExtension -cultures:en-us hdlregisterwizard-user.wixobj
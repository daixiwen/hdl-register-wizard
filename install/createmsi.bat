@echo creating MSI file using the WiX Toolset...

candle hdlregisterwizard.wxs
light -ext WixUIExtension -cultures:en-us hdlregisterwizard.wixobj

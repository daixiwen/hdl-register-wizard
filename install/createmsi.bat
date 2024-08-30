@echo creating MSI file using the WiX Toolset...

candle -arch x64 hdlregisterwizard.wxs
light -ext WixUIExtension -cultures:en-us -out hdlregisterwizard-%1.msi hdlregisterwizard.wixobj
candle hdlregisterwizard-user.wxs
light -ext WixUIExtension -cultures:en-us -out hdlregisterwizard-user-%1.msi hdlregisterwizard-user.wixobj

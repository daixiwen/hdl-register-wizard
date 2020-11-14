# HDL Register Wizard

This is a webapp that can generate VHDL code and documentation to create hardware registers accessible on a memory mapped bus. It can load and save files in the Model Description Format developped by Bitvis for its [Register Wizard](https://bitvis.no/dev-tools/register-wizard/). Files saved by this webapp should be usable by Bitvis' tool.

## Project Status

The project is under development and is not currently usable. The aim for the first release is to be able to load and save MDF files, as the [Bitvis Register Wizard](https://bitvis.no/dev-tools/register-wizard/) currently lacks a GUI.
A future release will also be able to generate code and documentation.

## Howto

1. Start a new project by using the [Edit](edit) menu.
1. Give the project a name
1. Create an interface and choose the protocol
1. Create registers, give them names and set the required parameters
1. Optionally add fields to some registers
1. Save the file with the File menu
1. Use the export menu to generate the required code and documentation

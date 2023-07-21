Here you can change the register parameters.

## General parameters
The name will be used in both the code and the documentation, while the summary and description will appear in the documentation only. The summary is used in the register table, while the descrfiption will be put in a chapter dedicated to the register. If the description is empty (and the register is not a bitfield), then no chapter will be generated for that register and it will only be mentioned in the registers list.

A manual address will fix the register to that address, while an auto setting will make the application assign an address during generation. Check the "Stride" box if you want to generate a register spanning over several addresses. The code will then generate an array for that register. You must specify the count, which will be the number of elements in the array, and optionnally an increment to define the offset in the address between the two elements. If you don't specify an increment, the interface width will be used.

## Register description
The signal type will define which type will be used in the code. Using a single bit type (boolean or std\_logic) with a width higher than 1 is not recommended but possible. In that case the bit will be repeated through the whole width when the register is read, and when written a zero will only be written is all bits are zeros.

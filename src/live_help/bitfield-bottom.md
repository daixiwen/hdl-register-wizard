The signal type will define which type will be used in the code. Using a single bit type (boolean or std\_logic) with a width higher than 1 is not recommended but possible. In that case the bit will be repeated through the whole width when the field is read, and when written a zero will only be written is all bits are zeros.

Reset value is the value the field will have after a reset.

You can define the location of the field if the location of the whole bitfield has been set to "define per field".

The Core Properties let define additional parameters for the signals betweem the generated PIF and the user provided core. "Use read enable" generates a pulse each time the field is read from the interface, while "Use write enable" generates a pulse each time the field is written to.

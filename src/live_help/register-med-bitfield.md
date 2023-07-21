The location can be set to "PIF", meaning that the register will be located within the generated code, or "Core", meaning that it needs to be located withing the user logic. It can also be set to "define per field" and in this case every field must specify wether it is located in the core or the PIF.

## Bitfield

A visual representation of the bitfield then follows. Each bit is shown with a color indicating its status. A black bit is unused. A blue bit is used in one field, and a green bit is used in the currently selected field. A red bit indicates an error, meaning this bit is currently used by several fields.

Use the "New field" button to create a new field. It will then be selected and can be modified. If a field is currently selected and appears in green in the bitfield, it can be deselected with the "Deselect field" button. If several fields are using the same bits, you can use the "Assign bits" button to shift them until all fields aren't overlapping. The "Unassign bits" button shifts back all fields to bit 0\. If you need to reorganize the fields in the bitfield, it can help to use "Unassign bits", change the fields order in the one you want, and then use "Assign bits" to distribute them.

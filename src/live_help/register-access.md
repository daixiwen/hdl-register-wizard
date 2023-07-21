Access defines what valid operations the master can do to the contents. It can be read\-only, write\-only or read/write. The table below explains what is generated with all possible combinations of location and access:

| Access | Location | Description                                                                                                                                                 |
| :----: |  :----:  | :---                                                                                                                                                        |
| RW     | pif      | Normal, Read/Write register in PIF. Core gets signal from PIF with current value.                                                                           |
| RW     | core     | Read/Write register in core. Core gets data value from register access and optionally read/write strobes.                                                   |
| RO     | pif      | Read\-Only register in PIF. Can only have a fixed value (given by reset value). Useful for version registers, but not much else.                             |
| RO     | core     | Read\-Only register in core. Core sends data to PIF. Used for status registers, etc.                                                                         |
| WO     | pif      | Write\-Only register in PIF.                                                                                                                                 |
| WO     | core     | Write\-Only register in core. Useful for functionality where the written data is no longer available, e.g. it was pushed out on an interface or into a FIFO. |
| WO     | core     | (with core properties useWriteEnable = false) Write\-to\-Trigger. Generates a single cycle pulse from PIF to core when written.                               |


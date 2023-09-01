{description}

- type: {interface_type_pretty}
- address size: {address_width} bits
- data width: {data_width} bits
- interface entity name: {pif_name}

## external ports

| Name | Direction | Type | Description |
| :----: |  :----: | :----: | :----: |{{ for port in ports }}
| ``{port.name}`` | {port.direction} | ``{port.port_type}`` | {port.description} |{{ endfor }}

## core to interface record

- name: {core2pif_name}

| Name | Type | Description |
| :----: | :----: | :----: |{{ for register in registers }}{{ for field in register.fields}}{{ for signal in field.core2pif}}
| ``{signal.name}`` | ``{signal.signal_type}`` | {signal.description} |{{ endfor }}{{ endfor }}{{ endfor}}

## interface to core record

- name: {pif2core_name}

| Name | Type | Description |
| :----: | :----: | :----: |{{ for register in registers }}{{ for field in register.fields}}{{ for signal in field.pif2core}}
| ``{signal.name}`` | ``{signal.signal_type}`` | {signal.description} |{{ endfor }}{{ endfor }}{{ endfor}}

## registers

| Address | Name | Type | Access | Description |
| :----: | :----: | :----: | :----: | :----: |{{ for register in registers }}
| {register.address_pretty} | {register.name} | {{ if register.is_bitfield }} bitfield | - {{ else }} {{ for only_field in register.fields }} {only_field.sig_type} | {only_field.rw_mode} {{ endfor }} {{ endif }} | {register.summary} |{{ endfor }}

{{ if regs_doc_details }}
## registers details

{{ for register in registers }}
{{ if register.doc_details }}
### {register.name}

{register.description}

{{ if register.is_bitfield }}
| Position | Name | Type | Access | Description |
| :----: | :----: | :----: | :----: | :----: |{{ for field in fields }}
| {field.position} | {field.name} | {field.sig_type} | {field.rw_mode} | {field.description} |{{endfor}}

{{ endif}}
{{ endif }}
{{ endfor }}
{{ endif }}

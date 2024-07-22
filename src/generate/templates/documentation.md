{%- macro doc_interface(interface) -%}

{{ interface.description }}

- type: {{ interface.interface_type_pretty }}
- address size: {{ interface.address_width }} bits
- data width: {{ interface.data_width }} bits
- interface entity name: {{ interface.pif_name }}


## external ports

| Name | Direction | Type | Description |
| :----: |  :----: | :----: | :----: |
{% for port in interface.ports -%}
| `{{ port.name }}` | {{ port.direction }} | `{{ port.port_type }}` | {{ port.description }} |
{% endfor %}

## core to interface record

name: {{ interface.core2pif_name }}

| Name | Type | Description |
| :----: | :----: | :----: |
{% for register in interface.registers -%}
{%- for field in register.fields -%}
{%- for signal in field.core2pif -%}
| `{{ signal.name }}` | `{{ signal.signal_type }}` | {{ signal.description }} |
{% endfor -%}
{%- endfor -%}
{%- endfor %}

## interface to core record

name: {{ interface.pif2core_name }}

| Name | Type | Description |
| :----: | :----: | :----: |
{% for register in interface.registers -%}
{%- for field in register.fields -%}
{%- for signal in field.pif2core -%}
| `{{ signal.name }}` | `{{ signal.signal_type }}` | {{ signal.description }} |
{% endfor -%}
{%- endfor -%}
{%- endfor %}

## registers

| Address | Name | Type | Access | Description |
| :----: | :----: | :----: | :----: | :----: |
{% for register in interface.registers -%}
| {{ register.address_pretty }} | `{{ register.name }}` | 
{%- if register.is_bitfield -%}
bitfield | - 
{%- else -%}
`{{ register.fields.0.sig_type }}` | {{ register.fields.0.rw_mode }}
{%- endif -%}
| {{ register.summary }} |
{% endfor %}

{%- if interface.regs_doc_details %}
## registers details

{%- for register in interface.registers %}
{%- if register.doc_details %}
### {{ register.name }}

{{ register.description }}

{%- if register.is_bitfield %}
| Position | Name | Type | Access | Description |
| :----: | :----: | :----: | :----: | :----: |
{% for field in fields -%}
| {{ field.position }} | `{{ field.name }}` | `{{ field.sig_type }}` | {{ field.rw_mode }} | {{ field.description }} |
{%- endfor %}

{% endif %}
{% endif %}
{% endfor %}
{% endif %}

{%- endmacro doc_interface -%}
# Documentation for {{ name }}

{%- if single_interface %}
{{ self::doc_interface(interface = interfaces.0) }}
{%- else -%}
{%- for interface in interfaces -%}

# {{ interface.name }}

{{ self::doc_interface(interface = interface) }}
{% endfor %}

{%- endif -%}

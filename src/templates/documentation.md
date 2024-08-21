{%- macro doc_interface(interface) -%}

{{ interface.description | escape_markdown }}

- type: {{ interface.interface_type_pretty }}
- address size: {{ interface.address_width }} bits
- data width: {{ interface.data_width }} bits
- interface entity name: {{ interface.pif_name | escape_markdown }}


## external ports

| Name | Direction | Type | Description |
| :----: |  :----: | :----: | :----: |
{% for port in interface.ports -%}
| `{{ port.name | escape_markdown }}` | {{ port.direction }} | `{{ port.port_type | escape_markdown }}` | {{ port.description | escape_markdown }} |
{% endfor %}

## core to interface record

name: {{ interface.core2pif_name | escape_markdown }}

| Name | Type | Description |
| :----: | :----: | :----: |
{% for register in interface.registers -%}
{%- for field in register.fields -%}
{%- for signal in field.core2pif -%}
| `{{ signal.name | escape_markdown }}` | `{{ signal.signal_type | escape_markdown }}` | {{ signal.description | escape_markdown }} |
{% endfor -%}
{%- endfor -%}
{%- endfor %}

## interface to core record

name: {{ interface.pif2core_name | escape_markdown }}

| Name | Type | Description |
| :----: | :----: | :----: |
{% for register in interface.registers -%}
{%- for field in register.fields -%}
{%- for signal in field.pif2core -%}
| `{{ signal.name | escape_markdown }}` | `{{ signal.signal_type | escape_markdown }}` | {{ signal.description | escape_markdown }} |
{% endfor -%}
{%- endfor -%}
{%- endfor %}

## registers

| Address | Name | Type | Access | Description |
| :----: | :----: | :----: | :----: | :----: |
{% for register in interface.registers -%}
| {{ register.address_pretty }} | `{{ register.name | escape_markdown }}` | 
{%- if register.is_bitfield -%}
bitfield | - 
{%- else -%}
`{{ register.fields.0.sig_type | escape_markdown }}` | {{ register.fields.0.rw_mode }}
{%- endif -%}
| {{ register.summary | escape_markdown }} |
{% endfor %}

{%- if interface.regs_doc_details %}
## registers details

{%- for register in interface.registers %}
{%- if register.doc_details %}
### {{ register.name | escape_markdown }}

{{ register.description | escape_markdown }}

{%- if register.is_bitfield %}
| Position | Name | Type | Access | Description |
| :----: | :----: | :----: | :----: | :----: |
{%- for field in register.fields %}
| {{ field.position }} | `{{ field.name | escape_markdown }}` | `{{ field.sig_type | escape_markdown }}` | {{ field.rw_mode }} | {{ field.description | escape_markdown }} |
{%- endfor %}

{% endif %}
{% endif %}
{% endfor %}
{% endif %}

{%- endmacro doc_interface -%}
# Documentation for {{ name | escape_markdown }}

{%- if single_interface %}
{{ self::doc_interface(interface = interfaces.0) }}
{%- else -%}
{%- for interface in interfaces -%}

# {{ interface.name | escape_markdown }}

{{ self::doc_interface(interface = interface) }}
{% endfor %}

{%- endif -%}

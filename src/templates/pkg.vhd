-- register definitions for {{name}}

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.math_real.all;

package {{ pkg_name }} is

{%- macro pkg_interface(interface) -%}

  -- Register list
  type {{ register_enum_name }} is

  {%- for register in interface.registers -%}
    {{ register.token_name}}{%- if not loop.last %-},{%- endif %}
  {% endfor %};

  -- Addresses list
  {% for register in interface.registers -%}
  constant {{ register.address_const_name }} : integer := 16#{{ register.address_hex}}#;
  {% endfor %};

  {%- if interface.use_stride %-}
  -- Stride constants list
    {%- for register in interface.registers -%}
      {%- if register.is_stride %}
  constant {{ register.stride_count_const_name }} : integer := {{ register.stride_count}};
  constant {{ register.stride_offset_const_name }} : integer := {{ register.stride_increment}};
      {%- endif -%}
    {%- endfor -%};

  -- Stride array types
    {%- for register in interface.registers -%}
      {%- if register.is_stride %}
      type {{ register.stride_array_type }} is array ({{ register.stride_count - 1 }} downto 0) of {{ register.fields.0.sig_type_complete }};
      {%- endif -%}
    {%- endfor -%}
  {%- endif -%}

  -- Address decoding functions
  function {{ interface.address_decoder_name }} (address : unsigned) return {{ register_enum_name }};
  {%- if interface.use_stride %}
  function {{ interface.address_stride_func_name }} (address : unsigned) return integer;
  {% endif -%}

  -- records between core and PIF
  type {{ interface.core2pif_name }} is record
  {% for register in interface.registers -%}
    {% for field in register.fields -%}
      {% for entry in field.core2pif -%}
    {{ entry.name }} : {{ entry.type }};  -- {{ entry.description }}
      {%- endfor -%}    
    {%- endfor -%}    
  {%- endfor -%}    
  end record  {{ interface.core2pif_name }};

  type {{ interface.pif2core_name }} is record
  {% for register in interface.registers -%}
    {% for field in register.fields -%}
      {% for entry in field.pif2core -%}
    {{ entry.name }} : {{ entry.type }};  -- {{ entry.description }}
      {%- endfor -%}    
    {%- endfor -%}    
  {%- endfor -%}    
  end record  {{ interface.pif2core_name }};

{%- endmacro pkg_interface -%}

{%- if single_interface %}
{{ self::pkg_interface(interface = interfaces.0) }}
{%- else -%}
{%- for interface in interfaces -%}

---------------------------------------------------------------------
--
-- Interface {{ interface.name }} ( {{ interface.interface_type_pretty }} )
--
----------------------------------------------------------------------

{{ self::pkg_interface(interface = interface) }}
{% endfor %}

{%- endif -%}

end package {{ pkg_name }};
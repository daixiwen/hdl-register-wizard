-- register definitions for {{name}}

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.math_real.all;

package {{ pkg_name }} is

{%- macro pkg_interface(interface) %}
  -- Addresses list
  {% for register in interface.registers -%}
  constant {{ register.address_const_name }} : integer := 16#{{ register.address_hex}}#;
  {% endfor -%}

  {%- if interface.use_stride %}
  -- Stride constants list
    {%- for register in interface.registers -%}
      {%- if register.is_stride %}
  constant {{ register.stride_count_const_name }} : integer := {{ register.stride_count}};
  constant {{ register.stride_offset_const_name }} : integer := {{ register.stride_increment}};
      {%- endif -%}
    {%- endfor -%}

  -- Stride array types
    {%- for register in interface.registers -%}
      {%- if register.is_stride %}
        {%- if register.is_bitfield %}
          {%- for field in register.fields %}
  type {{ field.stride_array_type }} is array ({{ register.stride_count - 1 }} downto 0) of {{ field.sig_type_complete }};
          {%- endfor -%}
        {%- else %}
  type {{ register.fields.0.stride_array_type }} is array ({{ register.stride_count - 1 }} downto 0) of {{ register.fields.0.sig_type_complete }};
        {%- endif -%}
      {%- endif -%}
    {%- endfor -%}
  {%- endif %}

  {%- if interface.use_bitfield %}
  
  -- Bitfield contstants list
    {%- for register in interface.registers -%}
      {%- if register.is_bitfield %}
        {%- for field in register.fields %}
  constant {{ field.width_const_name }} : integer := {{ field.width }}; -- width for the field {{ field.name }}
  constant {{ field.offset_const_name }} : integer := {{ field.offset }}; -- offset for the field {{ field.name }}
        {%- endfor -%}
      {%- endif -%}
    {%- endfor -%}
  {%- endif %}

  -- Register list
  type {{ interface.register_enum_name }} is (

  {%- for register in interface.registers %}
    {{ register.token_name  }}
      {%- if not loop.last -%} 
        ,
      {%- endif -%}
  {%- endfor %});

  -- Address decoding function
  type {{ interface.address_decoder_return_type }} is record
    address_valid : boolean;  -- if true, the address decoded to an actual register
    reg : {{ interface.register_enum_name }}; -- which register it is decoded to (if address_valid is true)
  {%- if interface.use_stride %}
    stride_num : integer; -- when a stride register is decoded, register number
  {% endif -%}
  end record  {{ interface.address_decoder_return_type }};

  function {{ interface.address_decoder_name }} (address : unsigned) return {{ interface.address_decoder_return_type }};

  -- records between core and PIF
  type {{ interface.core2pif_type }} is record
  {%- for register in interface.registers -%}
    {%- for field in register.fields -%}
      {% for entry in field.core2pif %}
    {{ entry.name }} :
        {%- if register.is_stride and (entry.function == "data") %} {{ field.stride_array_type }}
        {%- else %} {{ entry.signal_type }}
        {%- endif %};  -- {{ entry.description }}
      {%- endfor -%}    
    {%- endfor -%}    
  {%- endfor %}    
  end record  {{ interface.core2pif_type }};

  type {{ interface.pif2core_type }} is record
  {%- for register in interface.registers -%}
    {%- for field in register.fields -%}
      {% for entry in field.pif2core %}
    {{ entry.name }} :
        {%- if register.is_stride and (entry.function == "data") %} {{ field.stride_array_type }}
        {%- else %} {{ entry.signal_type }}
        {%- endif %};  -- {{ entry.description }}
      {%- endfor -%}    
    {%- endfor -%}    
  {%- endfor %}    
  end record  {{ interface.pif2core_type }};

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

{%- endif %}

end package {{ pkg_name }};

package body {{ pkg_name }} is

{%- macro pkg_interface_body(interface) %}
  -- address decoder function
  function {{ interface.address_decoder_name }} (address : unsigned) return {{ interface.address_decoder_return_type }} is
    variable return_value : {{ interface.address_decoder_return_type }};
  begin

    return_value.address_valid := false;
  {%- if interface.use_stride %}
    return_value.stride_num := 0;
  {%- endif %}
  {% if interface.use_not_stride %}
    case to_integer(address) is

    {%- for register in interface.registers %}
      {%- if not register.is_stride %}
      when {{ register.address_const_name}} =>
        return_value.address_valid := true;
        return_value.reg := {{ register.token_name}};
      {% endif -%}
    {%- endfor -%}
      when others =>
  {%- endif %}
  {%- for register in interface.registers %}
    {%- if register.is_stride %}

        -- {{ register.name }}
        for i in 0 to {{ register.stride_count_const_name }} - 1 loop
          if to_integer(address) = {{ register.address_const_name }} + i * {{ register.stride_offset_const_name }} then
            return_value.address_valid := true;
            return_value.reg := {{ register.token_name}};
            return_value.stride_num := i; 
          end if;       
        end loop;
    {% endif -%}
  {%- endfor -%}
  {% if interface.use_not_stride %}
    end case;
  {%- endif %}
    return return_value;
  end function  {{ interface.address_decoder_name }};

{%- endmacro pkg_interface_body -%}

{%- if single_interface %}
{{ self::pkg_interface_body(interface = interfaces.0) }}
{%- else -%}
{%- for interface in interfaces -%}

---------------------------------------------------------------------
--
-- Interface {{ interface.name }} ( {{ interface.interface_type_pretty }} )
--
----------------------------------------------------------------------

{{ self::pkg_interface_body(interface = interface) }}
{% endfor %}

{%- endif %}

end package body {{ pkg_name }};
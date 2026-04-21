-- top level entity for {{ name }}

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.math_real.all;

use work.{{ pkg_name }}.all;

entity {{ top_name }} is
  port (
      -- clock
    clk : in std_logic;
      -- reset
    reset : in std_logic
{%- for interface in interfaces -%}
  {%- for port in interface.ports -%};
      -- {{ port.description }}
    {{ port.name }} : {{ port.direction }} {{ port.port_type }}
  {%- endfor -%}
{%- endfor -%}
  );
end entity {{ top_name }};

architecture rtl of {{ top_name }} is
  
{%- for interface in interfaces %}
  signal {{ interface.core2pif_name }} : {{ interface.core2pif_type }};
  signal {{ interface.pif2core_name }} : {{ interface.pif2core_type }};
{%- endfor %}

begin
  
  -- Core instantiation
  {{ core_instance }} : entity work.{{ core_name }}
    port map (
      clk => clk,
      reset => reset
{%- for interface in interfaces -%},
      {{ interface.core2pif_name }} => {{ interface.core2pif_name }},
      {{ interface.pif2core_name }} => {{ interface.pif2core_name }}
{%- endfor -%});
{% for interface in interfaces -%}
  {%- if single_interface %}
  -- PIF instantiation
  {%- else -%}
  -- PIF instantiation for {{ interface.name }}
  {%- endif %}
  {{ interface.pif_instance }} : entity work.{{ interface.pif_name }}
    port map (
      clk => clk,
      reset => reset,
      {{ interface.core2pif_name }} => {{ interface.core2pif_name }},
      {{ interface.pif2core_name }} => {{ interface.pif2core_name }}
  {%- for port in interface.ports -%},
      {{ port.name }} => {{ port.name }}
  {%- endfor -%});
{% endfor %}
end architecture rtl;
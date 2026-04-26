-- interface entity for {{ name }} (project {{ global.name }} )

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.math_real.all;

use work.{{ global.pkg_name }}.all;

entity {{ pif_name }} is
  port (
      -- clock
    clk : in std_logic;
      -- reset
    reset : in std_logic;
      -- connection to core
    {{ core2pif_name }} : in  {{ core2pif_type }};
    {{ pif2core_name }} : out {{ pif2core_type }}
{%- for port in ports -%};
      -- {{ port.description }}
    {{ port.name }} : {{ port.direction }} {{ port.port_type }}
{%- endfor -%}
  );
end entity {{ pif_name }};

architecture rtl of {{ pif_name }} is
  
begin

  p_pif : process (clk, reset)
  begin
    if reset = '1' then
      -- initialize interface signals
      {{ ports_names.rdata }} <= (others => '0');
      {{ ports_names.ready }} <= '0';

      -- initialize output record
      pif2core <= (
{%- set notfirst = false -%}
{%- for register in registers -%}
  {%- for field in register.fields -%}
    {%- for entry in field.pif2core %}{% if notfirst %},{% endif %}
        {{ entry.name }} => 
      {%- if register.is_stride %} ( others => {% endif -%}
      {%- if (entry.function == "read_enable") or (entry.function == "write_enable") %} false
      {%- else %} {{ field.reset }}
      {%- endif -%}
      {%- if register.is_stride %} ) {% endif -%}
      {%- set_global notfirst = true -%}
    {%- endfor -%}    
  {%- endfor -%}    
{% endfor %}
      );   

    elsif rising_edge(clk) then
      
    end if;
  end process;

end architecture rtl;
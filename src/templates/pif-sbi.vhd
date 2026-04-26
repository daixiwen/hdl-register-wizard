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

  -- Add Core code here

end architecture rtl;
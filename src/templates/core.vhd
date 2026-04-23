-- core entity for {{ name }}

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.math_real.all;

use work.{{ pkg_name }}.all;

entity {{ core_name }} is
  port (
      -- clock
    clk : in std_logic;
      -- reset
    reset : in std_logic
{%- for interface in interfaces -%};
      -- connection to interface {{ interface.name }}
    {{ interface.pif2core_name }} : in  {{ interface.pif2core_type }};
    {{ interface.core2pif_name }} : out {{ interface.core2pif_type }}
{%- endfor -%}
  );
end entity {{ core_name }};

architecture rtl of {{ core_name }} is
  
begin

  -- Add Core code here

end architecture rtl;
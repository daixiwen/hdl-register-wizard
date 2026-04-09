-- register definitions for irqc

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.math_real.all;

package irqc_pkg is
-- Register list
  type t_regs isIER
  IRR
  IPR
  ICR
  ITR
  IRQ2CPU_ENA
  IRQ2CPU_allowed
  ;

  -- Addresses list
  constant c_irqc_ier_addr : integer := 16#0#;
  constant c_irqc_irr_addr : integer := 16#4#;
  constant c_irqc_ipr_addr : integer := 16#8#;
  constant c_irqc_icr_addr : integer := 16#c#;
  constant c_irqc_itr_addr : integer := 16#10#;
  constant c_irqc_irq2cpu_ena_addr : integer := 16#14#;
  constant c_irqc_irq2cpu_allowed_addr : integer := 16#18#;
  ;-- Address decoding functions
  function f_address_decode (address : unsigned) return t_regs;-- records between core and PIF
  type core2pif is record
  irr : std_logic_vector(31 downto 0);  -- data for IRRipr : std_logic_vector(31 downto 0);  -- data for IPRirq2cpu_allowed : std_logic;  -- data for IRQ2CPU_allowedend record  core2pif;

  type pif2core is record
  ier : std_logic_vector(31 downto 0);  -- data for IERicr : std_logic_vector(31 downto 0);  -- data for ICRitr : std_logic_vector(31 downto 0);  -- data for ITRirq2cpu_ena : std_logic;  -- data for IRQ2CPU_ENAirq2cpu_ena_we : boolean;  -- signals that IRQ2CPU_ENA is being writtenend record  pif2core;end package irqc_pkg;
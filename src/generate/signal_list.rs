
use crate::generate::genmodel;
use crate::generate::tokenlist;
use std::error::Error;
use crate::file_formats::mdf;

/// direction of a signal (for a port)
pub enum SignalDirection {
    In,
    Out,
//    InOut
}

/// definition for a signal
pub struct SignalDef<'a> {
    /// part of the name specific to this signal
    token_name : &'a str,
    /// template used to generate the vhdl type of the signal
    type_template: &'a str,
    /// direction
    direction: SignalDirection,
    /// signal description 
    description: &'a str,
    /// attribute used by xilinx to identify this signal
    xilinx_attr: &'a str
}

/// templates for signal names. Different templates can be used depending on direction
pub struct SignalTemplates<'a> {
    /// template for inputs
    template_in : &'a str,
    /// template for outputs
    template_out : &'a str
}

/// table of signals for SBI
const SBI_SIGNALS : [SignalDef<'static>;7] = [
    SignalDef {
        token_name : "cs",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "chip select",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "addr",
        type_template : "unsigned({{ address_width - 1 }} downto 0)",
        direction : SignalDirection::In,
        description : "address",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "rena",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "read command",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "wena",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "write command",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "rdata",
        type_template : "std_logic_vector({{ data_width - 1 }} downto 0)",
        direction : SignalDirection::Out,
        description : "read back data",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "wdata",
        type_template : "std_logic_vector({{ data_width - 1 }} downto 0)",
        direction : SignalDirection::In,
        description : "data to write",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "ready",
        type_template : "std_logic",
        direction : SignalDirection::Out,
        description : "ready signal",
        xilinx_attr : ""
    },
];

/// template for SBI signal names
const SBI_TEMPLATES : SignalTemplates<'static> = SignalTemplates {
    template_in : "{{ interface }}_{{ signal }}*",
    template_out : "{{ interface }}_{{ signal }}*",
};

/// table of signals for APB3
const APB3_SIGNALS : [SignalDef<'static>;8] = [
    SignalDef {
        token_name : "penable",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "enable signal",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "paddr",
        type_template : "std_logic_vector({{ address_width - 1 }} downto 0)",
        direction : SignalDirection::In,
        description : "address",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "psel",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "slave select command",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "pwrite",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "write indication",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "prdata",
        type_template : "std_logic_vector({{ data_width - 1 }} downto 0)",
        direction : SignalDirection::Out,
        description : "read back data",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "pwdata",
        type_template : "std_logic_vector({{ data_width - 1 }} downto 0)",
        direction : SignalDirection::In,
        description : "data to write",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "pready",
        type_template : "std_logic",
        direction : SignalDirection::Out,
        description : "ready signal",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "pslverr",
        type_template : "std_logic",
        direction : SignalDirection::Out,
        description : "slave error",
        xilinx_attr : ""
    },
];

/// template for APB3d signal names
const APB3_TEMPLATES : SignalTemplates<'static> = SignalTemplates {
    template_in : "apbs_{{ interface }}_{{ signal }}*",
    template_out : "apbs_{{ interface }}_{{ signal }}*",
};

/// table of signals for avalon memory mapped
const AVALON_SIGNALS : [SignalDef<'static>;5] = [
    SignalDef {
        token_name : "address",
        type_template : "std_logic_vector({{ address_width - 1 }} downto 0)",
        direction : SignalDirection::In,
        description : "address",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "read",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "read command",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "write",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "write command",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "readdata",
        type_template : "std_logic_vector({{ data_width - 1 }} downto 0)",
        direction : SignalDirection::Out,
        description : "read back data",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "writedata",
        type_template : "std_logic_vector({{ data_width - 1 }} downto 0)",
        direction : SignalDirection::In,
        description : "data to write",
        xilinx_attr : ""
    },
];

/// template for Avalon memory mapped signal names
const AVALON_TEMPLATES : SignalTemplates<'static> = SignalTemplates {
    template_in : "avs_{{ interface }}_{{ signal }}*",
    template_out : "avs_{{ interface }}_{{ signal }}*",
};

/// table of signals for AXI4 light
const AXI4L_SIGNALS : [SignalDef<'static>;16] = [
    SignalDef {
        token_name : "awaddr",
        type_template : "std_logic_vector({{ address_width - 1 }} downto 0)",
        direction : SignalDirection::In,
        description : "write address channel address",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "awready",
        type_template : "std_logic",
        direction : SignalDirection::Out,
        description : "write address channel ready",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "awvalid",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "write address channel valid",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "wdata",
        type_template : "std_logic_vector({{ data_width - 1 }} downto 0)",
        direction : SignalDirection::In,
        description : "write data channel data",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "wready",
        type_template : "std_logic",
        direction : SignalDirection::Out,
        description : "write data channel ready",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "wvalid",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "write data channel valid",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "bresp",
        type_template : "std_logic_vector(1 downto 0)",
        direction : SignalDirection::In,
        description : "write response channel response",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "bready",
        type_template : "std_logic",
        direction : SignalDirection::Out,
        description : "write response channel ready",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "bvalid",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "write response channel valid",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "araddr",
        type_template : "std_logic_vector({{ address_width - 1 }} downto 0)",
        direction : SignalDirection::In,
        description : "read address channel address",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "arready",
        type_template : "std_logic",
        direction : SignalDirection::Out,
        description : "read address channel ready",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "arvalid",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "read address channel valid",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "rdata",
        type_template : "std_logic_vector({{ data_width - 1 }} downto 0)",
        direction : SignalDirection::Out,
        description : "read data channel data",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "rready",
        type_template : "std_logic",
        direction : SignalDirection::In,
        description : "read data channel ready",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "rvalid",
        type_template : "std_logic",
        direction : SignalDirection::Out,
        description : "read data channel valid",
        xilinx_attr : ""
    },
    SignalDef {
        token_name : "rresp",
        type_template : "std_logic_vector(1 downto 0)",
        direction : SignalDirection::Out,
        description : "read data channel response",
        xilinx_attr : ""
    },
];

/// template for Avalon memory mapped signal names
const AXI4L_TEMPLATES : SignalTemplates<'static> = SignalTemplates {
    template_in : "s_{{ interface }}_{{ signal }}*",
    template_out : "s_{{ interface }}_{{ signal }}*",
};

/// generate a GenIntPort from a signal definition
pub fn to_gen_int_port(definition: &SignalDef::<'_>, templates: &SignalTemplates::<'_>, context: &tera::Context, general_token_list : &mut tokenlist::TokenList) -> Result<genmodel::GenIntPort, Box<dyn Error>> {
    let mut new_context = context.clone();
    new_context.insert("signal", definition.token_name);

    // template engine
    let name_template = match definition.direction {
        SignalDirection::In => templates.template_in,
        SignalDirection::Out => templates.template_out
    };
    //tt.add_template("type", definition.type_template)?;

    // build all the elements of the GenIntPort structure
    let function = definition.token_name.to_owned();
    let name = general_token_list.generate_token(&tera::Tera::one_off(&name_template, &new_context, false)?);
    let port_type = tera::Tera::one_off(definition.type_template, &new_context, false)?;
    let direction = match &definition.direction {
        SignalDirection::In => "in".to_owned(),
        SignalDirection::Out => "out".to_owned()
    };
    let description = definition.description.to_owned();
    let xilinx_attr = definition.xilinx_attr.to_owned();

    Ok(genmodel::GenIntPort {
        function,
        name,
        port_type,
        direction,
        description,
        xilinx_attr
    })
}

/// generate a port list for the given interface type
pub fn to_port_list(interface_type : mdf::InterfaceType, context: &tera::Context, general_token_list : &mut tokenlist::TokenList) -> Result<Vec<genmodel::GenIntPort>, Box<dyn Error>> {
    // choose the right definitions list and name templates
    
    let (defs, templates) = match interface_type {
        mdf::InterfaceType::SBI => (SBI_SIGNALS.iter(), &SBI_TEMPLATES),
        mdf::InterfaceType::APB3 => (APB3_SIGNALS.iter(), &APB3_TEMPLATES),
        mdf::InterfaceType::AvalonMm => (AVALON_SIGNALS.iter(), &AVALON_TEMPLATES),
        mdf::InterfaceType::AXI4Light => (AXI4L_SIGNALS.iter(), &AXI4L_TEMPLATES)
    };
    
    // apply the templates to the signal list and return it
    defs.map(|x| to_gen_int_port(x, templates, &context, general_token_list)).collect()
}

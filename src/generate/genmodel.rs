//! Model description adapted for generation
//! 
//! This model removes all "optional" or "automatic" information and fills everything with
//! determinate information, either computed or from the original model. It also holds
//! additionnal information, such as VHDL identifiers.
//! A lot of information in this model is redundant, but it makes templates writing easyer

use serde::Serialize;
use crate::file_formats::mdf;
use crate::settings::Settings;
use std::collections::HashMap;
use std::error::Error;
use super::tokenlist::{TokenList, to_vhdl_token};
use crate::utils;
use crate::page::PageType;  
use crate::generate::generror::GenError;
use super::signal_list;
use super::user_strings;
use tera::Tera;

/// Project model for generation
#[derive(Serialize)]
pub struct GenModel {
    /// project name
    pub name : String,
    /// name used for token generation 
    pub token_name : String,
    /// top level entity name
    pub top_name : String,
    /// core vhdl entity name
    pub core_name : String,
    /// core instance name in top level
    pub core_instance : String,
    /// package name with all the definitions
    pub pkg_name : String,
    /// if true, only has one interface
    pub single_interface : bool,
    /// list of interfaces
//    #[serde(skip)]
    pub interfaces : Vec<GenInterface>,
}

impl GenModel {
    /// take a Mdf model and convert it to a GenModel
    pub fn from_model(model: &mdf::Mdf, settings: &Settings, templates: &Tera) -> Result<Self, Box<dyn Error>> {
        let mut token_list = TokenList::new();

        let name = model.name.clone();

        let token_name = to_vhdl_token(&name);

        let single_interface = model.interfaces.len() == 1;

        let mut context = tera::Context::new();
        context.insert("project", &token_name);

        let top_name = token_list.generate_token(&templates.render(user_strings::GM_TOP_NAME, &context)?);
        let core_name = token_list.generate_token(&templates.render(user_strings::GM_CORE_NAME, &context)?);
        let core_instance = token_list.generate_token(&templates.render(user_strings::GM_CORE_INSTANCE, &context)?);
        let pkg_name = token_list.generate_token(&templates.render(user_strings::GM_PKG_NAME, &context)?);

        // apply a conversion to each interface
        let interfaces = model.interfaces.iter().enumerate().map(
            |(n, interface)| GenInterface::from_interface(
                interface, PageType::Interface(n), settings, templates, &token_name, 
                &single_interface, &mut token_list)
        ).collect::<Result<Vec<GenInterface>, Box<dyn Error>>>()?;
    

        Ok(GenModel {
            name,
            token_name : token_name.clone(),
            top_name,
            core_name,
            core_instance,
            pkg_name,
            single_interface,
            interfaces
        })
    }
}

/// Interface model for generation
#[derive(Serialize)]
pub struct GenInterface {
    /// interface name
    pub name: String,
    /// name used for token generation (empty if project has only one interface)
    pub token_name: String,
    /// name for the pif entity
    pub pif_name : String,
    /// name for the pif instance in top level
    pub pif_instance : String,   
    /// name for the core2pif record
    pub core2pif_name : String,
    /// name for the pif2core record
    pub pif2core_name : String,
    /// name for the register enum
    pub register_enum_name : String,
    /// name for the address decoder function
    pub address_decoder_name : String,
    /// name for the address stride number function
    pub address_stride_func_name : String,  
    /// name for the address width constant
    pub address_width_const_name : String,
    /// name for the data width constant
    pub data_width_const_name : String,
    /// description for the interface
    pub description: String,
    /// interface type (protocol used)
    pub interface_type: mdf::InterfaceType,
    /// interface type (as a string for documentation)
    pub interface_type_pretty : String,
    /// width of the address bus.
    pub address_width: u32,
    /// width of the data bus.
    pub data_width: u32,
    /// if true, some registers are arrays
    pub use_stride: bool,
    /// if true, some registers are non arrays
    pub use_not_stride: bool,
    /// list of interface porte
    pub ports: Vec<GenIntPort>,
    // list of signals for interface as a map (with function as index and name as value)
    pub ports_names: HashMap<String, String>,
    /// if true, some registers have details for the documentation
    pub regs_doc_details : bool,
    /// list of registers
    pub registers : Vec<GenRegister>,
}

/// Interface port model
#[derive(Serialize)]
pub struct GenIntPort {
    /// port function
    pub function : String,
    /// vhdl name of the port
    pub name : String,
    /// port type
    pub port_type: String,
    /// direction
    pub direction: String,
    /// signal description 
    pub description: String,
    /// attribute used by xilinx to identify this signal
    pub xilinx_attr: String
}

impl GenInterface {
    /// take a Mdf interface and convert it to a GenInterface
    pub fn from_interface(interface: &mdf::Interface, page: PageType, settings: &Settings, templates: &Tera, project_token_name : &String, single_interface : &bool, general_token_list : &mut TokenList) -> Result<Self, Box<dyn Error>> {

        // duplicate the interface and assign an address to all registers
        let mut interface = interface.clone();
        interface.assign_addresses()?;

        let name = interface.name.clone();
        let token_name = if *single_interface { "".to_owned()} else {to_vhdl_token(&name)};

        let description = utils::opt_vec_str_to_textarea(&interface.description);
        let interface_type = interface.interface_type;
        let interface_type_pretty = match interface_type {
            mdf::InterfaceType::SBI => "SBI",
            mdf::InterfaceType::APB3 => "APB3",
            mdf::InterfaceType::AvalonMm => "Avalon memory mapped",
            mdf::InterfaceType::AXI4Light => "AXI4 Light",
        }.to_owned();
        let address_width = match interface.get_address_width() {
            Some(width) => width,
            None => Err(GenError::new(&page, &format!("couldn't determine interface {} address width", name)))?
        };
        let data_width = match interface.get_data_width() {
            Some(width) => width,
            None => Err(GenError::new(&page, &format!("couldn't determine interface {} data width", name)))?
        };
        // go through all the registers and check if at least one uses an address stride
        let use_stride = interface.registers.iter().fold(false, 
            | use_stride, reg  | { use_stride || reg.address.stride.is_some() } );
        let use_not_stride = interface.registers.iter().fold(false, 
            | use_not_stride, reg  | { use_not_stride || reg.address.stride.is_none() } );
            
        let mut context = tera::Context::new();
        context.insert("project", project_token_name);
        context.insert("interface", &token_name);

        let pif_name = general_token_list.generate_token(&templates.render(user_strings::GI_PIF_NAME, &context)?);
        let pif_instance = general_token_list.generate_token(&templates.render(user_strings::GI_PIF_INSTANCE, &context)?); 
        let core2pif_name = general_token_list.generate_token(&templates.render(user_strings::GI_CORE2PIF_NAME, &context)?);
        let pif2core_name = general_token_list.generate_token(&templates.render(user_strings::GI_PIF2CORE_NAME, &context)?); 
        let register_enum_name = general_token_list.generate_token(&templates.render(user_strings::GI_REGISTER_ENUM_NAME, &context)?); 
        let address_decoder_name = general_token_list.generate_token(&templates.render(user_strings::GI_ADDRESS_DECODER_NAME, &context)?);
        let address_stride_func_name = general_token_list.generate_token(&templates.render(user_strings::GI_ADDRESS_STRIDE_FUNC_NAME, &context)?); 
        let address_width_const_name = general_token_list.generate_token(&templates.render(user_strings::GI_ADDRESS_WIDTH_CONST_NAME, &context)?); 
        let data_width_const_name = general_token_list.generate_token(&templates.render(user_strings::GI_DATA_WIDTH_CONST_NAME, &context)?); 


        let mut port_context = tera::Context::new();
        port_context.insert("project", &project_token_name);
        port_context.insert("interface", &token_name);
        port_context.insert("address_width", &address_width);
        port_context.insert("data_width", &data_width);

        let ports = signal_list::to_port_list(interface_type, &port_context, general_token_list)?;

        // make a second ports list, a hashmap from function to name
        let ports_names : HashMap<String, String> = ports.iter().map(
            | signal | (signal.function.clone(), signal.name.clone())).collect();

        // go through all the registers and add them to the list
        let registers =  match page {
            PageType::Interface(int_num) => {
                let mut corfe2pif_token_list = TokenList::new();
                let mut pif2core_token_list = TokenList::new();

                interface.registers.iter().enumerate().map(|(n, register)| GenRegister::from_register(
                    register, PageType::Register(int_num,n, None),
                    settings, templates, project_token_name, &token_name, data_width,
                    general_token_list, &mut corfe2pif_token_list, &mut pif2core_token_list))
                    .collect::<Result<Vec<GenRegister>,  Box<dyn Error>>>()?},
            _ => Err(GenError::new(&page, "wrong value for the page parameter in register call"))?
        };

        // go through all registers to see if some have some doc details
        let regs_doc_details = registers.iter().fold(false, |prev, reg| { prev || reg.doc_details} );

        Ok(GenInterface { 
            name, 
            token_name: token_name.clone(), 
            pif_name, 
            pif_instance, 
            core2pif_name,
            pif2core_name, 
            register_enum_name, 
            address_decoder_name, 
            address_stride_func_name, 
            address_width_const_name,
            data_width_const_name,
            description, 
            interface_type, 
            interface_type_pretty, 
            address_width, 
            data_width, 
            use_stride,
            use_not_stride,
            ports,
            ports_names,
            regs_doc_details,
            registers})

    }
}

/// register model for generation
/// 
/// field is always used. If the register is not a bitfield, details about
/// the register type and value will be in a unique GenField element
#[derive(Serialize)]
pub struct GenRegister {
    /// register name
    pub name: String,
    /// name used for token generation
    pub token_name: String,
    /// name used for the constant with the address
    pub address_const_name : String,
    /// address (hexadecimal) excluding quotes
    pub address_hex : String,
    /// address for display in documentation
    pub address_pretty : String,
    /// if true, register is an array
    pub is_stride : bool,
    /// quick description
    pub summary : String,
    /// longer description
    pub description : String,
    /// if true, is a bitfield
    pub is_bitfield : bool,
    /// if true, documentation has more details (either because it is a bitfield, or it has a description field)
    pub doc_details : bool,
    /// name used for the constant with the array length (only valid if is_stride = true)    
    pub stride_count_const_name : String,
    /// name used for the constant with the address offset between array elements (only valid if is_stride = true)    
    pub stride_offset_const_name : String,
    /// name used for the array type (only valid if is_stride = true)
    pub stride_array_type : String,
    /// array length (only valid if is_stride = true)
    pub stride_count : u32,
    /// address offset between array elements (only valid if is_stride = true)
    pub stride_increment : u32,
    /// if true, array addresses are continuous (only valid if is_stride = true)
    pub stride_continuous : bool,
    /// fields (if the register is not a bitfield, holds a single element with the register description)
    pub fields : Vec<GenField>
}

/// field model for generation
#[derive(Serialize)]
pub struct GenField {
    /// field name
    pub name : String,
    /// field description
    pub description : String,
    /// field width
    pub width : u32,
    /// if true, the field is the same size than the interface data width
    pub width_matches_interface : bool,
    /// field least significant bit (offset)
    pub offset : u32,
    /// field position (msb..lsb) as a string
    pub position : String,
    /// name of the constant for the field width
    pub width_const_name : String,
    /// name of the constant for the field lsb (offset)
    pub offset_const_name : String,
    /// read-write mode
    pub rw_mode : String,
    /// read access
    pub is_read : bool,
    /// write access
    pub is_write : bool,
    /// field type (only valid if not a bitfield)
    pub sig_type : String,
    /// complete type, including vector downto size
    pub sig_type_complete : String,
    /// true if type is bit
    pub sig_type_is_bit : bool,
    /// true if type is bool
    pub sig_type_is_bool : bool,
    /// true if type is a vector
    pub sig_type_is_vector : bool,
    /// field reset value, including quotes if required
    pub reset : String,
    /// field location
    pub is_in_core : bool,
    /// read enable
    pub core_read_enable : bool,
    /// write enable
    pub core_write_enable : bool,    
    /// if true, there is a data signal in core2pif
    pub core2pif_has_data : bool,
    /// if true, there is a data signal in pif2core
    pub pif2core_has_data : bool,
    /// signals from this register in the core2pif record
    pub core2pif: Vec<GenStructSignal>,
    // list of signals for core2pif as a map (with function as index and name as value)
    pub core2pif_names: HashMap<String, String>,
    /// signals from this register in the pif2core record
    pub pif2core: Vec<GenStructSignal>,
    // list of signals for core2pif as a map (with function as index and name as value)
    pub pif2core_names: HashMap<String, String>,
}

/// signal element in the core2pif and pif2core records
#[derive(Serialize)]
pub struct GenStructSignal {
    /// signal function
    pub function : String,
    /// vhdl name of the signal
    pub name : String,
    /// signal type
    pub signal_type: String,
    /// signal description 
    pub description: String,
}

/// create a GenStructSignal using the given templates
pub fn gen_registersignal(templates: &Tera, function: &str, name_template : &str, full_type : &str, description_template : &str, context: &tera::Context, token_list: &mut TokenList) -> Result<GenStructSignal, Box<dyn Error>>
{
    let name = token_list.generate_token(&templates.render(name_template, context)?);
    let description = templates.render(description_template, context)?; 

    Ok(GenStructSignal { 
        function: function.to_owned(), 
        name, 
        signal_type : full_type.to_owned(), 
        description})
}

/// convert a list of GenStructSignal to a map of names
pub fn gen_names_map(signals: &[GenStructSignal]) -> HashMap<String, String> {
    signals.iter().map(|signal| (signal.function.clone(), signal.name.clone())).collect()
}

impl GenRegister {
    /// take a Mdf register and convert it to a GenRegister
    pub fn from_register(register: &mdf::Register, page: PageType, settings: &Settings, templates: &Tera, project_token_name : &String, interface_token_name : &String, interface_data_width: u32, general_token_list : &mut TokenList, corfe2pif_token_list : &mut TokenList, pif2core_token_list : &mut TokenList) -> Result<Self, Box<dyn Error>> {

        let name = register.name.clone();
        let token_name = to_vhdl_token(&name);
        let address_hex = format!("{:x}", register.address.value.ok_or("address not defined")?.value);
        let address_pretty = register.address.nice_str();
        let is_stride = register.address.stride.is_some();
        let summary = utils::opt_vec_str_to_textarea(&register.summary);
        let description = utils::opt_vec_str_to_textarea(&register.description);
        let is_bitfield = register.signal.is_none();
        let doc_details = is_bitfield || !description.is_empty();

        // the fields: either a single field with the register, or a bunch of fields
        let fields = if !is_bitfield {
            let width = register.width.unwrap_or(interface_data_width);
            let width_matches_interface = width == interface_data_width;

            // use templates for names and tokens
            let mut context = tera::Context::new();
            context.insert("project", project_token_name);
            context.insert("interface", &interface_token_name);
            context.insert("register", &token_name);
            context.insert("full_name", &name);
            context.insert("data_width", &width);

            let position = if width == 1 {
                "0".to_owned()
            } else {
                format!("{}..0", width-1)};

            let width_const_name = general_token_list.generate_token(&templates.render(user_strings::GR_WIDTH_CONST_NAME, &context)?);

            let rw_mode = register.access.ok_or(GenError::new(&page,"access type needed for register"))?;
            let is_read = rw_mode != mdf::AccessType::WO;
            let is_write = rw_mode != mdf::AccessType::RO;
                // rw_mode should be a string
            let rw_mode = rw_mode.to_string();
            let sig_type = register.signal.unwrap().to_string();
            let sig_type_complete = match register.signal {
                Some(utils::SignalType::Boolean) | Some(utils::SignalType::StdLogic) => sig_type.clone(),
                _ => format!("{}({} downto 0)", &sig_type, width-1)
            };
    
            let sig_type_is_bit = register.signal == Some(utils::SignalType::StdLogic);
            let sig_type_is_bool = register.signal == Some(utils::SignalType::Boolean);
            let sig_type_is_vector = (!sig_type_is_bit) && (!sig_type_is_bool);
    
            let reset = match register.reset {
                None => Err(GenError::new(&page,"reset value not specified"))?,      // non bitfield, we must have a value
                Some(reset_value) => match register.signal.unwrap() {
                    // the way we format the value depends on the type
                    utils::SignalType::Boolean => match reset_value.value {
                            0 => "false".to_owned(),
                            _ => "true".to_owned()
                        },

                    utils::SignalType::StdLogic => match reset_value.value {
                            0 => "'0'".to_owned(),
                            _ => "'1'".to_owned()
                        },
                        
                    _ => format!("{}x\"{:x}\"",width,reset_value.value)
                }
            };
    
            let is_in_core = register.location.ok_or(GenError::new(&page,"location for register {} needs to be specified"))? == mdf::LocationType::Core;
    
            let core_read_enable = register.core_signal_properties.use_read_enable.unwrap_or(false);
            let core_write_enable = register.core_signal_properties.use_write_enable.unwrap_or(false);

            let core2pif_has_data = is_in_core && is_read;
            let pif2core_has_data = is_write;

            // build the core2pif and pif2core elements
            let mut core2pif : Vec<GenStructSignal> = Default::default();
            let mut pif2core : Vec<GenStructSignal> = Default::default();
            
            if core2pif_has_data {
                core2pif.push(gen_registersignal(templates, "data", user_strings::GR_DATA_NAME, &sig_type_complete, user_strings::GR_DATA_DESCRIPTION, &context, corfe2pif_token_list)?);
            }
            if pif2core_has_data {
                pif2core.push(gen_registersignal(templates, "data", user_strings::GR_DATA_NAME, &sig_type_complete, user_strings::GR_DATA_DESCRIPTION, &context, pif2core_token_list)?);
            }
            if core_read_enable {
                pif2core.push(gen_registersignal(templates, "read_enable",user_strings::GR_READ_ENABLE_NAME, "boolean", user_strings::GR_READ_ENABLE_DESCRIPTION, &context, pif2core_token_list)?);
            }
            if core_write_enable {
                pif2core.push(gen_registersignal(templates, "write_enable", user_strings::GR_WRITE_ENABLE_NAME, "boolean", user_strings::GR_WRITE_ENABLE_DESCRIPTION, &context, pif2core_token_list)?);
            }

            let core2pif_names = gen_names_map(&core2pif);
            let pif2core_names = gen_names_map(&pif2core);

            
            let unique_field = GenField {
                name : Default::default(),
                description : Default::default(),
                width,
                width_matches_interface,
                offset: 0,
                position,
                width_const_name,
                offset_const_name: Default::default(),
                rw_mode,
                is_read,
                is_write,
                sig_type,
                sig_type_complete,
                sig_type_is_bit,
                sig_type_is_bool,
                sig_type_is_vector,
                reset,
                is_in_core,
                core_read_enable,
                core_write_enable,
                core2pif_has_data,
                pif2core_has_data,
                core2pif,
                core2pif_names,
                pif2core,
                pif2core_names
            };

            vec![unique_field]
        } else {
            // this is a bitfield, we need to convert each field
            match page {
                PageType::Register(int_num, reg_num, None) => 
                    register.fields.iter().enumerate().map(|(n, field)| GenField::from_field(
                        register, field, PageType::Register(int_num,reg_num, Some(n)),
                        settings, templates, project_token_name, interface_token_name, interface_data_width,
                        &token_name, general_token_list, corfe2pif_token_list,
                        pif2core_token_list)).collect::<Result<Vec<GenField>,  Box<dyn Error>>>()?,
                _ => Err(GenError::new(&page, "wrong value for the page parameter in register call"))?
            }
        };

        // use templates for names and tokens
        let mut context = tera::Context::new();
        context.insert("project", project_token_name);
        context.insert("interface", &interface_token_name);
        context.insert("register", &token_name);
        context.insert("full_name", &name);
        context.insert("data_width", &interface_data_width);
        
        
        let stride_count = match &register.address.stride {
            None => 1,
            Some(stride) => stride.count.value
        } as u32;

        let stride_increment = match &register.address.stride {
            None => interface_data_width,
            Some(stride) => match stride.increment {
                None => interface_data_width,
                Some(increment_value) => increment_value.value as u32
            }
        };
        let stride_continuous = stride_increment == (interface_data_width + 7)/8;

        let address_const_name = general_token_list.generate_token(&templates.render(user_strings::GR_ADDRESS_CONST_NAME, &context)?);
        let stride_count_const_name = general_token_list.generate_token(&templates.render(user_strings::GR_STRIDE_COUNT_CONST_NAME, &context)?);
        let stride_offset_const_name = general_token_list.generate_token(&templates.render(user_strings::GR_STRIDE_OFFSET_CONST_NAME, &context)?);
        let stride_array_type = general_token_list.generate_token(&templates.render(user_strings::GR_STRIDE_ARRAY_TYPE, &context)?);

        Ok(GenRegister { 
            name, 
            token_name, 
            address_const_name,
            address_hex,
            address_pretty,
            is_stride,
            summary,
            description,
            is_bitfield,
            doc_details,
            stride_count_const_name,
            stride_offset_const_name,
            stride_array_type,
            stride_count,
            stride_increment,
            stride_continuous,
            fields})
    }

}

impl GenField {
    /// take a Mdf field and convert it to a GenField
    pub fn from_field(register: &mdf::Register, field: &mdf::Field, page: PageType, _settings: &Settings, templates: &Tera, project_token_name : &String, interface_token_name : &String, interface_data_width: u32, register_token_name : &String, general_token_list : &mut TokenList, corfe2pif_token_list : &mut TokenList, pif2core_token_list : &mut TokenList) -> Result<Self, Box<dyn Error>> {


        let name = field.name.clone();
        let token_name = to_vhdl_token(&name);
        let description = utils::opt_vec_str_to_textarea(&field.description);

        let width = match field.position {
            mdf::FieldPosition::Single(_) => 1,
            mdf::FieldPosition::Field(msb, lsb) => if msb >= lsb { Ok(msb - lsb + 1 as u32)} else {Err(GenError::new(&page, "wrong bit order specified"))}?
        };
        let width_matches_interface = width == interface_data_width;
        let offset = match field.position {
            mdf::FieldPosition::Single(position) => position,
            mdf::FieldPosition::Field(_, lsb) => lsb
        };

        let position = if width == 1 {
            (width-1).to_string()
        } else {
            format!("{}..{}", width + offset - 1, offset)
        };

        // use templates for names and tokens
        let mut context = tera::Context::new();
        context.insert("project", project_token_name);
        context.insert("interface", &interface_token_name);
        context.insert("register", &register_token_name);
        context.insert("field", &token_name);
        context.insert("full_name", &name);
        context.insert("data_width", &interface_data_width);

        let width_const_name = general_token_list.generate_token(&templates.render(user_strings::GF_WIDTH_CONST_NAME, &context)?);
        let offset_const_name = general_token_list.generate_token(&templates.render(user_strings::GF_OFFSET_CONST_NAME, &context)?);

        let rw_mode = field.access;
        let is_read = rw_mode != mdf::AccessType::WO;
        let is_write = rw_mode != mdf::AccessType::RO;
            // rw_mode should be a string
        let rw_mode = rw_mode.to_string();

        let sig_type = field.signal.to_string();
        let sig_type_complete = match field.signal {
            utils::SignalType::Boolean | utils::SignalType::StdLogic => sig_type.clone(),
            _ => format!("{}({} downto 0)", &sig_type, width-1)
        };

        let sig_type_is_bit = field.signal == utils::SignalType::StdLogic;
        let sig_type_is_bool = field.signal == utils::SignalType::Boolean;
        let sig_type_is_vector = (!sig_type_is_bit) && (!sig_type_is_bool);

        let reset = match field.signal {
            // the way we format the value depends on the type
            utils::SignalType::Boolean => match field.reset.value {
                    0 => "false".to_owned(),
                    _ => "true".to_owned()
                },

            utils::SignalType::StdLogic => match field.reset.value {
                    0 => "'0'".to_owned(),
                    _ => "'1'".to_owned()
                },
                
            _ => format!("{}x\"{:x}\"",width,field.reset.value)
        };

        let location = field.location.unwrap_or(register.location.ok_or(GenError::new(&page, "location needs to be defined for field or register"))?);
        let is_in_core = location == mdf::LocationType::Core;

        let core_read_enable = field.core_signal_properties.use_read_enable.unwrap_or(false);
        let core_write_enable = field.core_signal_properties.use_write_enable.unwrap_or(false);

        let core2pif_has_data = is_in_core && is_read;
        let pif2core_has_data = is_write;

        // build the core2pif and pif2core elements
        let mut core2pif : Vec<GenStructSignal> = Default::default();
        let mut pif2core : Vec<GenStructSignal> = Default::default();
        
        if core2pif_has_data {
            core2pif.push(gen_registersignal(templates, "data", user_strings::GF_DATA_NAME, &sig_type_complete, user_strings::GF_DATA_DESCRIPTION, &context, corfe2pif_token_list)?);
        }
        if pif2core_has_data {
            pif2core.push(gen_registersignal(templates, "data", user_strings::GF_DATA_NAME, &sig_type_complete, user_strings::GF_DATA_DESCRIPTION, &context, pif2core_token_list)?);
        }
        if core_read_enable {
            pif2core.push(gen_registersignal(templates, "read_enable", user_strings::GF_READ_ENABLE_NAME, "boolean", user_strings::GF_READ_ENABLE_DESCRIPTION, &context, pif2core_token_list)?);
        }
        if core_write_enable {
            pif2core.push(gen_registersignal(templates, "write_enable", user_strings::GF_WRITE_ENABLE_NAME, "boolean", user_strings::GF_WRITE_ENABLE_DESCRIPTION, &context, pif2core_token_list)?);
        }

        let core2pif_names = gen_names_map(&core2pif);
        let pif2core_names = gen_names_map(&pif2core);

        Ok(GenField {
            name,
            description,
            width,
            width_matches_interface,
            offset,
            position,
            width_const_name,
            offset_const_name,
            rw_mode,
            is_read,
            is_write,
            sig_type,
            sig_type_complete,
            sig_type_is_bit,
            sig_type_is_bool,
            sig_type_is_vector,
            reset,
            is_in_core,
            core_read_enable,
            core_write_enable,
            core2pif_has_data,
            pif2core_has_data,
            core2pif,
            core2pif_names,
            pif2core,
            pif2core_names
        })
    }
}

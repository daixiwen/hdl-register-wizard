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
use tinytemplate;
use super::tokenlist::{TokenList, to_vhdl_token};
use crate::utils;
use crate::page::PageType;  
use crate::generate::generror::GenError;
use super::signal_list;

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
    #[serde(skip)]
    pub interfaces : Vec<GenInterface>,
}

#[derive(Serialize)]
struct GenModelContext<'a> {
    project: &'a str,
}

impl GenModel {
    /// take a Mdf model and convert it to a GenModel
    pub fn from_model(model: &mdf::Mdf, settings: &Settings) -> Result<Self, Box<dyn Error>> {
        let mut token_list = TokenList::new();

        let mut tt = tinytemplate::TinyTemplate::new();
        tt.set_default_formatter(&tinytemplate::format_unescaped);

        tt.add_template("top_name", "{project}*")?;
        tt.add_template("core_name", "{project}*_core")?;
        tt.add_template("core_instance", "i_{project}*_core_0")?;
        tt.add_template("pkg_name", "{project}*_pkg")?;

        let name = model.name.clone();

        let token_name = to_vhdl_token(&name);

        let single_interface = model.interfaces.len() == 1;

        let context = GenModelContext {
            project : &token_name
        };

        let top_name = token_list.generate_token(&tt.render("top_name", &context)?);
        let core_name = token_list.generate_token(&tt.render("core_name", &context)?);
        let core_instance = token_list.generate_token(&tt.render("core_instance", &context)?);
        let pkg_name = token_list.generate_token(&tt.render("pkg_name", &context)?);

        // apply a conversion to each interface
        let interfaces = model.interfaces.iter().enumerate().map(
            |(n, interface)| GenInterface::from_interface(
                interface, PageType::Interface(n), settings, &token_name, 
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
    /// list of registers
    #[serde(skip)]
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

#[derive(Serialize)]
struct GenInterfaceContext<'a> {
    project: &'a str,
    interface: &'a str
}

impl GenInterface {
    /// take a Mdf interface and convert it to a GenInterface
    pub fn from_interface(interface: &mdf::Interface, page: PageType, _settings: &Settings, project_token_name : &String, single_interface : &bool, general_token_list : &mut TokenList) -> Result<Self, Box<dyn Error>> {

        let mut tt = tinytemplate::TinyTemplate::new();
        tt.set_default_formatter(&tinytemplate::format_unescaped);

        tt.add_template("pif_name", "{project}_{interface}*_pif")?;
        tt.add_template("pif_instance", "i_{project}_{interface}*_pif_0")?;
        tt.add_template("core2pif_name", "{interface}*_core2pif")?;
        tt.add_template("pif2core_name", "{interface}*_pif2core")?;
        tt.add_template("register_enum_name", "t_{interface}*_regs")?;
        tt.add_template("address_decoder_name", "f_{interface}*_address_decode")?;
        tt.add_template("address_stride_func_name", "f_{interface}*_address_stride")?;
        tt.add_template("address_width_const_name", "c_{interface}*_address_width")?;
        tt.add_template("data_width_const_name", "c_{interface}*_data_width")?;


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
            
        let context = GenInterfaceContext {
            project: project_token_name,
            interface: &token_name
        };

        let pif_name = general_token_list.generate_token(&tt.render("pif_interface", &context)?);
        let pif_instance = general_token_list.generate_token(&tt.render("pif_instance", &context)?); 
        let core2pif_name = general_token_list.generate_token(&tt.render("core2pif_name", &context)?);
        let pif2core_name = general_token_list.generate_token(&tt.render("pif2core_name", &context)?); 
        let register_enum_name = general_token_list.generate_token(&tt.render("register_enum_name", &context)?); 
        let address_decoder_name = general_token_list.generate_token(&tt.render("address_decoder_name", &context)?);
        let address_stride_func_name = general_token_list.generate_token(&tt.render("address_stride_func_name", &context)?); 
        let address_width_const_name = general_token_list.generate_token(&tt.render("address_width_const_name", &context)?); 
        let data_width_const_name = general_token_list.generate_token(&tt.render("data_width_const_name", &context)?); 


        let port_context = signal_list::PortTemplateContext {
            project : &project_token_name,
            interface : &token_name,
            signal: "",
            address_width,
            address_width_m1 : address_width - 1,
            data_width,
            data_width_m1 : data_width - 1
        };

        let ports = signal_list::to_port_list(interface_type, port_context, general_token_list)?;

        // make a second ports list, a hashmap from function to name
        let ports_names : HashMap<String, String> = ports.iter().map(
            | signal | (signal.function.clone(), signal.name.clone())).collect();

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
            registers : Default::default()})

    }
}

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
    /// if true, register is an array
    pub is_stride : bool,
    /// quick description
    pub summary : String,
    /// longer description
    pub description : String,
    /// if true, is a bitfield
    pub is_bitfield : bool,
    /// register width (only valid if not a bitfield)
    pub width : u32,
    /// read-write mode (only valid if not a bitfield)
    pub rw_mode : String,
    /// read access (only valid if not a bitfield)
    pub is_read : bool,
    /// write access (only valid if not a bitfield)
    pub is_write : bool,
    /// register type (only valid if not a bitfield)
    pub reg_type : String,
    /// true if type is bit (only valid if not a bitfield)
    pub reg_type_is_bit : bool,
    /// true if type is bool (only valid if not a bitfield)
    pub reg_type_is_bool : bool,
    /// register reset value, including quotes if required (only valid if not a bitfield)
    pub reset : String,
    /// register location (only valid if not a bitfield)
    pub is_in_core : bool,
    /// read enable (only valid if not a bitfield)
    pub core_read_enable : bool,
    /// write enable (only valid if not a bitfield)
    pub core_write_enable : bool,
    /// name used for the constant with the array length (only valid if is_stride = true)    
    pub stride_count_const_name : String,
    /// name used for the constant with the address offset between array elements (only valid if is_stride = true)    
    pub stride_offset_const_name : String,
    /// name used for the array type (only valid if is_stride = true)
    pub stride_array_type : String,
    /// array length (only valid if is_stride = true)
    pub stride_count : u32,
    /// address offset between array elements (only valid if is_stride = true)
    pub stride_offset : u32,
    /// if true, array addresses are continuous (only valid if is_stride = true)
    pub stride_continuous : bool,
    /// signals from this register in the core2pif record
    pub core2pif: Vec<GenStructSignal>,
    // list of signals for core2pif as a map (with function as index and name as value)
    pub core2pif_names: HashMap<String, String>,
    /// signals from this register in the pif2core record
    pub pif2core: Vec<GenStructSignal>,
    // list of signals for core2pif as a map (with function as index and name as value)
    pub pif2core_names: HashMap<String, String>,
}

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

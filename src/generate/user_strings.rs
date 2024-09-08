//! handling of user-specified strings
use std::collections::BTreeMap;
use core::slice::Iter;

pub struct UserStringSpec {
    pub template_name : &'static str,
    pub label : &'static str,
    pub default_value : &'static str,
    pub description : &'static str,
}

pub const GM_TOP_NAME : &str = "gm_top_name";
pub const GM_CORE_NAME : &str = "gm_core_name";
pub const GM_CORE_INSTANCE : &str = "gm_core_instance";
pub const GM_PKG_NAME : &str = "gm_pkg_name";
pub const GI_PIF_NAME : &str = "gi_pif_name";
pub const GI_PIF_INSTANCE : &str = "gi_pif_instance";
pub const GI_CORE2PIF_NAME : &str = "gi_core2pif_name";
pub const GI_PIF2CORE_NAME : &str = "gi_pif2core_name";
pub const GI_REGISTER_ENUM_NAME  : &str = "gi_register_enum_name";
pub const GI_ADDRESS_DECODER_NAME : &str = "gi_address_decoder_name";
pub const GI_ADDRESS_STRIDE_FUNC_NAME  : &str = "gi_address_stride_func_name";
pub const GI_ADDRESS_WIDTH_CONST_NAME  : &str = "gi_address_width_const_name";
pub const GI_DATA_WIDTH_CONST_NAME : &str = "gi_data_width_const_name";
pub const GR_ADDRESS_CONST_NAME  : &str = "gr_address_const_name";
pub const GR_STRIDE_COUNT_CONST_NAME : &str = "gr_stride_count_const_name";
pub const GR_STRIDE_OFFSET_CONST_NAME : &str = "gr_stride_offset_const_name";
pub const GR_STRIDE_ARRAY_TYPE : &str = "gr_stride_array_type";
pub const GR_WIDTH_CONST_NAME : &str = "gr_width_const_name";
pub const GR_DATA_NAME : &str = "gr_data_name";
pub const GR_DATA_DESCRIPTION : &str = "gr_data_description";
pub const GR_READ_ENABLE_NAME : &str = "gr_read_enable_name";
pub const GR_READ_ENABLE_DESCRIPTION : &str = "gr_read_enable_description";
pub const GR_WRITE_ENABLE_NAME : &str = "gr_write_enable_name";
pub const GR_WRITE_ENABLE_DESCRIPTION : &str = "gr_write_enable_description";
pub const GF_WIDTH_CONST_NAME : &str = "gf_width_const_name";
pub const GF_OFFSET_CONST_NAME : &str = "gf_offset_const_name";
pub const GF_DATA_NAME : &str = "gf_data_name";
pub const GF_DATA_DESCRIPTION : &str = "gf_data_description";
pub const GF_READ_ENABLE_NAME : &str = "gf_read_enable_name";
pub const GF_READ_ENABLE_DESCRIPTION : &str = "gf_read_enable_description";
pub const GF_WRITE_ENABLE_NAME : &str = "gf_write_enable_name";
pub const GF_WRITE_ENABLE_DESCRIPTION  : &str = "gf_write_enable_description";

pub const USER_NAMES_SPECS : [UserStringSpec; 26] = [
    UserStringSpec { template_name: GM_TOP_NAME, label: "Top", default_value: "{{ project }}*", description: "Name of the top entity, instanciating the core and the PIFs" },
    UserStringSpec { template_name: GM_CORE_NAME, label: "Core", default_value: "{{ project }}*_core", description: "Name of the core entity, containing user code" },
    UserStringSpec { template_name: GM_CORE_INSTANCE, label: "Core instance", default_value: "i_{{ project }}*_core_0", description: "Name of the core instance in the top entity" },
    UserStringSpec { template_name: GM_PKG_NAME, label: "Package", default_value: "{{ project }}*_pkg", description: "Name of the package containing all the definitions" },
    UserStringSpec { template_name: GI_PIF_NAME, label: "Pif", default_value: "{{ project }}_{{ interface }}*_pif", description: "Name of the pif (processor interface) entity, containing the interface generated code" },
    UserStringSpec { template_name: GI_PIF_INSTANCE, label: "Pif instance", default_value: "i_{{ project }}_{{ interface }}*_pif_0", description: "Name of the pif instance in the top entity" },
    UserStringSpec { template_name: GI_CORE2PIF_NAME, label: "Core to pif", default_value: "{{ interface }}*_core2pif", description: "Name of the record containing the signals from the core to the pif" },
    UserStringSpec { template_name: GI_PIF2CORE_NAME, label: "Pif to core", default_value: "{{ interface }}*_pif2core", description: "Name of the record containing the signals from the pif to the core" },
    UserStringSpec { template_name: GI_REGISTER_ENUM_NAME, label: "Register enum", default_value: "t_{{ interface }}*_regs", description: "Name of the type enumerating all the registers" },
    UserStringSpec { template_name: GI_ADDRESS_DECODER_NAME, label: "Address decoder", default_value: "f_{{ interface }}*_address_decode", description: "Name of the function decoding the address in the pif" },
    UserStringSpec { template_name: GI_ADDRESS_STRIDE_FUNC_NAME, label: "Address stride", default_value: "f_{{ interface }}*_address_stride", description: "Name of the function decoding the stride number for a register" },
    UserStringSpec { template_name: GI_ADDRESS_WIDTH_CONST_NAME, label: "Address width", default_value: "c_{{ interface }}*_address_width", description: "Name of the constant containing the size of the address bus" },
    UserStringSpec { template_name: GI_DATA_WIDTH_CONST_NAME, label: "Data width", default_value: "c_{{ interface }}*_data_width", description: "Name of the constant containing the size of the data bus" },
    UserStringSpec { template_name: GR_ADDRESS_CONST_NAME, label: "Register address", default_value: "c_{{ project }}_{{ interface }}_{{ register }}*_addr", description: "Name of the constant containing the register address" },
    UserStringSpec { template_name: GR_STRIDE_COUNT_CONST_NAME, label: "Stride count", default_value: "c_{{ project }}_{{ interface }}_{{ register }}*_count", description: "Name of the constant containing the register stride number" },
    UserStringSpec { template_name: GR_STRIDE_OFFSET_CONST_NAME, label: "Stride offset", default_value: "c_{{ project }}_{{ interface }}_{{ register }}*_offset", description: "Name of the constant containing the register stride offset" },
    UserStringSpec { template_name: GR_STRIDE_ARRAY_TYPE, label: "Stride array type", default_value: "{{ project }}_{{ interface }}_{{ register }}*_array_t", description: "Name of the type for the stride array" },
    UserStringSpec { template_name: GR_WIDTH_CONST_NAME, label: "Register width", default_value: "c_{{ project }}_{{ interface }}_{{ register }}*_width", description: "Name of the constant contining the register width" },
    UserStringSpec { template_name: GR_DATA_NAME, label: "Register name", default_value: "{{ register }}*", description: "Name of the register in the core2pif and pif2core records" },
    UserStringSpec { template_name: GR_READ_ENABLE_NAME, label: "Register read enable", default_value: "{{ register }}_re*", description: "Name of the register read enable signal in the pif2core record" },
    UserStringSpec { template_name: GR_WRITE_ENABLE_NAME, label: "Register write enable", default_value: "{{ register }}_we*", description: "Name of the register write enable signal in the pif2core record" },
    UserStringSpec { template_name: GF_WIDTH_CONST_NAME, label: "Field width", default_value: "c_{{ project }}_{{ interface }}_{{ register }}_{{ field }}*_width", description: "Name of the constant containing the field width" },
    UserStringSpec { template_name: GF_OFFSET_CONST_NAME, label: "Field offset", default_value: "c_{{ project }}_{{ interface }}_{{ register }}_{{ field }}*_offset", description: "Name of the constant containing the field offset" },
    UserStringSpec { template_name: GF_DATA_NAME, label: "Field name", default_value: "{{ field }}*", description: "Name of the field in the register record" },
    UserStringSpec { template_name: GF_READ_ENABLE_NAME, label: "Field read enable", default_value: "{{ field }}_re*", description: "Name of the field read enable signal" },
    UserStringSpec { template_name: GF_WRITE_ENABLE_NAME, label: "Field write enable", default_value: "{{ field }}_we*", description: "Name of the field write enable signal" },
];

pub const USER_COMMENTS_SPECS : [UserStringSpec; 6] = [
    UserStringSpec { template_name: GR_DATA_DESCRIPTION, label: "Register description", default_value: "data for {{ full_name }}", description: "Description for register" },
    UserStringSpec { template_name: GR_READ_ENABLE_DESCRIPTION, label: "Register read enable", default_value: "signals that {{ full_name }} is being read", description: "Description for the register read enable signal" },
    UserStringSpec { template_name: GR_WRITE_ENABLE_DESCRIPTION, label: "Register write enable", default_value: "signals that {{ full_name }} is being written", description: "Description for the write enable signal" },
    UserStringSpec { template_name: GF_DATA_DESCRIPTION, label: "Field description", default_value: "data for {{ full_name }}", description: "Description for field" },
    UserStringSpec { template_name: GF_READ_ENABLE_DESCRIPTION, label: "Field read enable", default_value: "signals that {{ full_name }} is being read", description: "Description for the field read enable signal" },
    UserStringSpec { template_name: GF_WRITE_ENABLE_DESCRIPTION, label: "Field write enable", default_value: "signals that {{ full_name }} is being written", description: "Description for the field write enable signal" },
];

fn load_defaults_from_iter(templates_list: &mut BTreeMap<String,String>, iter: Iter<'_, UserStringSpec>) {
    for spec in iter {
        if !templates_list.contains_key(spec.template_name) {
            templates_list.insert(spec.template_name.to_owned(), spec.default_value.to_owned());
        }
    }
}

// Make sure that non existing templates in the settings are loaded with default values
// If a new version of the application comes out with new user templates, loading old settings is still possible and the new
// templates will just be added with default values 
pub fn load_defaults(templates_list: &mut BTreeMap<String,String>) {
    load_defaults_from_iter(templates_list, USER_NAMES_SPECS.iter());
    load_defaults_from_iter(templates_list, USER_COMMENTS_SPECS.iter());
}

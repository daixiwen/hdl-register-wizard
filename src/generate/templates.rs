use tera::{Tera,Result};
use std::collections::HashMap;

fn escape_markdown(value : &tera::Value, _args : &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let in_string : String = tera::from_value(value.clone())?;
    let underscore_replaced = str::replace(&in_string, "_", r"\_");
    let star_replaced = str::replace(&underscore_replaced, "*", r"\*");

    Ok(tera::to_value(star_replaced)?)
}

pub fn load_template(tera: &mut Tera, name : &str) -> Result<()> {
    let rel_fname = format!("templates/{name}");
    if let Some(template_path) = crate::assets::find_asset(&rel_fname) {
        tera.add_template_file(template_path, Some(name))?;
        Ok(())
    } else {
        Err(tera::Error::msg(format!("template file not found: {name}")))
    }
}

pub fn gen_templates() -> Result<Tera> {
    let mut tera = Tera::default();

    tera.autoescape_on(vec![]);
    tera.register_filter("escape_markdown", escape_markdown);

    // documentation template
    //tera.add_raw_template("documentation.md", include_str!("../templates/documentation.md"))?;
    load_template(&mut tera, "documentation.md")?;

    // genmodel templates. The templates used to generate tokens have the special * character which is used by the tokenlist object to
    // know where it can insert a number
    tera.add_raw_templates(vec![
        ("gm_top_name", "{{ project }}*"),
        ("gm_core_name", "{{ project }}*_core"),
        ("gm_core_instance", "i_{{ project }}*_core_0"),
        ("gm_pkg_name", "{{ project }}*_pkg"),
        ("gi_pif_name", "{{ project }}_{{ interface }}*_pif"),
        ("gi_pif_instance", "i_{{ project }}_{{ interface }}*_pif_0"),
        ("gi_core2pif_name", "{{ interface }}*_core2pif"),
        ("gi_pif2core_name", "{{ interface }}*_pif2core"),
        ("gi_register_enum_name", "t_{{ interface }}*_regs"),
        ("gi_address_decoder_name", "f_{{ interface }}*_address_decode"),
        ("gi_address_stride_func_name", "f_{{ interface }}*_address_stride"),
        ("gi_address_width_const_name", "c_{{ interface }}*_address_width"),
        ("gi_data_width_const_name", "c_{{ interface }}*_data_width"),
        ("gr_address_const_name", "c_{{ project }}_{{ interface }}_{{ register }}*_addr"),
        ("gr_stride_count_const_name", "c_{{ project }}_{{ interface }}_{{ register }}*_count"),
        ("gr_stride_offset_const_name", "c_{{ project }}_{{ interface }}_{{ register }}*_offset"),
        ("gr_stride_array_type", "{{ project }}_{{ interface }}_{{ register }}*_array_t"),
        ("gr_width_const_name", "c_{{ project }}_{{ interface }}_{{ register }}*_width"),
        ("gr_data_name", "{{ register }}*"),
        ("gr_data_description", "data for {{ full_name }}"),
        ("gr_read_enable_name", "{{ register }}_re*"),
        ("gr_read_enable_description", "signals that {{ full_name }} is being read"),
        ("gr_write_enable_name", "{{ register }}_we*"),
        ("gr_write_enable_description", "signals that {{ full_name }} is being written"),
        ("gf_width_const_name", "c_{project}_{interface}_{register}_{field}*_width"),
        ("gf_offset_const_name", "c_{project}_{interface}_{register}_{field}*_offset"),
        ("gf_data_name", "{register}*"),
        ("gf_data_description", "data for {full_name}"),
        ("gf_read_enable_name", "{register}_re*"),
        ("gf_read_enable_description", "signals that {full_name} is being read"),
        ("gf_write_enable_name", "{register}_we*"),
        ("gf_write_enable_description", "signals that {full_name} is being written")
        ])?;
    
    Ok(tera)
}

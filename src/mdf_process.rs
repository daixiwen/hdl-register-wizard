//! functions acting on mdf data

use super::mdf_format::Mdf;
use super::mdf_format::LocationType;
use super::mdf_format::Interface;
use super::mdf_format::Register;
use super::mdf_format::Field;

impl Mdf {
    /// goes through the file and removes all extra options that are not
    /// allowed.
    pub fn clean(&mut self) {
        for interface in &mut self.interfaces {
            interface.clean();
        }
    }
}

impl Interface {
    /// goes through the interface and removes all extra options that are not
    /// allowed.
    pub fn clean(&mut self) {
        for register in &mut self.registers {
            register.clean();
        }
    }
}

impl Register {
    /// goes through the register and removes all extra options that are not
    /// allowed.
    pub fn clean(&mut self) {
        // remove all core properties if register location is not in core
        let register_location = self.location;
        match register_location {
            Some(LocationType::Pif) => {
                self.core_signal_properties.use_read_enable = None;
                self.core_signal_properties.use_write_enable = None;                        
            },
            _ => ()

        }

        // remove register wide properties if fields are defined
        if ! self.fields.is_empty() {
            self.width = None;
            self.access = None;
            self.signal = None;
            self.reset = None;
            self.core_signal_properties.use_read_enable = None;
            self.core_signal_properties.use_write_enable = None;                        
        }

        for field in &mut self.fields {
            field.clean(register_location);
        }
    }
}

impl Field {
    /// goes through the register and removes all extra options that are not
    /// allowed.
    pub fn clean(&mut self, register_location : Option<LocationType>) {
        // remove all core properties if register location is not in core
        match self.location {
            Some(LocationType::Pif) => {
                self.core_signal_properties.use_read_enable = None;
                self.core_signal_properties.use_write_enable = None;                        
            },
            None => { match register_location {
                Some(LocationType::Pif) => {
                    self.core_signal_properties.use_read_enable = None;
                    self.core_signal_properties.use_write_enable = None;                        
                },

                _ => ()
            
                }
            },
            _ => ()

        }
    }
}

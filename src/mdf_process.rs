//! functions acting on mdf data

use super::file_formats::mdf;
use mdf::Mdf;
use mdf::LocationType;
use mdf::Interface;
use mdf::Register;
use mdf::Field;

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

    /// returns the interface data width. None if the width can't be determined
    pub fn get_data_width(&self) -> Option<u32>{
        match self.data_width {
            Some(width) => Some(width),
            None => {
                // goes through all registers to find the biggest size
                self.registers.iter().fold(None, | width, reg | {
                    // with two Somes, find the maximum. With one None and one Some, return the Some
                    match reg.get_data_width() {
                        None => width,
                        Some(reg_width) => match width {
                            None => Some(reg_width),
                            Some(previous_width) => Some(u32::max(previous_width, reg_width))
                        }
                    }
                })
            }
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

    /// returns the register data size. None means it will use the size of the interface
    pub fn get_data_width(&self) -> Option<u32>{
        match self.signal {
            None => {
                // this is a bitfield. Find the msb within all the fields
                let msb = self.fields.iter().fold(0, | width, field | {
                    u32::max(width, match field.position {
                        mdf::FieldPosition::Single(bitpos) => bitpos,
                        mdf::FieldPosition::Field(msb, _) => msb
                    })
                });
                Some(msb+1)
            }
            Some(_) => self.width
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

//! functions acting on mdf data

use super::file_formats::mdf;
use crate::utils::VectorValue;
use mdf::Field;
use mdf::Interface;
use mdf::LocationType;
use mdf::Mdf;
use mdf::Register;

impl Mdf {
    /// goes through the file and removes all extra options that are not
    /// allowed.
    pub fn clean(&mut self) {
        for interface in &mut self.interfaces {
            interface.clean();
        }
    }
}

fn add_address(
    addresses: &mut std::collections::HashSet<u128>,
    register: &mdf::Register,
    interface_width_bytes: u32,
) -> Result<(), String> {
    if let Some(address) = register.address.value {
        match &register.address.stride {
            None => {
                if addresses.contains(&address.value) {
                    return Err(format!(
                        "Register {}'s address already in use",
                        register.name
                    ));
                }
                addresses.insert(address.value);
            }
            Some(stride) => {
                let increment = match stride.increment {
                    Some(stride_increment) => stride_increment.value,
                    None => interface_width_bytes as u128,
                };
                // go for a complete run first to see if all addresses are available
                for i in 0..stride.count.value {
                    let current_address = address.value + i * increment;
                    if addresses.contains(&current_address) {
                        return Err(format!(
                            "Register {}'s address already in use",
                            register.name
                        ));
                    }
                }
                // now add the addresses
                for i in 0..stride.count.value {
                    let current_address = address.value + i * increment;
                    addresses.insert(current_address);
                }
            }
        }
    }

    Ok(())
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
    pub fn get_data_width(&self) -> Option<u32> {
        match self.data_width {
            Some(width) => Some(width),
            None => {
                // goes through all registers to find the biggest size
                self.registers.iter().fold(None, |width, reg| {
                    // with two Somes, find the maximum. With one None and one Some, return the Some
                    match reg.get_data_width() {
                        None => width,
                        Some(reg_width) => match width {
                            None => Some(reg_width),
                            Some(previous_width) => Some(u32::max(previous_width, reg_width)),
                        },
                    }
                })
            }
        }
    }

    /// automatically assign addresses to the registers
    /// this is not a very good algorithm. it is rather brute force, but it is simple and won't be called that often any way
    /// it should still be pretty fast in standard settings
    pub fn assign_addresses(&mut self) -> Result<(), String> {
        let mut addresses: std::collections::HashSet<u128> = Default::default();
        if let Some(width_bits) = self.get_data_width() {
            // convert width to bytes
            let width_bytes = (width_bits + 7) / 8;

            // first make a list of all used addresses, to be sure there aren't any duplicates
            for register in &self.registers {
                add_address(&mut addresses, register, width_bytes)?;
            }

            // now loop within all registers without addresses and assign one to them
            let mut current_address: u128 = 0;
            for register in self.registers.iter_mut() {
                match register.address.value {
                    Some(addr) => {
                        // update current address to after this register's address
                        current_address = addr.value + width_bytes as u128;
                    }
                    None => {
                        // try to add the register at the current address
                        loop {
                            register.address.value = Some(VectorValue::from(current_address));
                            if add_address(&mut addresses, register, width_bytes).is_ok() {
                                break;
                            } else {
                                // try the next one
                                current_address += width_bytes as u128;
                            }
                        }
                    }
                }
            }

            Ok(())
        } else {
            Err(format!(
                "Unable to determine the width of interface {}",
                self.name
            ))
        }
    }

    /// remove all assigned addresses to the registers
    pub fn deassign_addresses(&mut self) -> Result<(), String> {
        for register in self.registers.iter_mut() {
            register.address.value = None;
        }

        Ok(())
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
            }
            _ => (),
        }

        // remove register wide properties if fields are defined
        if !self.fields.is_empty() {
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
    pub fn get_data_width(&self) -> Option<u32> {
        match self.signal {
            None => {
                // this is a bitfield. Find the msb within all the fields
                let msb = self.fields.iter().fold(0, |width, field| {
                    u32::max(
                        width,
                        match field.position {
                            mdf::FieldPosition::Single(bitpos) => bitpos,
                            mdf::FieldPosition::Field(msb, _) => msb,
                        },
                    )
                });
                Some(msb + 1)
            }
            Some(_) => self.width,
        }
    }

    /// make sure all the fields are assigned to different bits
    pub fn assign_fields(&mut self) -> Result<(), String> {
        let mut current_msb = 0;
        for field in self.fields.iter_mut() {
            match field.position {
                mdf::FieldPosition::Single(bitnum) => {
                    if bitnum < current_msb {
                        field.position = mdf::FieldPosition::Single(current_msb);
                        current_msb += 1;
                    } else {
                        current_msb = bitnum + 1;
                    }
                }
                mdf::FieldPosition::Field(msb, lsb) => {
                    if lsb > msb {
                        return Err(format!("Field '{}' has lsb bigger than msb", field.name));
                    }
                    if lsb < current_msb {
                        field.position =
                            mdf::FieldPosition::Field(msb + current_msb - lsb, current_msb);
                        current_msb += msb - lsb + 1;
                    } else {
                        current_msb = msb + 1;
                    }
                }
            }
        }

        Ok(())
    }

    /// realign all the fields to lsb 0
    pub fn deassign_fields(&mut self) -> Result<(), String> {
        for field in self.fields.iter_mut() {
            match field.position {
                mdf::FieldPosition::Single(_) => {
                    field.position = mdf::FieldPosition::Single(0);
                }
                mdf::FieldPosition::Field(msb, lsb) => {
                    if lsb > msb {
                        return Err(format!("Field '{}' has lsb bigger than msb", field.name));
                    } else {
                        field.position = mdf::FieldPosition::Field(msb - lsb, 0);
                    }
                }
            }
        }

        Ok(())
    }
}

impl Field {
    /// goes through the register and removes all extra options that are not
    /// allowed.
    pub fn clean(&mut self, register_location: Option<LocationType>) {
        // remove all core properties if register location is not in core
        match self.location {
            Some(LocationType::Pif) => {
                self.core_signal_properties.use_read_enable = None;
                self.core_signal_properties.use_write_enable = None;
            }
            None => match register_location {
                Some(LocationType::Pif) => {
                    self.core_signal_properties.use_read_enable = None;
                    self.core_signal_properties.use_write_enable = None;
                }

                _ => (),
            },
            _ => (),
        }
    }
}

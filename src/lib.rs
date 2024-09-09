use components::{Direction, StreetType, Unit};
use regex::{REG_DIRECTIONAL, REG_DIRECTIONAL_ABBVR, REG_STREET0, REG_STREET1, REG_STREET2, REG_STREET3, REG_STREET_ABBVR0, REG_STREET_ABBVR1, REG_STREET_ABBVR2, REG_STREET_ABBVR3, REG_STREET_NUMBER, REG_UNIT, REG_UNIT_ABBVR, REG_UNIT_CHAR, REG_UNIT_NO};

pub mod components;
pub mod regex;

#[derive(Debug)]
pub struct Address {
    pub street_no: String,
    pub direction: Direction,
    pub street_name: String,
    pub street_type: StreetType,
    pub unit_no: String,
    pub unit_type: Unit,
}

pub trait AddressParsing {
    fn parse_addr(&self) -> Address;
}

impl AddressParsing for String {
    fn parse_addr(&self) -> Address {
        let address: Address = Address { 
            street_no: "".to_string(), 
            direction: Direction::Abbreviated("".to_string()), 
            street_name: "".to_string(), 
            street_type: StreetType::Abbreviated("".to_string()), 
            unit_no: "".to_string(), 
            unit_type: Unit::Abbreviated("".to_string())
        };

        let full_address = self.trim().to_ascii_uppercase();

        address
            .street_no(&full_address)
            .directional(&full_address)
            .street_type(&full_address)
            .unit_type_and_no(&full_address)
            .street_name(&full_address)
    }
}

impl Address {
    pub fn street_no(mut self, full_address: &str) -> Self {
        self.street_no = if let Some(captures) = REG_STREET_NUMBER.captures(&full_address) {
            let needle = captures.get(0).map_or("", |m|m.as_str());
            needle.trim().replace("\"", "")
        } else { "".to_owned() };

        self
    }

    pub fn street_type(mut self, full_address: &str) -> Self {
        self.street_type = if let Some(needle) = REG_STREET_ABBVR0.captures(&full_address) {
            let val = needle.get(0).map_or("", |m|m.as_str());
            StreetType::Abbreviated(val.trim().replace("\"", ""))
        } else { 
            let captured = if let Some(needle) = REG_STREET_ABBVR1.captures(&full_address) {
                let val = needle.get(0).map_or("", |m|m.as_str());
                StreetType::Abbreviated(val.trim().replace("\"", ""))
            } else {
                let captured = if let Some(needle) = REG_STREET_ABBVR2.captures(&full_address) {
                    let val = needle.get(0).map_or("", |m|m.as_str());
                    StreetType::Abbreviated(val.trim().replace("\"", ""))
                } else {
                    let captured = if let Some(needle) = REG_STREET_ABBVR3.captures(&full_address) {
                        let val = needle.get(0).map_or("", |m|m.as_str());
                        StreetType::Abbreviated(val.trim().replace("\"", ""))
                    } else { StreetType::Abbreviated("".to_owned()) };
                    captured
                };
                captured
            };
            captured
        };

        if self.street_type == StreetType::Abbreviated("".to_owned()) {
            self.street_type = if let Some(needle) = REG_STREET0.captures(&full_address) {
                let val = needle.get(0).map_or("", |m|m.as_str());
                StreetType::Full(val.trim().replace("\"", ""))
            } else { 
                let captured = if let Some(needle) = REG_STREET1.captures(&full_address) {
                    let val = needle.get(0).map_or("", |m|m.as_str());
                    StreetType::Full(val.trim().replace("\"", ""))
                } else {
                    let captured = if let Some(needle) = REG_STREET2.captures(&full_address) {
                        let val = needle.get(0).map_or("", |m|m.as_str());
                        StreetType::Full(val.trim().replace("\"", ""))
                    } else {
                        let captured = if let Some(needle) = REG_STREET3.captures(&full_address) {
                            let val = needle.get(0).map_or("", |m|m.as_str());
                            StreetType::Full(val.trim().replace("\"", ""))
                        } else { StreetType::Abbreviated("".to_owned()) };
                        captured
                    };
                    captured
                };
                captured
            };
        }
        
        self
    }

    pub fn unit_type_and_no(mut self, full_address: &str) -> Self {
        self.unit_type = if let Some(needle) = REG_UNIT_ABBVR.captures(&full_address) {
            let val = &needle.get(0).map_or("", |m|m.as_str());

            self.unit_no = if let Some(needle) = REG_UNIT_NO.captures(&full_address) {
                let val = &needle.get(0).map_or("", |m|m.as_str());
                val.trim().replace("\"", "")
            } else {
                let captured = if let Some(needle) = REG_UNIT_CHAR.captures(&full_address) {
                    let val = &needle.get(0).map_or("", |m|m.as_str());
                    val.trim().replace("\"", "")
                } else { "".to_owned() };
                captured
            };

            Unit::Abbreviated(val.trim().replace("\"", ""))
        } else {
            let captured = if let Some(needle) = REG_UNIT.captures(&full_address) {
                let val = needle.get(0).map_or("", |m|m.as_str());

                self.unit_no = if let Some(needle) = REG_UNIT_NO.captures(&full_address) {
                    let val = &needle.get(0).map_or("", |m|m.as_str());
                    val.trim().replace("\"", "")
                } else {
                    let captured = if let Some(needle) = REG_UNIT_CHAR.captures(&full_address) {
                        let val = &needle.get(0).map_or("", |m|m.as_str());
                        val.trim().replace("\"", "")
                    } else { "".to_owned() };
                    captured
                };

                Unit::Full(val.trim().replace("\"", ""))
            } else { Unit::Abbreviated("".to_owned()) };
            captured
        };

        self
    }

    pub fn directional(mut self, full_address: &str) -> Self {
        self.direction = if let Some(needle) = REG_DIRECTIONAL_ABBVR.captures(&full_address) {
            let val = needle.get(0).map_or("", |m|m.as_str());
            Direction::Abbreviated(val.trim().replace("\"", ""))
        } else { 
            let captured = if let Some(needle) = REG_DIRECTIONAL.captures(&full_address) {
                let val = needle.get(0).map_or("", |m|m.as_str());
                Direction::Full(val.trim().replace("\"", ""))
            } else { Direction::Abbreviated("".to_owned()) };
            captured
        };

        self
    }

    pub fn street_name(mut self, full_address: &str) -> Self {
        let street_no = &self.street_no;
        let direction = &self.direction.to_string();
        let street_type = &self.street_type.to_string();
        let unit_type = &self.unit_type.to_string();
        let unit_no = &self.unit_no;

        let street_name = format!(" {} ", full_address)
            .replace(&format!(" {} ", street_type), " ")
            .replace(street_no, "")
            .replace(&format!(" {} ", direction), " ")
            .replace(&format!(" {} ", unit_type), " ")
            .replace(&format!(" {} ", unit_no), "");

        self.street_name = street_name.trim().replace("\"", "");
        self
    }
}
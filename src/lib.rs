use components::{Direction, StreetType, Unit};

pub mod components;
pub mod regex;
pub mod parse;

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


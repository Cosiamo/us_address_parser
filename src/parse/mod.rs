use direction::parse_direction;
use street::{parse_street_name, parse_street_no};
use street_type::parse_street_type;
use unit::parse_unit_type_and_no;

use crate::Address;

pub(crate) mod street_type;
pub(crate) mod unit;
pub(crate) mod direction;
pub(crate) mod street;

impl Address {
    /// Gets the street number for an address
    pub fn street_no(self, full_address: &str) -> Self {
        parse_street_no(self, full_address)
    }

    /// Matches an input address to a list of 748 U.S. street types (half of them are abbreviated, other half are full types) to parse out the street type
    pub fn street_type(self, full_address: &str) -> Self {
        parse_street_type(self, full_address)
    }

    /// Matches an input address to a list of 48 unit types (half of them are abbreviated, other half are full types) to parse out the unit type and unit number
    pub fn unit_type_and_no(self, full_address: &str) -> Self {
        parse_unit_type_and_no(self, full_address)
    }

    /// Finds the pre or post direction in an input address
    pub fn directional(self, full_address: &str) -> Self {
        parse_direction(self, full_address)
    }

    /// Uses the [`Address::street_no()`], [`Address::street_type()`], [`Address::unit_type_and_no()`], and [`Address::directional()`] 
    /// to strip out those parts of an input address to get the street name.
    /// !!!USE LAST!!! when adding methods to [`Address`].
    /// ```rust
    /// address
    ///    .street_no(&full_address)
    ///    .directional(&full_address)
    ///    .street_type(&full_address)
    ///    .unit_type_and_no(&full_address)
    ///    .street_name(&full_address)
    /// ```
    pub fn street_name(self, full_address: &str) -> Self {
        parse_street_name(self, full_address)
    }
}
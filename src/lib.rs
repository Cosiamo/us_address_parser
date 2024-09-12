pub(crate) mod components;
pub(crate) mod regex;
pub mod parse;

#[derive(Debug, Clone)]
/// `Address` is the return type for [`AddressParsing::parse_addr`].
/// Breaks-down an address into it's basic parts.
pub struct Address {
    pub street_no: Option<String>,
    pub direction: Option<String>,
    pub street_name: Option<String>,
    pub street_type: Option<String>,
    pub unit_no: Option<String>,
    pub unit_type: Option<String>,
}

/// Used to add the [`AddressParsing::parse_addr`] method to your types.
/// Example:
/// ```rust
///struct CustomerInfo {
///    id: usize,
///    customer_name: String,
///    customer_address: String
///}
///
///impl AddressParsing for CustomerInfo {
///    fn parse_addr(&self) -> Address {
///        let address = self.customer_address;
///        us_address_parser::string_to_address(address)
///    }
///}
///```
pub trait AddressParsing {
    fn parse_addr(&self) -> Address;
}

impl AddressParsing for String {
    fn parse_addr(&self) -> Address {
        string_to_address(self)
    }
}

impl AddressParsing for &str {
    fn parse_addr(&self) -> Address {
        string_to_address(&self.to_string())
    }
}

/// Used to convert `String` to [`Address`]
pub fn string_to_address(full_address: &String) -> Address {
    let full_address = full_address
        .trim()
        .replace(".", "")
        .replace(",", "")
        .replace("#", "")
        .to_ascii_uppercase();
    let address: Address = Address { 
        street_no: None, 
        direction: None, 
        street_name: None, 
        street_type: None, 
        unit_no: None, 
        unit_type: None
    };

    address
        .street_no(&full_address)
        .directional(&full_address)
        .street_type(&full_address)
        .unit_type_and_no(&full_address)
        .street_name(&full_address)
}
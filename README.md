# US Address Parser
A Rust Crate that parses United States street addresses.

## How To Use

To get started, simply import the [`AddressParsing`] trait to your file, and add the `parse_addr()` method to the [`String`] or [`&str`] you want to parse. It will return the [`Address`] struct, which contains; the street number, street name, street type, unit type, and unit number. Each of these fields are `Option<String>`.

```rust
use us_address_parser::AddressParsing;

fn main() {
    let addresses: Vec<&str> = vec![
        "123 Main Ave",
        "456 Maple Blvd Apt 122",
        "789 3rd St Lot B"
    ];

    for addr in addresses {
        println!("{}", &addr);

        let address = addr.parse_addr();
        
        println!("Street_no:{:?}, dir:{:?}, street_name:{:?}, street_type:{:?}, unit_type:{:?}, unit_no:{:?}\n==========", 
            address.street_no, address.direction, address.street_name,
            address.street_type, address.unit_type, address.unit_no
        );
    }
}
```

##### Result:
```
123 Main Ave
Street_no:Some("123"), dir:None, street_name:Some("MAIN"), street_type:Some("AVE"), unit_type:None, unit_no:None
==========
456 Maple Blvd Apt 122
Street_no:Some("456"), dir:None, street_name:Some("MAPLE"), street_type:Some("BLVD"), unit_type:Some("APT"), unit_no:Some("122")
==========
789 3rd St Lot B
Street_no:Some("789"), dir:None, street_name:Some("3RD"), street_type:Some("ST"), unit_type:Some("LOT"), unit_no:Some("B")
==========
```

### Implementing A Custom Type

The [`AddressParsing`] trait is already implemented for strings, 

```rust
struct CustomerInfo {
    id: usize,
    customer_name: String,
    customer_address: String
}

impl AddressParsing for CustomerInfo {
    fn parse_addr(&self) -> Address {
        let address = self.customer_address;
        us_address_parser::string_to_address(address)
    }
}
```

The `string_to_address()` function does what it's name implies, converts a [`String`] to an [`Address`].
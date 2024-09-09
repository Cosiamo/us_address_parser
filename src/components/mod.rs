pub mod directional;
pub mod states;
pub mod street_type;
pub mod unit;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Full(String),
    Abbreviated(String)
}

#[derive(Debug, PartialEq)]
pub enum StreetType {
    Full(String),
    Abbreviated(String)
}

#[derive(Debug, PartialEq)]
pub enum State {
    Full(String),
    Abbreviated(String)
}

#[derive(Debug, PartialEq)]
pub enum Unit {
    Full(String),
    Abbreviated(String)
}

macro_rules! to_string {
    ($type:ident, $input:ident) => {{
        let val = match $input {
            $type::Full(val) => val,
            $type::Abbreviated(val) => val,
        };
        (val).to_string()
    }};
}

impl Direction {
    pub fn to_string(&self) -> String { to_string!(Direction, self) }
}

impl StreetType {
    pub fn to_string(&self) -> String { to_string!(StreetType, self) }
}

impl State {
    pub fn to_string(&self) -> String { to_string!(State, self) }
}

impl Unit {
    pub fn to_string(&self) -> String { to_string!(Unit, self) }
}
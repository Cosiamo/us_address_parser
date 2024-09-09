use once_cell::sync::Lazy;
use regex::Regex;

use crate::components::{directional::{DIRECTIONAL, DIRECTIONAL_ABBVR}, street_type::{STREET_TYPE, STREET_TYPE_ABBVR}, unit::{UNIT, UNIT_ABBVR}};

pub static REG_STREET_ABBVR0: Lazy<Regex> = Lazy::new(||{
    let size = STREET_TYPE_ABBVR.len() / 4;
    let modified = STREET_TYPE_ABBVR[..size].iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_STREET_ABBVR1: Lazy<Regex> = Lazy::new(||{
    let size = STREET_TYPE_ABBVR.len() / 4;
    let modified = STREET_TYPE_ABBVR[size..size*2].iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_STREET_ABBVR2: Lazy<Regex> = Lazy::new(||{
    let size = STREET_TYPE_ABBVR.len() / 4;
    let modified = STREET_TYPE_ABBVR[size*2..size*3].iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_STREET_ABBVR3: Lazy<Regex> = Lazy::new(||{
    let size = STREET_TYPE_ABBVR.len() / 4;
    let modified = STREET_TYPE_ABBVR[size*3..].iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_DIRECTIONAL_ABBVR: Lazy<Regex> = Lazy::new(||{
    let modified = DIRECTIONAL_ABBVR.iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_UNIT_ABBVR: Lazy<Regex> = Lazy::new(||{
    let modified = UNIT_ABBVR.iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_STREET_NUMBER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\d+[ ]\.?").unwrap()
});

pub static REG_UNIT_NO: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[ ]+\d+\.?").unwrap()
});

pub static REG_UNIT_CHAR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\w)$").unwrap()
});

pub static REG_STREET0: Lazy<Regex> = Lazy::new(||{
    let size = STREET_TYPE.len() / 4;
    let modified = STREET_TYPE[..size].iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_STREET1: Lazy<Regex> = Lazy::new(||{
    let size = STREET_TYPE.len() / 4;
    let modified = STREET_TYPE[size..size*2].iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_STREET2: Lazy<Regex> = Lazy::new(||{
    let size = STREET_TYPE.len() / 4;
    let modified = STREET_TYPE[size*2..size*3].iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_STREET3: Lazy<Regex> = Lazy::new(||{
    let size = STREET_TYPE.len() / 4;
    let modified = STREET_TYPE[size*3..].iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_DIRECTIONAL: Lazy<Regex> = Lazy::new(||{
    let modified = DIRECTIONAL.iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_UNIT: Lazy<Regex> = Lazy::new(||{
    let modified = UNIT.iter().map(|street| {
        format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase())
    }).collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});
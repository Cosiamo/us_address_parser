use once_cell::sync::Lazy;
use regex::Regex;

use crate::components::{
    directional::{DIRECTIONAL, DIRECTIONAL_ABBVR},
    unit::{UNIT, UNIT_ABBVR},
};

pub fn regex_map(haystack: &str, phrase: &Lazy<Regex>) -> Option<String> {
    if let Some(needle) = phrase.captures(&haystack) {
        let val = needle.get(0).map_or("", |m| m.as_str());
        return Some(val.trim().replace("\"", ""));
    } else {
        None
    }
}

pub static REG_DIRECTIONAL_ABBVR: Lazy<Regex> = Lazy::new(|| {
    let modified = DIRECTIONAL_ABBVR
        .iter()
        .map(|street| format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase()))
        .collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_UNIT_ABBVR: Lazy<Regex> = Lazy::new(|| {
    let modified = UNIT_ABBVR
        .iter()
        .map(|street| format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase()))
        .collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_STREET_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+[ ]\.?").unwrap());

pub static REG_UNIT_NO: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ ]+\d+\.?$").unwrap());

pub static REG_UNIT_CHAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w)$").unwrap());

pub static REG_DIRECTIONAL: Lazy<Regex> = Lazy::new(|| {
    let modified = DIRECTIONAL
        .iter()
        .map(|street| format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase()))
        .collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

pub static REG_UNIT: Lazy<Regex> = Lazy::new(|| {
    let modified = UNIT
        .iter()
        .map(|street| format!(r"(?:^|\W){}(?:$|\W)", street.to_ascii_uppercase()))
        .collect::<Vec<String>>();
    Regex::new(&(&modified.join("|"))).unwrap()
});

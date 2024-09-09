use once_cell::sync::Lazy;

pub static UNIT: Lazy<Vec<&str>> = Lazy::new(||{
    vec![
        "Apartment",
        "Building",
        "Basement",
        "Department",
        "Floor",
        "Front",
        "Hanger",
        "Key",
        "Lobby",
        "Lot",
        "Lower",
        "Office",
        "Penthouse",
        "Pier",
        "Rear",
        "Room",
        "Side",
        "Slip",
        "Space",
        "Suite",
        "Stop",
        "Trailer",
        "Unit",
        "Upper",
    ]
});

pub static UNIT_ABBVR: Lazy<Vec<&str>> = Lazy::new(||{
    vec![
        // Apartment
        "APT",
        // Building
        "BLDG",      
        // Basement
        "BSMT",
        // Department
        "DEPT",
        // Floor
        "FL ",
        // Front
        "FRNT",
        // Hanger
        "HNGR",
        // Key
        "KEY",
        // Lobby
        "LBBY",
        // Lot
        "LOT",
        // Lower
        "LOWR",
        // Office
        "OFC",
        // Penthouse
        "PH ",
        // Pier
        "PIER",
        // Rear
        "REAR",
        // Room
        "RM ",
        // Side
        "SIDE",
        // Slip
        "SLIP",
        // Space
        "SPC",
        // Suite
        "STE",
        // Stop
        "STOP",
        // Trailer
        "TRLR",
        // Unit
        "UNIT",
        // Upper
        "UPPR",
    ]
});
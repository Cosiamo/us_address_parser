use once_cell::sync::Lazy;

pub static DIRECTIONAL: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "north",
        "northeast",
        "east",
        "southeast",
        "south",
        "southwest",
        "west",
        "northwest",
    ]
});

pub static DIRECTIONAL_ABBVR: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        // north     
        "N",
        // northeast 
        "NE",
        // east      
        "E",
        // southeast 
        "SE",
        // south     
        "S",
        // southwest 
        "SW",
        // west      
        "W",
        // northwest 
        "NW",
    ]
});
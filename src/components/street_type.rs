use once_cell::sync::Lazy;

pub static STREET_TYPE: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "allee",
        "alley",
        "ally",
        "anex",
        "annex",
        "annx",
        "arcade",
        "av",
        "aven",
        "avenu",
        "avenue",
        "avn",
        "avnue",
        "bayoo",
        "bayou",
        "beach",
        "bend",
        "bluf",
        "bluff",
        "bluffs",
        "bot",
        "bottm",
        "bottom",
        "boul",
        "boulevard",
        "boulv",
        "branch",
        "brdge",
        "bridge",
        "brnch",
        "brook",
        "brooks",
        "burg",
        "burgs",
        "bypa",
        "bypas",
        "bypass",
        "byps",
        "camp",
        "canyn",
        "canyon",
        "cape",
        "causeway",
        "causway",
        "causwa",
        "cen",
        "cent",
        "center",
        "centers",
        "centr",
        "centre",
        "circ",
        "circl",
        "circle",
        "circles",
        "ck",
        "cliff",
        "cliffs",
        "club",
        "cmp",
        "cnter",
        "cntr",
        "cnyn",
        "common",
        "commons",
        "corner",
        "corners",
        "course",
        "court",
        "courts",
        "cove",
        "coves",
        "cr",
        "crcl",
        "crcle",
        "crecent",
        "creek",
        "crescent",
        "cresent",
        "crest",
        "crossing",
        "crossroad",
        "crossroads",
        "crscnt",
        "crsent",
        "crsnt",
        "crssing",
        "crssng",
        "crt",
        "curve",
        "dale",
        "dam",
        "div",
        "divide",
        "driv",
        "drive",
        "drives",
        "drv",
        "dvd",
        "estate",
        "estates",
        "exp",
        "expr",
        "express",
        "expressway",
        "expw",
        "extension",
        "extensions",
        "extn",
        "extnsn",
        "fall",
        "falls",
        "ferry",
        "field",
        "fields",
        "flat",
        "flats",
        "ford",
        "fords",
        "forest",
        "forests",
        "forg",
        "forge",
        "forges",
        "fork",
        "forks",
        "fort",
        "freeway",
        "freewy",
        "frry",
        "frt",
        "frway",
        "frwy",
        "garden",
        "gardens",
        "gardn",
        "gateway",
        "gatewy",
        "gatway",
        "glen",
        "glens",
        "grden",
        "grdn",
        "grdns",
        "green",
        "greens",
        "grov",
        "grove",
        "groves",
        "gtway",
        "harb",
        "harbor",
        "harbors",
        "harbr",
        "haven",
        "havn",
        "height",
        "heights",
        "hgts",
        "highway",
        "highwy",
        "hill",
        "hills",
        "hiway",
        "hiwy",
        "hllw",
        "hollow",
        "hollows",
        "holws",
        "hrbor",
        "ht",
        "hway",
        "inlet",
        "island",
        "islands",
        "isles",
        "islnd",
        "islnds",
        "jction",
        "jctn",
        "jctns",
        "junction",
        "junctions",
        "junctn",
        "juncton",
        "key",
        "keys",
        "knol",
        "knoll",
        "knolls",
        "la",
        "lake",
        "lakes",
        "land",
        "landing",
        "lane",
        "lanes",
        "ldge",
        "light",
        "lights",
        "lndng",
        "loaf",
        "lock",
        "locks",
        "lodg",
        "lodge",
        "loops",
        "mall",
        "manor",
        "manors",
        "meadow",
        "meadows",
        "medows",
        "mews",
        "mill",
        "mills",
        "mission",
        "missn",
        "mnt",
        "mntain",
        "mntn",
        "mntns",
        "motorway",
        "mount",
        "mountain",
        "mountains",
        "mountin",
        "mssn",
        "mtin",
        "neck",
        "orchard",
        "orchrd",
        "overpass",
        "ovl",
        "parks",
        "parkway",
        "parkways",
        "parkwy",
        "pass",
        "passage",
        "paths",
        "pikes",
        "pine",
        "pines",
        "pk",
        "pkway",
        "pkwys",
        "pky",
        "place",
        "plain",
        "plaines",
        "plains",
        "plaza",
        "plza",
        "point",
        "points",
        "port",
        "ports",
        "prairie",
        "prarie",
        "prk",
        "prr",
        "rad",
        "radial",
        "radiel",
        "ranch",
        "ranches",
        "rapid",
        "rapids",
        "rdge",
        "rest",
        "ridge",
        "ridges",
        "river",
        "rivr",
        "rnchs",
        "road",
        "roads",
        "route",
        "rvr",
        "row",
        "rue",
        "run",
        "shoal",
        "shoals",
        "shoar",
        "shoars",
        "shore",
        "shores",
        "skyway",
        "spng",
        "spngs",
        "spring",
        "springs",
        "sprng",
        "sprngs",
        "spurs",
        "sqr",
        "sqre",
        "sqrs",
        "squ",
        "square",
        "squares",
        "station",
        "statn",
        "stn",
        "str",
        "strav",
        "strave",
        "straven",
        "stravenue",
        "stravn",
        "stream",
        "street",
        "streets",
        "streme",
        "strt",
        "strvn",
        "strvnue",
        "sumit",
        "sumitt",
        "summit",
        "terr",
        "terrace",
        "throughway",
        "tpk",
        "tr",
        "trace",
        "traces",
        "track",
        "tracks",
        "trafficway",
        "trail",
        "trails",
        "trk",
        "trks",
        "trls",
        "trnpk",
        "trpk",
        "tunel",
        "tunls",
        "tunnel",
        "tunnels",
        "tunnl",
        "turnpike",
        "turnpk",
        "turn",
        "underpass",
        "union",
        "unions",
        "valley",
        "valleys",
        "vally",
        "vdct",
        "viadct",
        "viaduct",
        "view",
        "views",
        "vill",
        "villag",
        "village",
        "villages",
        "ville",
        "villg",
        "villiage",
        "vist",
        "vista",
        "vlly",
        "vst",
        "vsta",
        "wall",
        "walks",
        "well",
        "wells",
        "wy",
    ]
});

pub static STREET_TYPE_ABBVR: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        // allee 
        "aly",
        // alley 
        "aly",
        // ally 
        "aly",
        // anex 
        "anx",
        // annex 
        "anx",
        // annx 
        "anx",
        // arcade 
        "arc",
        // av 
        "ave",
        // aven 
        "ave",
        // avenu 
        "ave",
        // avenue 
        "ave",
        // avn 
        "ave",
        // avnue 
        "ave",
        // bayoo 
        "byu",
        // bayou 
        "byu",
        // beach 
        "bch",
        // bend 
        "bnd",
        // bluf 
        "blf",
        // bluff 
        "blf",
        // bluffs 
        "blfs",
        // bot 
        "btm",
        // bottm 
        "btm",
        // bottom 
        "btm",
        // boul 
        "blvd",
        // boulevard 
        "blvd",
        // boulv 
        "blv", // was "blvd"
        // branch 
        "br",
        // brdge 
        "brg",
        // bridge 
        "brg",
        // brnch 
        "br",
        // brook 
        "brk",
        // brooks 
        "brks",
        // burg 
        "bg",
        // burgs 
        "bgs",
        // bypa 
        "byp",
        // bypas 
        "byp",
        // bypass 
        "byp",
        // byps 
        "byp",
        // camp 
        "cp",
        // canyn 
        "cyn",
        // canyon 
        "cyn",
        // cape 
        "cpe",
        // causeway 
        "cswy",
        // causway 
        "cswy",
        // causwa 
        "cswy",
        // cen 
        "ctr",
        // cent 
        "ctr",
        // center 
        "ctr",
        // centers 
        "ctrs",
        // centr 
        "ctr",
        // centre 
        "ctr",
        // circ 
        "cir",
        // circl 
        "cir",
        // circle 
        "cir",
        // circles 
        "cirs",
        // ck 
        "crk",
        // cliff 
        "clf",
        // cliffs 
        "clfs",
        // club 
        "clb",
        // cmp 
        "cp",
        // cnter 
        "ctr",
        // cntr 
        "ctr",
        // cnyn 
        "cyn",
        // common 
        "cmn",
        // commons 
        "cmns",
        // corner 
        "cor",
        // corners 
        "cors",
        // course 
        "crse",
        // court 
        "ct",
        // courts 
        "cts",
        // cove 
        "cv",
        // coves 
        "cvs",
        // cr 
        "crk",
        // crcl 
        "cir",
        // crcle 
        "cir",
        // crecent 
        "cres",
        // creek 
        "crk",
        // crescent 
        "cres",
        // cresent 
        "cres",
        // crest 
        "crst",
        // crossing 
        "xing",
        // crossroad 
        "xrd",
        // crossroads 
        "xrds",
        // crscnt 
        "cres",
        // crsent 
        "cres",
        // crsnt 
        "cres",
        // crssing 
        "xing",
        // crssng 
        "xing",
        // crt 
        "ct",
        // curve 
        "curv",
        // dale 
        "dl",
        // dam 
        "dm",
        // div 
        "dv",
        // divide 
        "dv",
        // driv 
        "dr",
        // drive 
        "dr",
        // drives 
        "drs",
        // drv 
        "dr",
        // dvd 
        "dv",
        // estate 
        "est",
        // estates 
        "ests",
        // exp 
        "expy",
        // expr 
        "expy",
        // express 
        "expy",
        // expressway 
        "expy",
        // expw 
        "expy",
        // extension 
        "ext",
        // extensions 
        "exts",
        // extn 
        "ext",
        // extnsn 
        "ext",
        // fall 
        "fall",
        // falls 
        "fls",
        // ferry 
        "fry",
        // field 
        "fld",
        // fields 
        "flds",
        // flat 
        "flt",
        // flats 
        "flts",
        // ford 
        "frd",
        // fords 
        "frds",
        // forest 
        "frst",
        // forests 
        "frst",
        // forg 
        "frg",
        // forge 
        "frg",
        // forges 
        "frgs",
        // fork 
        "frk",
        // forks 
        "frks",
        // fort 
        "ft",
        // freeway 
        "fwy",
        // freewy 
        "fwy",
        // frry 
        "fry",
        // frt 
        "ft",
        // frway 
        "fwy",
        // frwy 
        "fwy",
        // garden 
        "gdn",
        // gardens 
        "gdns",
        // gardn 
        "gdn",
        // gateway 
        "gtwy",
        // gatewy 
        "gtwy",
        // gatway 
        "gtwy",
        // glen 
        "gln",
        // glens 
        "glns",
        // grden 
        "gdn",
        // grdn 
        "gdn",
        // grdns 
        "gdns",
        // green 
        "grn",
        // greens 
        "grns",
        // grov 
        "grv",
        // grove 
        "grv",
        // groves 
        "grvs",
        // gtway 
        "gtwy",
        // harb 
        "hbr",
        // harbor 
        "hbr",
        // harbors 
        "hbrs",
        // harbr 
        "hbr",
        // haven 
        "hvn",
        // havn 
        "hvn",
        // height 
        "hts",
        // heights 
        "hts",
        // hgts 
        "hts",
        // highway 
        "hwy",
        // highwy 
        "hwy",
        // hill 
        "hl",
        // hills 
        "hls",
        // hiway 
        "hwy",
        // hiwy 
        "hwy",
        // hllw 
        "holw",
        // hollow 
        "holw",
        // hollows 
        "holw",
        // holws 
        "holw",
        // hrbor 
        "hbr",
        // ht 
        "hts",
        // hway 
        "hwy",
        // inlet 
        "inlt",
        // island 
        "is",
        // islands 
        "iss",
        // isles 
        "isle",
        // islnd 
        "is",
        // islnds 
        "iss",
        // jction 
        "jct",
        // jctn 
        "jct",
        // jctns 
        "jcts",
        // junction 
        "jct",
        // junctions 
        "jcts",
        // junctn 
        "jct",
        // juncton 
        "jct",
        // key 
        "ky",
        // keys 
        "kys",
        // knol 
        "knl",
        // knoll 
        "knl",
        // knolls 
        "knls",
        // la 
        "ln",
        // lake 
        "lk",
        // lakes 
        "lks",
        // land 
        "land",
        // landing 
        "lndg",
        // lane 
        "ln",
        // lanes 
        "ln",
        // ldge 
        "ldg",
        // light 
        "lgt",
        // lights 
        "lgts",
        // lndng 
        "lndg",
        // loaf 
        "lf",
        // lock 
        "lck",
        // locks 
        "lcks",
        // lodg 
        "ldg",
        // lodge 
        "ldg",
        // loops 
        "loop",
        "loo",
        "lp",
        // mall 
        "mall",
        // manor 
        "mnr",
        // manors 
        "mnrs",
        // meadow 
        "mdw",
        // meadows 
        "mdws",
        // medows 
        "mdws",
        // mews 
        "mews",
        // mill 
        "ml",
        // mills 
        "mls",
        // mission 
        "msn",
        // missn 
        "msn",
        // mnt 
        "mt",
        // mntain 
        "mtn",
        // mntn 
        "mtn",
        // mntns 
        "mtns",
        // motorway 
        "mtwy",
        // mount 
        "mt",
        // mountain 
        "mtn",
        // mountains 
        "mtns",
        // mountin 
        "mtn",
        // mssn 
        "msn",
        // mtin 
        "mtn",
        // neck 
        "nck",
        // orchard 
        "orch",
        // orchrd 
        "orch",
        // overpass 
        "opas",
        // ovl 
        "oval",
        // parks 
        "park",
        // parkway 
        "pkwy",
        // parkways 
        "pkwy",
        // parkwy 
        "pkwy",
        // pass 
        "pass",
        // passage 
        "psge",
        // paths 
        "path",
        "pat",
        // pikes 
        "pike",
        // pine 
        "pne",
        // pines 
        "pnes",
        // pk 
        "park",
        // pkway 
        "pkwy",
        // pkwys 
        "pkwy",
        // pky 
        "pkw", // "pkwy",
        // place 
        "pl",
        // plain 
        "pln",
        // plaines 
        "plns",
        // plains 
        "plns",
        // plaza 
        "plz",
        // plza 
        "plz",
        // point 
        "pt",
        // points 
        "pts",
        // port 
        "prt",
        // ports 
        "prts",
        // prairie 
        "pr",
        // prarie 
        "pr",
        // prk 
        "park",
        // prr 
        "pr",
        // rad 
        "radl",
        // radial 
        "radl",
        // radiel 
        "radl",
        // ranch 
        "rnch",
        // ranches 
        "rnch",
        // rapid 
        "rpd",
        // rapids 
        "rpds",
        // rdge 
        "rdg",
        // rest 
        "rst",
        // ridge 
        "rdg",
        // ridges 
        "rdgs",
        // river 
        "riv",
        // rivr 
        "riv",
        // rnchs 
        "rnch",
        // road 
        "rd",
        // roads 
        "rds",
        // route 
        "rte",
        // rvr 
        "riv",
        // row 
        "row",
        // rue 
        "rue",
        // run 
        "run",
        // shoal 
        "shl",
        // shoals 
        "shls",
        // shoar 
        "shr",
        // shoars 
        "shrs",
        // shore 
        "shr",
        // shores 
        "shrs",
        // skyway 
        "skwy",
        // spng 
        "spg",
        // spngs 
        "spgs",
        // spring 
        "spg",
        // springs 
        "spgs",
        // sprng 
        "spg",
        // sprngs 
        "spgs",
        // spurs 
        "spur",
        // sqr 
        "sq",
        // sqre 
        "sq",
        // sqrs 
        "sqs",
        // squ 
        "sq",
        // square 
        "sq",
        // squares 
        "sqs",
        // station 
        "sta",
        // statn 
        "sta",
        // stn 
        "sta",
        // str 
        "st",
        // strav 
        "stra",
        // strave 
        "stra",
        // straven 
        "stra",
        // stravenue 
        "stra",
        // stravn 
        "stra",
        // stream 
        "strm",
        // street 
        "st",
        // streets 
        "sts",
        // streme 
        "strm",
        // strt 
        "st",
        // strvn 
        "stra",
        // strvnue 
        "stra",
        // sumit 
        "smt",
        // sumitt 
        "smt",
        // summit 
        "smt",
        // terr 
        "ter",
        // terrace 
        "ter",
        // throughway 
        "trwy",
        // tpk 
        "tpke",
        // tr 
        "trl",
        // trace 
        "trce",
        // traces 
        "trce",
        // track 
        "trak",
        // tracks 
        "trak",
        // trafficway 
        "trfy",
        // trail 
        "trl",
        // trails 
        "trl",
        // trk 
        "trak",
        // trks 
        "trak",
        // trls 
        "trl",
        // trnpk 
        "tpke",
        // trpk 
        "tpke",
        // tunel 
        "tunl",
        // tunls 
        "tunl",
        // tunnel 
        "tunl",
        // tunnels 
        "tunl",
        // tunnl 
        "tunl",
        // turnpike 
        "tpke",
        // turnpk 
        "tpke",
        // underpass 
        "upas",
        // union 
        "un",
        // unions 
        "uns",
        // valley 
        "vly",
        // valleys 
        "vlys",
        // vally 
        "vly",
        // vdct 
        "via",
        // viadct 
        "via",
        // viaduct 
        "via",
        // view 
        "vw",
        // views 
        "vws",
        // vill 
        "vlg",
        // villag 
        "vlg",
        // village 
        "vlg",
        // villages 
        "vlgs",
        // ville 
        "vl",
        // villg 
        "vlg",
        // villiage 
        "vlg",
        // vist 
        "vis",
        // vista 
        "vis",
        // vlly 
        "vly",
        // vst 
        "vis",
        // vsta 
        "vis",
        // wall 
        "wall",
        // walks 
        "walk",
        // well 
        "wl",
        // wells 
        "wls",
        // wy 
        "way",
    ]
});
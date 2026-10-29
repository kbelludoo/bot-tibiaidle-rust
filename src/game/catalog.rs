#[derive(Debug, Clone)]
pub struct HuntRoute {
    pub id: u32,
    pub route_str: &'static str,
    pub min_level: u32,
    pub max_level: u32,
    pub base_exp: f64,
    pub base_gold: f64,
}

pub type AreaCandidate = HuntRoute;

pub static ALL_KNOWN_HUNT_IDENTIFIERS: &[&str] = &[
    "hunt-26", "hunt-28", "hunt-30", "hunt-32", "hunt-34", "hunt-38",
    "hunt-43", "hunt-45", "hunt-46", "hunt-49", "hunt-50", "hunt-52",
    "hunt-54", "hunt-55", "hunt-56", "hunt-58", "hunt-60", "hunt-62",
    "hunt-63", "hunt-64", "hunt-66", "hunt-68", "hunt-70", "hunt-72",
    "hunt-74", "hunt-75", "hunt-76", "hunt-78", "hunt-80", "hunt-82",
    "hunt-84", "hunt-85", "hunt-86", "hunt-88", "hunt-90", "hunt-92",
    "hunt-94", "hunt-95", "hunt-96", "hunt-98", "hunt-100", "hunt-102",
    "hunt-104", "hunt-105", "hunt-106", "hunt-108", "hunt-110", "hunt-112",
    "hunt-114", "hunt-115", "hunt-116", "hunt-118", "hunt-120", "hunt-122",
    "hunt-124", "hunt-125", "hunt-126", "hunt-128", "hunt-130", "hunt-132",
    "hunt-134", "hunt-135", "hunt-136", "hunt-138", "hunt-140", "hunt-142",
    "hunt-144", "hunt-145", "hunt-146", "hunt-148", "hunt-150", "hunt-152",
    "hunt-154", "hunt-155", "hunt-156", "hunt-158", "hunt-160", "hunt-162",
    "hunt-164", "hunt-165", "hunt-166", "hunt-168", "hunt-170", "hunt-172",
    "hunt-174", "hunt-175", "hunt-176", "hunt-178", "hunt-180", "hunt-182",
    "hunt-184", "hunt-185", "hunt-186", "hunt-188", "hunt-190", "hunt-192",
    "hunt-194", "hunt-195", "hunt-196", "hunt-198", "hunt-200", "hunt-202",
    "hunt-204", "hunt-205", "hunt-206", "hunt-208", "hunt-210", "hunt-212",
    "hunt-214", "hunt-215", "hunt-216", "hunt-218", "hunt-220",
];

pub static TIBIA_HUNTS_CATALOG: &[HuntRoute] = &[
    HuntRoute { id: 26, route_str: "hunt-26", min_level: 1, max_level: 20, base_exp: 120.0, base_gold: 60.0 },
    HuntRoute { id: 30, route_str: "hunt-30", min_level: 20, max_level: 50, base_exp: 300.0, base_gold: 180.0 },
    HuntRoute { id: 38, route_str: "hunt-38", min_level: 50, max_level: 80, base_exp: 800.0, base_gold: 450.0 },
    HuntRoute { id: 46, route_str: "hunt-46", min_level: 80, max_level: 120, base_exp: 1900.0, base_gold: 1100.0 },
    HuntRoute { id: 54, route_str: "hunt-54", min_level: 120, max_level: 170, base_exp: 4200.0, base_gold: 2400.0 },
    HuntRoute { id: 66, route_str: "hunt-66", min_level: 170, max_level: 230, base_exp: 9500.0, base_gold: 5800.0 },
    HuntRoute { id: 78, route_str: "hunt-78", min_level: 230, max_level: 310, base_exp: 21000.0, base_gold: 13000.0 },
    HuntRoute { id: 90, route_str: "hunt-90", min_level: 310, max_level: 400, base_exp: 48000.0, base_gold: 30000.0 },
    HuntRoute { id: 120, route_str: "hunt-120", min_level: 400, max_level: 600, base_exp: 110000.0, base_gold: 72000.0 },
    HuntRoute { id: 160, route_str: "hunt-160", min_level: 600, max_level: 900, base_exp: 280000.0, base_gold: 180000.0 },
    HuntRoute { id: 220, route_str: "hunt-220", min_level: 900, max_level: 1800, base_exp: 750000.0, base_gold: 480000.0 },
];

#[derive(Clone, Default)]
pub struct Catalog;

impl Catalog {
    pub fn select_best_hunt(&self, level: u32) -> &'static HuntRoute {
        for area in TIBIA_HUNTS_CATALOG {
            if level >= area.min_level && level <= area.max_level {
                return area;
            }
        }
        &TIBIA_HUNTS_CATALOG[TIBIA_HUNTS_CATALOG.len() - 1]
    }

    pub fn has_route(route: &str) -> bool {
        ALL_KNOWN_HUNT_IDENTIFIERS.contains(&route)
    }
}

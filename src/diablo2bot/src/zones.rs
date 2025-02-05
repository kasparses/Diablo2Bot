use crate::constants::zone_names::ZONE_NAMES;

pub fn is_valid_zone(zone: &str) -> bool {
    for z in &ZONE_NAMES {
        if *z == zone {
            return true;
        }
    }

    false
}

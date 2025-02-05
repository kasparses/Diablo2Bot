use std::collections::{HashMap, HashSet};

use super::zone_names::{
    ARREAT_PLATEAU, BARRACKS, BLACK_MARSH, BLOODY_FOOTHILLS, BLOOD_MOOR, BURIAL_GROUNDS, CATHEDRAL,
    CITY_OF_THE_DAMNED, COLD_PLAINS, DARK_WOOD, DRY_HILLS, FAR_OASIS, FLAYER_JUNGLE,
    FRIGID_HIGHLANDS, GREAT_MARSH, INNER_CLOISTER, KURAST_BAZAAR, KURAST_CAUSEWAY, LOST_CITY,
    LOWER_KURAST, MONASTERY_GATE, OUTER_CLOISTER, OUTER_STEPPES, PLAINS_OF_DESPAIR, RIVER_OF_FLAME,
    ROCKY_WASTE, SPIDER_FOREST, STONY_FIELD, TAMOE_HIGHLAND, THE_CHAOS_SANCTUARY, TRAVINCAL,
    UPPER_KURAST, VALLEY_OF_SNAKES,
};

struct ConnectedAreas {
    a: &'static str,
    b: &'static str,
}

const CONNECTED_AREAS: [ConnectedAreas; 25] = [
    ConnectedAreas {
        a: BLOOD_MOOR,
        b: COLD_PLAINS,
    },
    ConnectedAreas {
        a: COLD_PLAINS,
        b: STONY_FIELD,
    },
    ConnectedAreas {
        a: COLD_PLAINS,
        b: BURIAL_GROUNDS,
    },
    ConnectedAreas {
        a: DARK_WOOD,
        b: BLACK_MARSH,
    },
    ConnectedAreas {
        a: BLACK_MARSH,
        b: TAMOE_HIGHLAND,
    },
    ConnectedAreas {
        a: TAMOE_HIGHLAND,
        b: MONASTERY_GATE,
    },
    ConnectedAreas {
        a: MONASTERY_GATE,
        b: OUTER_CLOISTER,
    },
    ConnectedAreas {
        a: OUTER_CLOISTER,
        b: BARRACKS,
    },
    ConnectedAreas {
        a: INNER_CLOISTER,
        b: CATHEDRAL,
    },
    ConnectedAreas {
        a: ROCKY_WASTE,
        b: DRY_HILLS,
    },
    ConnectedAreas {
        a: DRY_HILLS,
        b: FAR_OASIS,
    },
    ConnectedAreas {
        a: FAR_OASIS,
        b: LOST_CITY,
    },
    ConnectedAreas {
        a: LOST_CITY,
        b: VALLEY_OF_SNAKES,
    },
    ConnectedAreas {
        a: SPIDER_FOREST,
        b: GREAT_MARSH,
    },
    ConnectedAreas {
        a: GREAT_MARSH,
        b: FLAYER_JUNGLE,
    },
    ConnectedAreas {
        a: FLAYER_JUNGLE,
        b: LOWER_KURAST,
    },
    ConnectedAreas {
        a: LOWER_KURAST,
        b: KURAST_BAZAAR,
    },
    ConnectedAreas {
        a: KURAST_BAZAAR,
        b: UPPER_KURAST,
    },
    ConnectedAreas {
        a: UPPER_KURAST,
        b: KURAST_CAUSEWAY,
    },
    ConnectedAreas {
        a: KURAST_CAUSEWAY,
        b: TRAVINCAL,
    },
    ConnectedAreas {
        a: OUTER_STEPPES,
        b: PLAINS_OF_DESPAIR,
    },
    ConnectedAreas {
        a: PLAINS_OF_DESPAIR,
        b: CITY_OF_THE_DAMNED,
    },
    ConnectedAreas {
        a: RIVER_OF_FLAME,
        b: THE_CHAOS_SANCTUARY,
    },
    ConnectedAreas {
        a: BLOODY_FOOTHILLS,
        b: FRIGID_HIGHLANDS,
    },
    ConnectedAreas {
        a: BLOODY_FOOTHILLS,
        b: ARREAT_PLATEAU,
    },
];

pub fn get_connected_areas(area_name: &str) -> Vec<&str> {
    let directly_connected_areas = get_directly_connected_areas();
    let mut connected_areas = HashSet::new();

    traverse_connected_areas(&directly_connected_areas, &mut connected_areas, area_name);

    connected_areas.remove(area_name);

    connected_areas.into_iter().collect()
}

fn traverse_connected_areas<'a>(
    directly_connected_areas: &HashMap<&'a str, Vec<&'a str>>,
    connected_areas: &mut HashSet<&'a str>,
    area_name: &'a str,
) {
    if !connected_areas.insert(area_name) {
        return;
    }

    if let Some(areas) = directly_connected_areas.get(area_name) {
        for area in areas {
            traverse_connected_areas(directly_connected_areas, connected_areas, area);
        }
    }
}

fn get_directly_connected_areas() -> HashMap<&'static str, Vec<&'static str>> {
    let mut directly_connected_areas = HashMap::new();

    for areas in CONNECTED_AREAS.iter() {
        directly_connected_areas
            .entry(areas.a)
            .or_insert_with(Vec::new)
            .push(areas.b);

        directly_connected_areas
            .entry(areas.b)
            .or_insert_with(Vec::new)
            .push(areas.a);
    }

    directly_connected_areas
}

#[cfg(test)]
mod tests {
    use crate::constants::{
        connected_areas::get_connected_areas,
        zone_names::{BLOOD_MOOR, BURIAL_GROUNDS, COLD_PLAINS, STONY_FIELD},
    };

    #[test]
    fn test_connected_areas() {
        let mut expected_connected_areas = vec![BLOOD_MOOR, BURIAL_GROUNDS, STONY_FIELD];
        let mut actual_connected_areas = get_connected_areas(COLD_PLAINS);

        expected_connected_areas.sort();
        actual_connected_areas.sort();

        assert_eq!(actual_connected_areas, expected_connected_areas);
    }
}

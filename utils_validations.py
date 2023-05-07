from enums_diablo import WaypointZone

def validate_profile_integer(value, key_name: str) -> int:
    if type(value) == float:
        print(f'"{str(value)}" is not a valid "{key_name}". It must be a whole number (not a decimal number)!')
        return 1
    elif type(value) != int:
        print(f'"{str(value)}" is not a valid "{key_name}". It must be a number!')
        return 1
    elif value < 0:
        print(f'"{str(value)}" is not a valid "{key_name}". It must be a positive number!')
        return 1

    return 0

def validate_profile_bool(value, key_name: str) -> int:
    if type(value) != bool:
        print(f'"{str(value)}" is not a valid "{key_name}". It must be an boolean value (true/false)!')
        return 1

    return 0

def validate_profile_percentage_float(value, key_name: str) -> int:
    if type(value) != float:
        print(f'"{str(value)}" is not a valid "{key_name}". It must be an float/decimal number!')
        return 1
    elif value < 0.0:
        print(f'"{str(value)}" is not a valid "{key_name}". It must be a positive number!')
        return 1
    elif value >= 1.0:
        print(f'"{str(value)}" is not a valid "{key_name}". It must be below 1.0!')
        return 1

    return 0
        
def validate_profile_belt_reservations(value, key_name: str) -> int:
    if value > 4:
        print(f'"{str(value)}" is not a valid "{key_name}". It must be one of: "0,1,2,3,4"!')
        return 1

    return 0

def validate_profile_zone_to_farm(zone: WaypointZone) -> int:
    town_waypoint_zones = {
        WaypointZone.ROGUE_ENCAMPMENT,
        WaypointZone.LUT_GHOLEIN,
        WaypointZone.KURAST_DOCKS,
        WaypointZone.THE_PANDEMONIUM_FORTRESS,
        WaypointZone.HARROGATH
    }

    if zone in town_waypoint_zones:
        print(f'Your chosen zone to farm "{str(zone.value)}" is a town zone which contains no monsters!')
        return 1

    return 0
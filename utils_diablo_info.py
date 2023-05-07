import math

from enums_diablo import CharacterClass


def get_animation_speed(character_class: CharacterClass) -> int:
    animation_speed = None
    if character_class in (CharacterClass.AMAZON, CharacterClass.ASSASSIN, CharacterClass.BARBARIAN, CharacterClass.NECROMANCER, CharacterClass.SORCERESS, CharacterClass.PALADIN):
        animation_speed = 256
    elif character_class == CharacterClass.DRUID:
        animation_speed = 208
    return animation_speed

def get_base_action_flag(character_class: CharacterClass) -> int:
    base_action_flag_mapping = {
        CharacterClass.AMAZON: 13,
        CharacterClass.ASSASSIN: 9,
        CharacterClass.BARBARIAN: 9,
        CharacterClass.PALADIN: 9,
        CharacterClass.DRUID: 10,
        CharacterClass.NECROMANCER: 8,
        CharacterClass.SORCERESS: 7
        }
    return base_action_flag_mapping[character_class]

def calculate_effective_faster_cast_rate(faster_cast_rate: int) -> int:
    # https://d2.maxroll.gg/resources/breakpoints-animations
    return math.floor(faster_cast_rate * 120 / (faster_cast_rate + 120))

def get_casting_base(character_class: CharacterClass, calculate_for_lightning_skill: bool=False) -> int:
    casting_base_mapping = {
        (CharacterClass.AMAZON, False): 20,
        (CharacterClass.AMAZON, True): 20,

        (CharacterClass.ASSASSIN, False): 17,
        (CharacterClass.ASSASSIN, True): 17,

        (CharacterClass.SORCERESS, False): 14,
        (CharacterClass.SORCERESS, True): 19,

        (CharacterClass.BARBARIAN, False): 14,
        (CharacterClass.BARBARIAN, True): 14,

        (CharacterClass.DRUID, False): 16,
        (CharacterClass.DRUID, True): 16,

        (CharacterClass.PALADIN, False): 16,
        (CharacterClass.PALADIN, True): 16,

        (CharacterClass.NECROMANCER, False): 15,
        (CharacterClass.NECROMANCER, True): 15
        }

    return casting_base_mapping[(character_class, calculate_for_lightning_skill)]

def calculate_cast_frames(character_class: CharacterClass, faster_cast_rate: int) -> int:
    """Calculate the number of frames it takes before the spell is cast.
    Example of casting teleport with on a Sorceress with 120 faster_cast_rate:
    Frame number    
    0               click mouse
    1               spell animation
    2               spell animation
    3               spell animation
    4               spell animation
    5               spell is cast!          This is the frame number that this function returns
    6               finishes animation
    7               finishes animation
    8               finishes animation

    Args:
        faster_cast_rate (int): The amount of faster_cast_rate we have from our gear
    """
    effective_faster_cast_rate = calculate_effective_faster_cast_rate(faster_cast_rate)
    animation_speed = get_animation_speed(character_class)
    base_action_flag = get_base_action_flag(character_class)
    return math.ceil(base_action_flag * 256 / math.floor(animation_speed * (100 + effective_faster_cast_rate) / 100))

def calculate_cooldown_frames(character_class: CharacterClass, faster_cast_rate: int, calculate_lightning_cooldown: bool=False) -> int:
    """This function calculates the total amount of frames it takes before we can cast a new spell after casting a spell
    Example of casting teleport with a Sorceress with 120 faster_cast_rate:
    Frame number    
    0               click mouse
    1               spell animation
    2               spell animation
    3               spell animation
    4               spell animation
    5               spell is cast!
    6               finishes animation
    7               finishes animation
    8               finishes animation      This is the frame number that this function returns
    9               ready to cast new spell

    Args:
        faster_cast_rate (int): The amount of faster_cast_rate we have from our gear
    """
    effective_faster_cast_rate = calculate_effective_faster_cast_rate(faster_cast_rate)
    animation_speed = get_animation_speed(character_class)
    casting_base = get_casting_base(character_class, calculate_for_lightning_skill=calculate_lightning_cooldown)

    if calculate_lightning_cooldown:
        return math.ceil(256 * casting_base / math.floor(animation_speed * (100 + effective_faster_cast_rate) / 100))
    return math.ceil(256 * casting_base / math.floor(animation_speed * (100 + effective_faster_cast_rate) / 100) - 1)
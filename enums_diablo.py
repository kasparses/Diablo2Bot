from enum import Enum

from dataclasses_diablo import Point

class Difficulty(Enum):
    NORMAL = "Normal"
    NIGHTMARE = "Nightmare"
    HELL = "Hell"
class ClickType(Enum):
    LEFT = "Left"
    RIGHT = "right"
class Skill(Enum):
    """All castable skills in the game
    """
    # Sorceress:
    # Fire Spells:
    FIRE_BOLT = "Fire Bolt"
    INFERNO = "Inferno"
    BLAZE = "Blaze"
    FIRE_BALL = "Fire Ball"
    FIRE_WALL = "Fire Wall"
    ENCHANT = "Enchant"
    METEOR = "Meteor"
    HYDRA = "Hydra"

    # Lightning Spells:
    CHARGED_BOLT = "Charged Bolt"
    STATIC_FIELD = "Static Field"
    TELEKINESIS = "Telekinesis"
    NOVA = "Nova"
    LIGHTNING = "Lightning"
    CHAIN_LIGHTNING = "Chain Lightning"
    TELEPORT = "Teleport"
    THUNDER_STORM = "Thunder Storm"
    ENERGY_SHIELD = "Energy Shield"

    # Cold Spells:
    ICE_BOLT = "Ice Bolt"
    FROZEN_ARMOR = "Frozen Armor"
    FROST_NOVA = "Frost Nova"
    ICE_BLAST = "Ice Blast"
    SHIVER_ARMOR = "Shiver Armor"
    GLACIAL_SPIKE = "Glacial Spike"
    BLIZZARD = "Blizzard"
    CHILLING_ARMOR = "Chilling Armor"
    FROZEN_ORB = "Frozen Orb"

    # Druid:
    # Summoning:
    RAVEN = "Raven"
    POISON_CREEPER = "Poison Creeper"
    OAK_SAGE = "Oak Sage"
    SUMMON_SPIRIT_WOLF = "Summon Spirit Wolf"
    CARRION_VINE = "Carrion Vine"
    HEART_OF_WOLVERINE = "Heart of Wolverine"
    SUMMON_DIRE_WOLF = "Summon Dire Wolf"
    SOLAR_CREEPER = "Solar Creeper"
    SPIRIT_OF_BARBS = "Spirit of Barbs"
    SUMMON_GRIZZLY = "Summon Grizzly"

    # Shape Shifting
    WEREWOLF = "Werewolf"
    WEREBEAR = "Werebear"
    FERAL_RAGE = "Feral Rage"
    MAUL = "Maul"
    RABIES = "Rabies"
    FIRE_CLAWS = "Fire Claws"
    HUNGER = "Hunger"
    SHOCK_WAVE = "Shock Wave"
    FURY = "Fury"

    # Elemental:
    FIRESTORM = "Firestorm"
    MOLTEN_BOULDER = "Molten Boulder"
    ARCTIC_BLAST = "Arctic Blast"
    FISSURE = "Fissure"
    CYCLONE_ARMOR = "Cyclone Armor"
    TWISTER = "Twister"
    VOLCANO = "Volcano"
    TORNADO = "Tornado"
    ARMAGEDDON = "Armageddon"
    HURRICANE = "Hurricane"

    # Paladin:
    # Combat Skills:
    SACRIFICE = "Sacrifice"
    SMITE = "Smite"
    HOLY_BOLT = "Holy Bolt"
    ZEAL = "Zeal"
    CHARGE = "Charge"
    VENGEANCE = "Vengeance"
    BLESSED_HAMMER = "Blessed Hammer"
    CONVERSION = "Conversion"
    HOLY_SHIELD = "Holy Shield"
    FIST_OF_THE_HEAVENS = "Fist of the Heavens"

    # Offensive Auras:
    MIGHT = "Might"
    HOLY_FIRE = "Holy Fire"
    THORNS = "Thorns"
    BLESSED_AIM = "Blessed Aim"
    CONCENTRATION = "Concentration"
    HOLY_FREEZE = "Holy Freeze"
    HOLY_SHOCK = "Holy Shock"
    SANCTUARY = "Sanctuary"
    FANATICISM = "Fanaticism"
    CONVICTION = "Conviction"

    # Defensive Auras:
    PRAYER = "Prayer"
    RESIST_FIRE = "Resist Fire"
    DEFIANCE = "Defiance"
    RESIST_COLD = "Resist Cold"
    CLEANSING = "Cleansing"
    RESIST_LIGHTNING = "Resist Lightning"
    VIGOR = "Vigor"
    MEDITATION = "Meditation"
    REDEMPTION = "Redemption"
    SALVATION = "Salvation"

    # Barbarian:
    # Combat Skills:
    BASH = "Bash"
    LEAP = "Leap"
    DOUBLE_SWING = "Double Swing"
    STUN = "Stun"
    DOUBLE_THROW = "Double Throw"
    LEAP_ATTACK = "Leap Attack"
    CONCENTRATE = "Concentrate"
    FRENZY = "Frenzy"
    WHIRLWIND = "Whirlwind"
    BERSERK = "Berserk"

    # Combat Masteries:

    # Warcries:
    HOWL = "Howl"
    FIND_POTION = "Find Potion"
    TAUNT = "Taunt"
    SHOUT = "Shout"
    FIND_ITEM = "Find Item"
    BATTLE_CRY = "Battle Cry"
    BATTLE_ORDERS = "Battle Orders"
    GRIM_WARD = "Grim Ward"
    WAR_CRY = "War Cry"
    BATTLE_COMMAND = "Battle Command"

    # Necromancer:
    # Curses:
    AMPLIFY_DAMAGE = "Amplify Damage"
    DIM_VISION = "Dim Vision"
    WEAKEN = "Weaken"
    IRON_MAIDEN = "Iron Maiden"
    TERROR = "Terror"
    CONFUSE = "Confuse"
    LIFE_TAP = "Life Tap"
    ATTRACT = "Attract"
    DECREPIFY = "Decrepify"
    LOWER_RESIST = "Lower Resist"

    # Poison and Bone Spells:
    TEETH = "Teeth"
    BONE_ARMOR = "Bone Armor"
    POISON_DAGGER = "Poison Dagger"
    CORPSE_EXPLOSION = "Corpse Explosion"
    BONE_WALL = "Bone Wall"
    POISON_EXPLOSION = "Poison Explosion"
    BONE_SPEAR = "Bone Spear"
    BONE_PRISON = "Bone Prison"
    POISON_NOVA = "Poison Nova"
    BONE_SPIRIT = "Bone Spirit"

    # Summoning Spells:
    RAISE_SKELETON = "Raise Skeleton"
    CLAY_GOLEM = "Clay Golem"
    RAISE_SKELETAL_MAGE = "Raise Skeletal Mage"
    BLOOD_GOLEM = "Blood Golem"
    IRON_GOLEM = "Iron Golem"
    FIRE_GOLEM = "Fire Golem"
    REVIVE = "Revive"

    # Assassin:
    # Traps:
    FIRE_BLAST = "Fire Blast"
    SHOCK_WEB = "Shock Web"
    BLADE_SENTINEL = "Blade Sentinel"
    CHARGED_BOLT_SENTRY = "Charged Bolt Sentry"
    WAKE_OF_FIRE = "Wake of Fire"
    BLADE_FURY = "Blade Fury"
    LIGHTNING_SENTRY = "Lightning Sentry"
    WAKE_OF_INFERNO = "Wake of Inferno"
    DEATH_SENTRY = "Death Sentry"
    BLADE_SHIELD = "Blade Shield"

    # Shadow Disciplines:
    PSYCHIC_HAMMER = "Psychic Hammer"
    BURST_OF_SPEED = "Burst of Speed"
    CLOAK_OF_SHADOWS = "Cloak of Shadows"
    FADE = "Fade"
    SHADOW_WARRIOR = "Shadow Warrior"
    MIND_BLAST = "Mind Blast"
    VENOM = "Venom"
    SHADOW_MASTER = "Shadow Master"

    # Martial Arts:
    TIGER_STRIKE = "Tiger Strike"
    DRAGON_TALON = "Dragon Talon"
    FISTS_OF_FIRE = "Fists of Fire"
    DRAGON_CLAW = "Dragon Claw"
    COBRA_STRIKE = "Cobra Strike"
    CLAWS_OF_THUNDER = "Claws of Thunder"
    DRAGON_TAIL = "Dragon Tail"
    BLADES_OF_ICE = "Blades of Ice"
    DRAGON_FLIGHT = "Dragon Flight"
    PHOENIX_STRIKE = "Phoenix Strike"

    # Amazon:
    # Bow and Crossbow Skills:
    MAGIC_ARROW = "Magic Arrow"
    FIRE_ARROW = "Fire Arrow"
    COLD_ARROW = "Cold Arrow"
    MULTIPLE_SHOT = "Multiple Shot"
    EXPLODING_ARROW = "Exploding Arrow"
    ICE_ARROW = "Ice Arrow"
    GUIDED_ARROW = "Guided Arrow"
    STRAFE = "Strafe"
    IMMOLATION_ARROW = "Immolation Arrow"
    FREEZING_ARROW = "Freezing Arrow"

    # Passive and Magic Skills:
    INNER_SIGHT = "Inner Sight"
    SLOW_MISSILES = "Slow Missiles"
    DECOY = "Decoy"
    VALKYRIE = "Valkyrie"

    # Javelin and Spear Skills:
    JAB = "Jab"
    POWER_STRIKE = "Power Strike"
    POISON_JAVELIN = "Poison Javelin"
    IMPALE = "Impale"
    LIGHTNING_BOLT = "Lightning Bolt"
    CHARGED_STRIKE = "Charged Strike"
    PLAGUE_JAVELIN = "Plague Javelin"
    FEND = "Fend"
    LIGHTNING_STRIKE = "Lightning Strike"
    LIGHTNING_FURY = "Lightning Fury"

    # Misc
    ATTACK = "Attack"
    THROW = "Throw"
    UNSUMMON = "Unsummon"
    TOME_OF_IDENTIFY = "Tome of Identify"
    TOME_OF_Townportal = "Tome of Townportal"
class GameInterfaceAction(Enum):
    AUTOMAP = "Automap"
    INVENTORY = "Inventory"
    BELT = "Belt"
    ITEMS = "Items"
    SHOW_PORTRAITS = "show_portraits"
    SWAP_WEAPONS = "swap_weapons"
    CHAT = "Chat"
class ACT(Enum):
    ACT1 = "ACT1"
    ACT2 = "ACT2"
    ACT3 = "ACT3"
    ACT4 = "ACT4"
    ACT5 = "ACT5"
class Weaponset(Enum):
    PRIMARY = "Primary"
    SECONDARY = "Secondary"
class CharacterClass(Enum):
    NECROMANCER = "Necromancer"
    SORCERESS = "Sorceress"
    DRUID = "Druid"
    AMAZON = "Amazon"
    PALADIN = "Paladin"
    ASSASSIN = "Assassin"
    BARBARIAN = "Barbarian"
class WayPointActSelectionPoints(Enum):
    ACT1 = Point(75, 115)
    ACT2 = Point(75, 175)
    ACT3 = Point(75, 235)
    ACT4 = Point(75, 295)
    ACT5 = Point(75, 355)
class WaypointZone(Enum):
    ROGUE_ENCAMPMENT = "Rogue Encampment"
    LUT_GHOLEIN = "Lut Gholein"
    KURAST_DOCKS = "Kurast Docks"
    THE_PANDEMONIUM_FORTRESS = "The Pandemonium Fortress"
    HARROGATH = "Harrogath"
    COLD_PLAINS = "Cold Plains"
    SEWERS_LEVEL_2 = "Sewers Level 2"
    SPIDER_FOREST = "Spider Forest"
    CITY_OF_THE_DAMNED = "City of the Damned"
    FRIGID_HIGHLANDS = "Frigid Highlands"
    STONY_FIELD = "Stony Field"
    DRY_HILLS = "Dry Hills"
    GREAT_MARSH = "Great Marsh"
    RIVER_OF_FLAME = "River of Flame"
    ARREAT_PLATEAU = "Arreat Plateau"
    DARK_WOOD = "Dark Wood"
    HALLS_OF_THE_DEAD_LEVEL_2 = "Halls of the Dead Level 2"
    FLAYER_JUNGLE = "Flayer Jungle"
    CRYSTALLINE_PASSAGE = "Crystalline Passage"
    BLACK_MARSH = "Black Marsh"
    FAR_OASIS = "Far Oasis"
    LOWER_KURAST = "Lower Kurast"
    GLACIAL_TRAIL = "Glacial Trail"
    OUTER_CLOISTER = "Outer Cloister"
    LOST_CITY = "Lost City"
    KURAST_BAZAAR = "Kurast Bazaar"
    HALLS_OF_PAIN = "Halls of Pain"
    JAIL_LEVEL_1 = "Jail Level 1"
    PALACE_CELLAR_LEVEL_1 = "Palace Cellar Level 1"
    UPPER_KURAST = "Upper Kurast"
    FROZEN_TUNDRA = "Frozen Tundra"
    INNER_CLOISTER = "Inner Cloister"
    ARCANE_SANCTUARY = "Arcane Sanctuary"
    TRAVINCAL = "Travincal"
    THE_ANCIENTS_WAY = "The Ancients' Way"
    CATACOMBS_LEVEL_2 = "Catacombs Level 2"
    CANYON_OF_THE_MAGI = "Canyon of the Magi"
    DURANCE_OF_HATE_LEVEL_2 = "Durance of Hate Level 2"
    WORLDSTONE_KEEP_LEVEL_2 = "Worldstone Keep Level 2"
class GameVersion(Enum):
    CLASSIC = "Classic"
    RESURRECTED = "Resurrected"
class ItemQuality(Enum):
    GREY = "Grey"
    COMMON = "Common"
    MAGIC = "Magic"
    RARE = "Rare"
    SET = "Set"
    UNIQUE = "Unique"
    RUNE = "Rune"
if __name__ == "__main__":
    difficulty = Difficulty.NORMAL
    print(difficulty == Difficulty.NORMAL)
    print(difficulty.value == "Normal")
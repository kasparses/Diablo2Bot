
# Diablo 2 computer vision bot

## Project goal
Create a bot that can play the video game Diablo 2.  

The bot shall be restricted to only using the input and output that a regular player has access to.  
All input must come from the visuals rendered by the game (by taking screenshots). This means we may not extract information from the memory.  

The bot must be able to:
- Detect and kill monsters  
- Detect loot and pickup loot specified in a loot filter  
- Navigate all zones in the game. To do this we must be able to read the overlayed map and to nagivate based on the map.

## Project status
This project is under development and not yet streamlined for easy use.

## Sample video
Here is a sample video of the bot running my lightning sorceress through cold plains:  
![sorceress_cold_plains](recordings/cold_plains.mp4)

## Setup, installation and use
First download the repository. Then create a profile as follows:
### Creating a profile
In this section I will explain how to create a profile for a character that the bot will use:  
The `profiles` folder contains sub folders for each character that the bot can use:  
`profiles/character1`  
`profiles/character2`  
`profiles/character3` ...

By default the project contains a single profile for my lightning sorceress `Shalan` in `profiles/shalan`  
Copy this folder and rename it to the name of your character such as: `profiles/<character_name>`.  
Open the `profiles/<character_name>/profile.json` file and change the values to fit your Diablo 2 installation and character details.

Below is an explanation of the values to be set in the `profile.json` file:
| Key  | Example value  | Explanation  |
|---|---|---|
| "diablo2_exe_file_path" | "E:\\Programs\\Diablo II\\Diablo II.exe"  | The path to your Diablo 2 executable. Needed to start the game  |
| "character_class"  | "Sorceress"  | Your character's class. Needed to calculate faster cast rate breakpoints  |
| "character_name"  |  "Shalan" |  The name of your character. Needed to select the character in the single player menu |
| "primary_attack_skill" | "Chain Lightning"  | The spell your character will use to kill monsters  |
| "faster_cast_rate_weaponset_primary" | 125 | How much faster cast rate you have with weaponset Ⅰ  |
| "faster_cast_rate_weaponset_secondary"  | 80  | How much faster cast rate you have with weaponset Ⅱ  |
| "left_skill_weaponset_primary"  | "Lightning"  | The skill selected in the left field (next to the health globe) with weaponset Ⅰ. Used to validate weapon switches  |
| "left_skill_weaponset_secondary"  | "Charged Bolt"  | The skill selected in the left field (next to the health globe) with weaponset Ⅱ. Used to validate weapon switches  |
| "zone_to_farm"  | "Cold Plains"  | The name of the zone where we will kill monsters. Must be a zone where the character has the waypoint  |
| "min_gold_to_pickup"  | 1000  | Any gold dropped below this limit will be ignored  |
|  "game_difficulty" | "Hell"  | The difficulty. Can be Normal, Nightmare or Hell  |
| "game_version"  | "Classic"  |  Classic or Resurrected. Ressurected is not yet supported |
| "num_belt_columns_reserved_for_healing_potions"  | 1  | How many of the belt columns we reserve for healing potions  |
| "num_belt_columns_reserved_for_mana_potions"  | 3  | How many of the belt columns we reserve for mana potions  |
| "health_limit"  | 0.75  | When the character's health drops below this limit (75% in this example) we will drink a healing potion  |
| "health_limit_hard"  | 0.3  | When the character's health drops below this limit we will instantly exit the game and start over  |
| "mana_limit"  | 0.3  |  When the character's mana drops below this limit (30% in this example) we will drink a mana potion |
| "players_count"  | 3  | How many players the game will simulate is in the game. Increases difficulty and loot drop rates if above 1  |
| "set_game_options"  | true  | If true the program will set these setings: "Lighting Quality" = "Low", "Automap Size" = "Full Screen", "Fade" = "No" and "Show Party" = "No". If false the program will assumese these settings are set already.  |
| "keybindings"/"skills" | "Chain Lightning": "d"  | The keybinding of the spell "Chain Lightning". |
| "keybindings"/"skills" | "Teleport": "e"  | The keybinding of the spell "Teleport". |
| # Fill out all spell keybindings as above. |  |  |
| "keybindings"/"game_interface_actions" | "Automap": "tab"  | The keybinding to toggle the automap |
| "keybindings"/"game_interface_actions" | "Inventory": "i"  | The keybinding to open the inventory |
| "keybindings"/"game_interface_actions" | "Belt": "æ"  | The keybinding to open the belt |
| "keybindings"/"game_interface_actions" | "Items": "alt"  | The keybinding to show the text of the items laying on the ground |
| "keybindings"/"game_interface_actions" | "show_portraits": "z"  | The keybinding to toggle the mercery/helper portraits |
| "buffs" | "skill": "Battle Command"  | The name of the buff spell. Keybinding for the buff spell must be typed in the "keybindings"/"skills" section    |
| "buffs | "duration": 135  | The duration of the buff. Refreshes the buff after the duration   |
| "only_castable_on_secondary_weaponset" | true  | If the buff comes from a weapon on your character's secondary weaponset set this to true. Then the character will switch weaponsets before casting the buff.   |
| # Fill out other buffs as above |   |    |
|  |   |    |

#### Loot filter
Most of the items in Diablo 2 are not worth picking up. We therefore need a loot filter  
The bot uses the `profiles/<character_name>/item_filter.json` file as loot filter. This file contains a list of all items and if our character should pick them up or leave them on the ground.  
Example:
```
"Shadow Bow": {
        "Grey": false,
        "Common": false,
        "Magic": false,
        "Rare": true,
        "Set": false,
        "Unique": true,
        "Rune": false,
        "row_size": 4,
        "col_size": 2
    }
```
This means that we wish to pickup a rare or unique `Shadow Bow`, but we will leave a `Shadow Bow` in any other quality.  
The `item_filter.json` file can be edited manually but as it can be tedious to type in your preference for each item we can instead generate the item filter:  
Open the `profiles/<character_name>/loot_profile_creator.json` file.  
Fill out what quality you want to pickup for each item type,  
Example:  
```
"Helms":    {"Normal": 		["Unique"],
             "Exceptional": ["Unique"],
             "Elite": 		["Set", "Unique"]},
	
"Armor":    {"Normal": 		[],
             "Exceptional": ["Unique"],
             "Elite": 		["Set", "Grey", "Unique"]},
```
Items in Diablo 2 have three quality levels: normal, exceptional and elite.  
In the "Helms" category we, for example, have "Cap", "War Hat", "Shako" as normal, exceptional and elite helms. For the full list of items see the `initialization_data/items/item_categories_names_sizes.json` file.  
In the above example we specify that we want to pickup helms in the normal category if the are unique and we wish to pickup helms in the elite category if they are unique or set.  
After filling out the `loot_profile_creator.json` file generate the new `item_filter.json` file by opening a terminal/command prompt at the root of the project directory and running the `create_item_filter.py` script like this: `python .\create_item_filter.py <profile_name>`. Example: `python .\create_item_filter.py shalan`. This will generate a new loot filter at `profiles/<character_name>/item_filter.json`  

## Character preparation
After filling out the profile information we must prepare the character before we can run it:  

### Weaponsets
Diablo 2 starts the game with the last active weaponset.  
The last active weaponset of your character MUST be weaponset Ⅰ.

### Inventory and stash
The bot will pickup items to its inventory until it is full.  
It will then move the items to the stash.  
It will continue this cycle until both its inventory and stash are full upon which the bot will exit the game and stop the program.
Items that are in the inventory when the bot starts will be assumed to be charms and will NOT be moved to the stash when the inventory is full. Ensure that the character only has essential items in the inventory before starting the bot.

## Run bot
After creating the profile and preparing the character we can start the bot as follows:  
Open a terminal/command prompt at the root of the project directory.  
Run this command: `python .\run.py <profile_name>` Example: `python .\run.py shalan`  
The bot can be terminated by pressing the `ctrl` key. 

## Limitations
Currently the bot only supports spell attacks.

## TODO
- Manage library requirements and installation.
- Add support for melee and bow attacks.
- Add support for Diablo 2 Resurrected
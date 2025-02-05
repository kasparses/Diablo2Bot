# Diablo 2 computer vision bot

## Project goal
Create a computer vision bot that can play Diablo 2.  

The bot shall be restricted to only using the input and output that a regular player has access to.  
All input must come from the visuals rendered by the game (by taking screenshots). This means we may not extract information from the memory.  

The bot must be able to:
* Detect and kill monsters  
* Pickup loot specified in a loot filter  
* Navigate all zones in the game
  * To do this we must be able to read the overlayed map and nagivate based on it

## Sample video
Here is a sample video of the bot running my lightning sorceress through `Cold Plains`:  
  
![sorceress_cold_plains](recordings/cold_plains.mp4)

## Setup

### Settings
Fill out these files:  
* `settings/system_settings.toml`
  * This file contains various system settings
* `settings/bot_settings.toml`
  * This file contains general settings that affects how the program should run
* `settings/profiles/sorceress.toml`
  * This file is the profile for my lightning sorceress
  * Rename this file and change its values to fit your character
  * You may create as many profiles as you like
* `settings/item_filters/default.toml`
  * This is the default loot filter
  * You can alter this loot filter or create your own
  * You may create as many loot filters as you like
  * Each profile specifies which loot filter to use

### Character preparation
Prepare your character before running the program:  

#### Weaponsets
Diablo 2 starts the game with the last active weaponset.  
The last active weaponset of your character MUST be weaponset Ⅰ.

#### Inventory and stash
The bot will pickup items to its inventory until it is full.  
It will then move the items to the stash.  
It will continue this cycle until both its inventory and stash are full.  
Items that are in the inventory when the bot starts will be assumed to be charms and will NOT be moved to the stash when the inventory is full.  
Ensure that your character only has essential items in its inventory before starting the program.

## Run bot
Start the program as follows:  
* Open a terminal/command prompt at the root of the project directory  
* Run this command: `./diablo2bot.exe <profile_name>`  
  * Example: `./diablo2bot.exe sorceress`  
* During development run this command: `cargo run --release <profile_name>`  
  * Example: `cargo run --release sorceress`
* The program can be stopped by manually moving the mouse around for approximately a second

## Dependencies

### Windows and Linux
`Diablo 2 Lord of Destuction v1.14b` must be installed on your system.  
Earlier versions of Diablo 2 may function but this is not guaranteed.  

### Linux
`xdotool`, `wmctrl` and `wine` must be installed.  

`xdotool` and `wmctrl` can be installered with the following commands:  
> sudo apt-get install xdotool  
> sudo apt-get install wmctrl  

## Build and package
Running the `builder` program will build the application and move all necessary files to a `Diablo2bot` folder:  
> cargo run -p builder  

## Limitations
Currently the bot only supports spell attacks.

## TODO
* Add support for melee and bow attacks
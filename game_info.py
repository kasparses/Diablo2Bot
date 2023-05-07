# -*- coding: utf-8 -*-
import ctypes
import json
import os
from ctypes import c_void_p

import cv2
import numpy as np

from dataclasses_diablo import Box, MapSpriteData, Point
from enums_diablo import (ACT, CharacterClass, Difficulty, GameInterfaceAction,
                          GameVersion, Skill, WaypointZone)
from utils_cells import get_cell_information
from utils_data import (acts_waypointzones, acts_zones,
                        items_that_can_be_picked_up_with_telekinesis,
                        waypoint_zones_monster_mapping)
from utils_data_loading import (load_fixed_routes,
                                load_validation_pixels_files,
                                read_level_sprites_table)
from utils_data_transformations import (flatten_sprite_indices,
                                        flatten_sprite_values,
                                        get_modulo_indices,
                                        get_sprite_indices_lengths)
from utils_diablo_info import calculate_cast_frames, calculate_cooldown_frames
from utils_misc import (create_validation_pixels, get_enum,
                        get_transformation_arrays, get_window_offsets,
                        load_font_data, pad_file_name, transform_image)
from utils_validations import (validate_profile_belt_reservations,
                               validate_profile_bool, validate_profile_integer,
                               validate_profile_percentage_float,
                               validate_profile_zone_to_farm)


class GameInfo:
    '''
    This class contains all static variables that will be used by the program.
    The idea is to have a single repository containing all static global variables.
    '''
    
    def __init__(self,
                profile: dict,
                _set_miscellaneous_values: bool = True,
                _set_profile_values: bool = True,
                _set_configuration_values: bool = True,
                _set_items_that_can_be_picked_up_with_telekinesis: bool = True,
                _set_locations: bool = True,
                _set_cells: bool = True,
                _set_consumable_items: bool = True,
                _set_vk_codes: bool = True,
                _set_zones: bool = True,
                _set_waypoints: bool = True,
                _set_fixed_routes: bool = True,
                _set_c_functions: bool = True,
                _set_monster_detection: bool = True,
                _set_validation_pixels: bool = True,
                _set_map_data: bool = True,
                _set_font_data: bool = True,
                _set_default_key_bindings: bool = True,
                _set_potion_values: bool = True,
                _set_speeds: bool = True,
                **kwargs
                ) -> None:

        if _set_miscellaneous_values: self.set_miscellaneous_values()
        if _set_profile_values: self.set_profile_values(profile)
        if _set_configuration_values: self.set_configuration_values()
        if _set_items_that_can_be_picked_up_with_telekinesis: self.set_items_that_can_be_picked_up_with_telekinesis()
        if _set_locations: self.set_locations()
        if _set_cells: self.set_cells()
        if _set_consumable_items: self.set_consumable_items()
        if _set_vk_codes: self.set_vk_codes()
        if _set_zones: self.set_zones()
        if _set_waypoints: self.set_waypoints()
        if _set_fixed_routes: self.set_fixed_routes()
        if _set_c_functions: self.set_c_functions()
        if _set_monster_detection: self.set_monster_detection()
        if _set_validation_pixels: self.set_validation_pixels()
        if _set_map_data: self.set_map_data()
        if _set_font_data: self.set_font_data()
        if _set_default_key_bindings: self.set_default_key_bindings()
        if _set_potion_values: self.set_potion_values()
        if _set_speeds: self.set_speeds()

    def set_map_data(self) -> None:
        self.set_zone_mapping()
        self.set_map_sprites()

        self.square_roots = np.sqrt(np.arange(31))

    def set_configuration_values(self) -> None:
        self.monster_detection_confidence_threshold: float = 0.4
        self.step_limit: int = 370
        self.movement_wide_start_size: int = 5
        self.start_position = Point(286, 408)
    def set_miscellaneous_values(self) -> None:
        self.root_folder: str = os.path.dirname(os.path.abspath(__file__))
        
        self.seconds_per_frame: float = 0.04 # Diablo 2 updates the screen at 25 fps. 1000 / 25 = 40
        self.healing_potion_fill_time: float = 7.0
        self.mana_potion_fill_time: float = 5.0

        self.folders_last_file_ids: dict = {}
        self.game_window_screen_location: dict[str, int] = None # We set this when we first start the game

        # Id of the window handle for the game window. This can be used with various functions in the win32gui library.
        self.game_window_handle = None
    def set_potion_values(self) -> None:
        # Healing and mana potions give different points depending on the class.
        # Here we set the value depending on the class we are playing as.
        #http://classic.battle.net/diablo2exp/items/potions.shtml
        self.healing_potions: dict[str, int] = {"Minor Healing Potion": None, "Light Healing Potion": None, "Healing Potion": None, "Greater Healing Potion": None, "Super Healing Potion": None}
        self.mana_potions: dict[str, int] = {"Minor Mana Potion": None, "Light Mana Potion": None, "Mana Potion": None, "Greater Mana Potion": None, "Super Mana Potion": None}
        self.rejuvenation_potions: set = {'Rejuvenation Potion', 'Full Rejuvenation Potion'}

        healing_potion_points = ((30,60,100,180,320),
                                 (45,90,150,270,480),
                                 (60,120,200,360,640))
        mana_potion_points = ((40,80,160,300,500),
                              (30,60,120,225,375),
                              (20,40,80,150,250))
        classes = ((CharacterClass.NECROMANCER, CharacterClass.SORCERESS, CharacterClass.DRUID),
                   (CharacterClass.AMAZON, CharacterClass.PALADIN, CharacterClass.ASSASSIN),
                   [CharacterClass.BARBARIAN])
        for (minor_healing_potion, light_healing_potion, healing_potion, greater_healing_potion, super_healing_potion), (minor_mana_potion, light_mana_potion, mana_potion, greater_mana_potion, super_mana_potion), _classes in zip(healing_potion_points, mana_potion_points, classes):
            if self.character_class in _classes:
                self.healing_potions['Minor Healing Potion'] = minor_healing_potion
                self.healing_potions['Light Healing Potion'] = light_healing_potion
                self.healing_potions['Healing Potion'] = healing_potion
                self.healing_potions['Greater Healing Potion'] = greater_healing_potion
                self.healing_potions['Super Healing Potion'] = super_healing_potion
                
                self.mana_potions['Minor Mana Potion'] = minor_mana_potion
                self.mana_potions['Light Mana Potion'] = light_mana_potion
                self.mana_potions['Mana Potion'] = mana_potion
                self.mana_potions['Greater Mana Potion'] = greater_mana_potion
                self.mana_potions['Super Mana Potion'] = super_mana_potion
                break
    def set_speeds(self) -> None:
        self.cooldown_seconds_primary: float = calculate_cooldown_frames(self.character_class, self.faster_cast_rate_weaponset_primary) * self.seconds_per_frame
        self.cooldown_seconds_secondary: float = calculate_cooldown_frames(self.character_class, self.faster_cast_rate_weaponset_secondary) * self.seconds_per_frame
        if self.character_class == CharacterClass.SORCERESS:
            self.cooldown_seconds_primary_lightning: float = calculate_cooldown_frames(self.character_class, self.faster_cast_rate_weaponset_primary, calculate_lightning_cooldown=True) * self.seconds_per_frame
            self.cooldown_seconds_secondary_lightning: float = calculate_cooldown_frames(self.character_class, self.faster_cast_rate_weaponset_secondary, calculate_lightning_cooldown=True) * self.seconds_per_frame

        self.cast_seconds_primary: float = calculate_cast_frames(self.character_class, self.faster_cast_rate_weaponset_primary) * self.seconds_per_frame
        self.cast_seconds_secondary: float = calculate_cast_frames(self.character_class, self.faster_cast_rate_weaponset_secondary) * self.seconds_per_frame
    def set_profile_values(self, profile: dict) -> None:

        enums = {
            "character_class": CharacterClass,
            "primary_attack_skill": Skill,
            "zone_to_farm": WaypointZone,
            "game_difficulty": Difficulty,
            "game_version": GameVersion
            }

        num_errors = 0

        for key, value in profile.items():

            if key in enums:
                value = get_enum(enums[key], value, key)
                if key == "zone_to_farm":
                    num_errors += validate_profile_zone_to_farm(value)
            elif key == "keybindings":
                keybindings = {}
                for skill, keybinding in value["skills"].items():
                    keybindings[get_enum(Skill, skill, "skill")] = keybinding

                for interface_action, keybinding in value["game_interface_actions"].items():
                    keybindings[get_enum(GameInterfaceAction, interface_action, "game_interface_action")] = keybinding
                
                for action, keybinding in value["miscellaneous"].items():
                    keybindings[action] = keybinding
                    
                value = keybindings
            elif key in {
                "faster_cast_rate_weaponset_primary",
                "faster_cast_rate_weaponset_secondary",
                "min_gold_to_pickup",
                "players_count"
            }:
                num_errors += validate_profile_integer(value, key)
            elif key in {
                "num_belt_columns_reserved_for_healing_potions",
                "num_belt_columns_reserved_for_mana_potions"
            }:
                num_errors += validate_profile_integer(value, key)
                num_errors += validate_profile_belt_reservations(value, key)
            elif key in {
                "health_limit",
                "health_limit_hard",
                "mana_limit"
            }:
                num_errors += validate_profile_percentage_float(value, key)
            elif key in {
                "set_game_options"
            }:
                num_errors += validate_profile_bool(value, key)

            setattr(self, key, value)

        if num_errors:
            print("Exiting program!")
            import sys
            sys.exit()

        self.load_item_filter()
    def load_item_filter(self) -> None:
        with open(os.path.join(self.root_folder, "profiles", self.profile_name, "item_filter.json")) as file:
            self.item_filter: dict = json.load(file)
    def set_items_that_can_be_picked_up_with_telekinesis(self) -> None:
        self.items_that_can_be_picked_up_with_telekinesis: list[str] = items_that_can_be_picked_up_with_telekinesis
    def set_locations(self) -> None:
        self.inventory_top_left_point = Point(315, 417)
        self.stash_top_left_point = Point(143, 153)
        self.belt_top_left_point = Point(466, 421)
        self.merchant_top_left_point = Point(123, 94)
        
        self.safe_mouse_position = Point(590, 790)

        self.life_text_screen_location = Box(0, 50, 492, 506)
        self.mana_text_screen_location = Box(635, 730, 492, 506)

        self.life_numbers_screen_location = Box(20, 140, 492, 506)
        self.mana_numbers_screen_location = Box(700, 799, 492, 506)

        self.skill_bar_screen_location = Box(0, 800, 553, 600)
        self.mini_panel_screen_location = Box(342, 456, 547, 553)

        self.health_globe_screen_location = Box(0, 109, 508, 600)
        self.mana_globe_screen_location = Box(695, 800, 508, 600)

        self.waypoint_text_area = Box(80, 400, 90, 440)

    def set_cells(self) -> None:
        self.cells_empty_inventory = get_cell_information(
            cv2.imread(os.path.join(self.root_folder, "initialization_data", "cells_empty_inventory.png")),
            4,
            10,
            self.inventory_top_left_point
            )
        
        self.cells_empty_stash = get_cell_information(
            cv2.imread(os.path.join(self.root_folder, "initialization_data", "cells_empty_stash.png")),
            8,
            6,
            self.stash_top_left_point
            )

    def set_consumable_items(self) -> None:
        # These are items that can be consumed and kept in the belt.
        dict_of_consumable_items = {}
        for file_name in os.listdir(os.path.join(self.root_folder, "initialization_data", "consumable_items")):
            file_name_without_extension = os.path.splitext(file_name)[0]
            dict_of_consumable_items[file_name_without_extension] = cv2.imread(os.path.join(self.root_folder, "initialization_data", "consumable_items", file_name))
        list_of_consumable_items = list(dict_of_consumable_items.values())
        self.consumable_items_position_names = {}
        for i, name in enumerate(dict_of_consumable_items):
            self.consumable_items_position_names[i] = name

        self.consumable_items: np.ndarray = np.vstack(list_of_consumable_items).reshape(len(list_of_consumable_items),24,24,3)

        # The background of consumables is either black or blue
        # Here we only check for pixels that is part of the item and not the background.
        consumable_items_is_not_background_color = ((self.consumable_items[:,:,:,0] !=  8) | (self.consumable_items[:,:,:,1] !=  8) | (self.consumable_items[:,:,:,2] != 8)) & \
                                                   ((self.consumable_items[:,:,:,0] != 40) | (self.consumable_items[:,:,:,1] != 12) | (self.consumable_items[:,:,:,2] != 12))
        
        self.consumable_items_indices_list = [] # List of all consumable items and the indices of the items excluding the background colors
        self.consumable_items_values_list = []
        
        for i in range(self.consumable_items.shape[0]):
            consumable_items_indices = np.where(consumable_items_is_not_background_color[i]) # TODO specify all axis like [i,:,:,:]
            self.consumable_items_indices_list.append(consumable_items_indices)
            self.consumable_items_values_list.append(self.consumable_items[i][consumable_items_indices])
    def set_vk_codes(self) -> None:
        with open(os.path.join(self.root_folder, "initialization_data", "vk_codes.json"), encoding='utf-8') as file:
            self.vk_codes: dict = json.load(file)
    def set_zones(self) -> None:           
        self.zones_acts = {}
        for act, zones in acts_zones.items():
            for zone in zones:
                self.zones_acts[zone] = act

        self.zones = set(
            acts_zones[ACT.ACT1] +
            acts_zones[ACT.ACT2] +
            acts_zones[ACT.ACT3] +
            acts_zones[ACT.ACT4] +
            acts_zones[ACT.ACT5]
            )

    def set_waypoints(self) -> None:
        self.acts_waypointzones = acts_waypointzones
    def set_fixed_routes(self) -> None:
        # Read files containing fixed routes         
        self.fixed_routes = load_fixed_routes(self.root_folder)
    def set_zone_mapping(self) -> None:
        with open(os.path.join(self.root_folder, "initialization_data", "zone_mapping.json"), encoding='utf-8') as file:
            self.zone_mapping: dict[str, str] = json.load(file)
    def set_validation_pixels(self)  -> None:
        self.validation_pixels = load_validation_pixels_files(os.path.join(self.root_folder, "initialization_data", "validation_pixels"))

        self.validation_pixels["primary_weapons"] = create_validation_pixels(cv2.imread(os.path.join(self.root_folder, "initialization_data", "skills", f"{self.left_skill_weaponset_primary}.png")), 553, 117)
        self.validation_pixels["secondary_weapons"] = create_validation_pixels(cv2.imread(os.path.join(self.root_folder, "initialization_data", "skills", f"{self.left_skill_weaponset_secondary}.png")), 553, 117)
    def set_font_data(self)  -> None:
        self.font_sprite_data = load_font_data(self.root_folder)

        self.window_offsets_all = get_window_offsets(
            self.c_functions,
            0,
            600,
            0,
            800 + self.font_sprite_data.symbol_col_size,
            self.font_sprite_data.symbol_row_size,
            self.font_sprite_data.symbol_col_size
            )

        self.window_offsets_life_mana = get_window_offsets(
            self.c_functions,
            490,
            490 + self.font_sprite_data.symbol_row_size,
            0,
            800 + self.font_sprite_data.symbol_col_size,
            self.font_sprite_data.symbol_row_size,
            self.font_sprite_data.symbol_col_size
            )

        self.window_offsets_zone_name = get_window_offsets(
            self.c_functions,
            9,
            9 + self.font_sprite_data.symbol_row_size,
            0,
            800 + self.font_sprite_data.symbol_col_size,
            self.font_sprite_data.symbol_row_size,
            self.font_sprite_data.symbol_col_size
            )


    def set_c_functions(self) -> None:
        self.c_functions: ctypes.CDLL = ctypes.CDLL(os.path.join(self.root_folder, "c_functions.so"))
    def set_monster_detection(self) -> None:
        self.monster_detection: ctypes.CDLL = ctypes.CDLL(os.path.join(self.root_folder, "monster_detection.so"))

        act_number = 0
        for i, waypoint_zones in enumerate(self.acts_waypointzones.values()):
            if self.zone_to_farm in waypoint_zones:
                act_number = i + 1
                break

        monster_key = waypoint_zones_monster_mapping[self.zone_to_farm].encode('utf-8')
        self.monster_detection._set_data(act_number, ctypes.c_char_p(self.root_folder.encode('utf-8')), ctypes.c_char_p(monster_key))


    def set_map_sprites(self)  -> None:
        num_total_sprites = len(os.listdir(os.path.join(self.root_folder, "initialization_data", "map_sprites", "input")))
        level_sprites = read_level_sprites_table(os.path.join(self.root_folder, "initialization_data", "level_sprites.csv"), num_total_sprites)

        self.transformation_array, self.transformation_array_back = get_transformation_arrays(self.root_folder, "ACT1")

        self.window_offsets = np.empty(445312, dtype=np.uint32)
        self.c_functions.get_window_offsets(c_void_p(self.window_offsets.ctypes.data))

        self.middle_indices = np.empty(1225, dtype=np.int32)
        self.c_functions.get_middle_indices(c_void_p(self.middle_indices.ctypes.data))

        self.window_offsets_zero = self.window_offsets[0: 13916]

        level_to_farm = self.zone_mapping[self.zone_to_farm.value]
        self.area_mapping = {}
        for level_name, (sprite_ids, sprite_ids_to_ignore) in level_sprites.items():

            if level_name == level_to_farm: # Only get data for the specific area we are going to farm
                # Get file_paths to images
                images_input_file_paths = [os.path.join(self.root_folder, "initialization_data", "map_sprites", "input", pad_file_name(str(sprite_id) + ".png", 4)) for sprite_id in sprite_ids]
                images_output_file_paths = [os.path.join(self.root_folder, "initialization_data", "map_sprites", "output", pad_file_name(str(sprite_id) + ".png", 4)) for sprite_id in sprite_ids]

                # Load and transform images
                images_input = [transform_image(cv2.imread(file_path), self.transformation_array) for file_path in images_input_file_paths]
                images_output = [transform_image(cv2.imread(file_path), self.transformation_array) for file_path in images_output_file_paths]

                # Get indices of non-zero pixels
                images_indices_input = [np.where((image != 0)) for image in images_input]
                images_indices_output = [np.where((image != 0)) for image in images_output]

                # Get indices of the walkable pixels (bridges for example)
                images_indices_walkable = [np.where((image_input != 0) & (image_output == 0)) for (image_input, image_output) in zip(images_input, images_output)]

                # Get image values
                images_values_input = [image[row_indices, col_indices] for image, (row_indices, col_indices) in zip(images_input, images_indices_input)]

                # Get overlap modulus array
                modulus = np.zeros((256,4,8), dtype=np.uint32)
                for values, (row_indices, col_indices) in zip(images_values_input, images_indices_input):
                    modulus[values, row_indices % 4, col_indices % 8] = 1
                modulus[151, :, :] = 1 # Add character map-circle color
                modulus[203, :, :] = 1 # Add merc map-circle color
                modulus = modulus.reshape((256*4*8))
                
                num_sprites = len(images_input)
                
                sprite_indices_length_counter_input = get_sprite_indices_lengths(images_indices_input, sprite_ids_to_ignore)
                sprite_indices_length_counter_output = get_sprite_indices_lengths(images_indices_output, sprite_ids_to_ignore)

                sprite_indices_length_counter_walkable = get_sprite_indices_lengths(images_indices_walkable, sprite_ids_to_ignore)
                sprite_indices_walkable = flatten_sprite_indices(images_indices_walkable, sprite_ids_to_ignore)

                sprite_indices_input = flatten_sprite_indices(images_indices_input, sprite_ids_to_ignore)
                sprite_indices_output = flatten_sprite_indices(images_indices_output, sprite_ids_to_ignore)

                sprite_values_input = flatten_sprite_values(images_values_input, sprite_ids_to_ignore)

                modulo_indices = get_modulo_indices(images_indices_input, sprite_ids_to_ignore)
                
                self.area_mapping[level_name] = MapSpriteData(
                    num_sprites,
                    sprite_indices_input,
                    sprite_values_input,
                    sprite_indices_length_counter_input,
                    sprite_indices_output,
                    sprite_indices_length_counter_output,
                    sprite_indices_length_counter_walkable,
                    sprite_indices_walkable,
                    modulus,
                    modulo_indices
                )
                    
    def set_default_key_bindings(self) -> None:
        """If these keys are not specified then we assume that it the default values.
        """
        for interface_action, default_keybinding in zip((GameInterfaceAction.AUTOMAP, GameInterfaceAction.ITEMS, GameInterfaceAction.BELT, GameInterfaceAction.INVENTORY),
                                                        ("tab",                       "alt",                     "æ",                      "i")):
            if interface_action not in self.keybindings:
                self.keybindings[interface_action] = default_keybinding 
    

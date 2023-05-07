# -*- coding: utf-8 -*-

import os
import sys
import time
import traceback
from typing import Callable, Union

import cv2
import numpy as np
import pyautogui
import win32api
import win32con
import win32gui
from mss import mss  # https://python-mss.readthedocs.io/examples.html

from dataclasses_diablo import (Automap, Box, Health_Mana, Item, Point,
                                TotalMovement, ValidationPixel)
from diablo_exceptions import (GameWindowActivationError, ValidationError,
                               WaypointLocationError)
from enums_diablo import (ACT, ClickType, Difficulty, GameInterfaceAction,
                          GameVersion, ItemQuality, Skill, Weaponset)
from game_info import GameInfo
from game_state import GameState
from utils_cells import (are_cells_filled, find_item_placement_in_cell_area,
                         get_cell_information, get_item_size,
                         search_cells_for_items)
from utils_misc import (_move_mouse, adjust_point, adjust_total_movement,
                        detect_monsters, draw_image, fill_maze,
                        find_waypoint_on_map, get_best_potion,
                        get_diamond_indices, get_diff_count,
                        get_diff_percentage, get_diffs, get_distances_to_edge,
                        get_draw_base_point, get_font_symbols,
                        get_full_screenshot, get_game_window_location,
                        get_isolated_map, get_middle_of_box,
                        get_middle_point_of_best_matching_template,
                        get_num_belt_rows, get_optimal_potion_belt_column,
                        get_pad_stats, get_path_coordinates,
                        get_path_diagonally, get_previous_base_point,
                        get_target_position, get_waypoint_zone_act,
                        get_zone_waypoint_act_point, is_in_zone_act,
                        is_item_in_items, is_point_in_area, pad_id, pad_image,
                        remove_areas_from_image,
                        remove_prefixes_from_item_name, transform_image,
                        transform_image_from_resurrected_to_classic,
                        update_belt_after_drinking_potion, validate_page)


class Functions:
    def __init__(self, game_info: GameInfo, game_state: GameState) -> None:
        self.game_info = game_info
        self.game_state = game_state
    def press_key(self, keybinding: str) -> None:
        self.activate_game_window()
        win32api.keybd_event(self.game_info.vk_codes[keybinding], 0,0,0)
        win32api.keybd_event(self.game_info.vk_codes[keybinding], 0,win32con.KEYEVENTF_KEYUP,0)
    def hold_key(self, keybinding: str) -> None:
        self.activate_game_window()
        if keybinding not in self.game_state.held_keys:
            self.game_state.held_keys.add(keybinding)
            win32api.keybd_event(self.game_info.vk_codes[keybinding], 0,0,0)
            time.sleep(self.game_info.seconds_per_frame)
    def release_key(self, keybinding: str) -> None:
        if self.game_state.is_activating_game_window is False:
            self.activate_game_window()
        self.game_state.held_keys.discard(keybinding)
        win32api.keybd_event(self.game_info.vk_codes[keybinding], 0, win32con.KEYEVENTF_KEYUP, 0)
    def click(
        self,
        point: Point,
        click_type: ClickType = ClickType.LEFT,
        sleep_after_cursor_movement: bool=False,
        sleep_after_click: bool=False
        ) -> None:
        
        self.activate_game_window()
        self.move_mouse(point)
        # Sometimes I find that we need to wait after moving the mouse before we click.
        # If we for example are to open the waypoint, then we must wait for the game to register that the mouse is hovering over the waypoint before we click it.
        # If the game has not registered the mouse movement yet, then we simply walk to waypoint instead of activating it.
        if sleep_after_cursor_movement:
            time.sleep(self.game_info.seconds_per_frame*2)
            
        if click_type == ClickType.LEFT:
            win32api.mouse_event(win32con.MOUSEEVENTF_LEFTDOWN, 0, 0)
            win32api.mouse_event(win32con.MOUSEEVENTF_LEFTUP, 0, 0)
        elif click_type == ClickType.RIGHT:
            win32api.mouse_event(win32con.MOUSEEVENTF_RIGHTDOWN, 0, 0)
            win32api.mouse_event(win32con.MOUSEEVENTF_RIGHTUP, 0, 0)

        if sleep_after_click:
            time.sleep(self.game_info.seconds_per_frame)
    def move_mouse(
        self,
        point: Point,
        sleep_after_movement: bool = False
        ) -> None:
        
        self.activate_game_window()
        self.game_state.mouse_point = point

        # Ensure that we do not click outside the window
        # row = min(600 - 1, point.row)
        row = max(min(600 - 1, point.row), 0)
        col = max(min(800 - 1, point.col), 0)

        # Since the game window in Diablo2 Resurrected is twice as large as in Diablo2 Classic we need to double our col and row
        if self.game_info.game_version == GameVersion.RESURRECTED:
            col = col * 2
            row = row * 2

        game_window_left_boundary = self.game_info.game_window_screen_location['left']
        game_window_top_boundary = self.game_info.game_window_screen_location['top']
        col = int(game_window_left_boundary + col)
        row = int(game_window_top_boundary + row)
        _move_mouse(Point(row, col))

        if sleep_after_movement:
            time.sleep(self.game_info.seconds_per_frame)
    def click_on_potion_seller(self) -> None:
        if self.game_state.current_act is None:
            self.update_current_act_and_zone()

        for act, potion_seller_name in zip(ACT, ("akara", "lysander", "ormus", "jamella", "malah")):
            if act == self.game_state.current_act:
                folder = os.path.join(self.game_info.root_folder, "initialization_data", "npcs", potion_seller_name)

        templates = []
        for file in os.listdir(folder):
            file_path = os.path.join(folder, file)
            templates.append(cv2.imread(file_path))

        screenshot = self.take_screenshot()
        potion_seller_middle_point = get_middle_point_of_best_matching_template(screenshot, templates)
        if potion_seller_middle_point.row is None:
            cv2.imwrite("Temp.png", screenshot)

        self.click(potion_seller_middle_point, sleep_after_cursor_movement=True)
        time.sleep(1)

        # Akara, Lysander, Ormus and Malah has the menus Talk, Trade, Cancel.
        # Jamella only has the Trade and Cancel.
        # Therefore we need step down one time with Akara, Lysander, Ormus and Malah, but not with Jamella.
        if self.game_state.current_act != ACT.ACT4:
            self.press_key('down_arrow')
            time.sleep(0.1)
        self.press_key('enter') # Open the trade window.
        time.sleep(0.1)
    def walk_to_potion_seller(self) -> None:
        if self.game_state.current_act is None:
            self.update_current_act_and_zone()

        for act, route_name in zip(ACT, ("act_1_start_to_potion_seller", "act_2_start_to_potion_seller", "act_3_start_to_potion_seller", "act_4_start_to_potion_seller", "act_5_start_to_potion_seller")):
            if act == self.game_state.current_act:
                route = self.game_info.fixed_routes[route_name]

        self.walk_route(route, wait_on_last_click=False)
    def buy_potions_from_merchant(self) -> None:
        cells = get_cell_information(
            self.take_screenshot(),
            10,
            10,
            self.game_info.merchant_top_left_point
            )

        cells = cells[:,:,0:-3,3:,:]
        merchant_items = search_cells_for_items(cells, self.game_info.consumable_items_indices_list, self.game_info.consumable_items_values_list, self.game_info.consumable_items_position_names)
        
        best_mana_potion = get_best_potion(merchant_items, "mana")
        best_healing_potion = get_best_potion(merchant_items, "healing")

        for potion_to_buy in (best_mana_potion, best_healing_potion):
            while True:
                potion_row, potion_col = self.get_potion_belt_position(potion_to_buy.name)
                if potion_row == -1:
                    break

                item_screen_point = Point(
                    self.game_info.merchant_top_left_point.row - 14 + (29 * (potion_to_buy.point.row + 1)),
                    self.game_info.merchant_top_left_point.col - 14 + (29 * (potion_to_buy.point.col + 1))
                )
                self.click(item_screen_point, click_type = ClickType.RIGHT, sleep_after_cursor_movement=True)

                self.game_state.belt[potion_row, potion_col] = potion_to_buy.name
                time.sleep(0.5)
    def buy_potions(self) -> None:
        self.walk_to_potion_seller()
        self.click_on_potion_seller()
        self.buy_potions_from_merchant()
        self.save_and_exit_game()
        self.game_state.is_buying_potions = False
        self.main_loop()
    def activate_skill(self, skill: Skill) -> None:
        if self.game_state.active_weaponset == Weaponset.PRIMARY: 
            if skill != self.game_state.active_skill:
                skill_key_binding = self.game_info.keybindings[skill]
                self.press_key(skill_key_binding)
                time.sleep(self.game_info.seconds_per_frame)
                self.game_state.active_skill = skill
        else:
            if self.game_state.active_skill_secondary_weaponset != skill:
                skill_key_binding = self.game_info.keybindings[skill]
                self.press_key(skill_key_binding)
                time.sleep(self.game_info.seconds_per_frame)
                self.game_state.active_skill_secondary_weaponset = skill
    def use_skill(
        self,
        point: Point,
        skill: Skill,
        sleep_after_cursor_movement: bool=False
        ) -> None:
        
        self.move_mouse(point)
        if sleep_after_cursor_movement:
            time.sleep(self.game_info.seconds_per_frame)
        else:
            time.sleep(0.01)
            
        self.activate_skill(skill)
        remaining_cooldown = self.game_state.last_skill_used_time + self.game_state.last_skill_used_cooldown - time.time()
        if remaining_cooldown > 0:
            time.sleep(remaining_cooldown)
        else:
            time.sleep(0.01)
        self.game_state.last_skill_used_time = time.time()
        self.game_state.last_skill_used_cooldown = self.get_skill_cooldown_in_seconds(skill, self.game_state.active_weaponset)
        self.click(point, click_type = ClickType.RIGHT)
    def get_next_file_path(self, folder_path: str, file_name: str, pad_length: int = 4, separator: str = '-') -> str:
        if separator in file_name:
            raise ValueError(f"Filename must not contain '{separator}'")

        file_name = os.path.splitext(file_name)

        if folder_path not in self.game_info.folders_last_file_ids:
            self.game_info.folders_last_file_ids[folder_path] = {}
        
        if file_name[0] in self.game_info.folders_last_file_ids[folder_path]:
            file_id = str(int(self.game_info.folders_last_file_ids[folder_path][file_name[0]])+1)
        else:
            highest_id = -1
            for file in os.listdir(folder_path):
                base = os.path.splitext(file)[0]
                base_split = base.split(separator)
                if base_split[0] == file_name[0]:
                    highest_id = max(highest_id, int(base_split[1]))
            file_id = str(highest_id + 1)

        self.game_info.folders_last_file_ids[folder_path][file_name[0]] = file_id

        file_name_new = file_name[0] + separator + pad_id(file_id, pad_length) + file_name[1]
        file_path = os.path.join(folder_path, file_name_new)
        
        return file_path
    def move_mouse_to_safe_zone(self) -> None:
        if (
            self.game_state.mouse_point.col != self.game_info.safe_mouse_position.col or
            self.game_state.mouse_point.row != self.game_info.safe_mouse_position.row
        ):
            self.move_mouse(self.game_info.safe_mouse_position, sleep_after_movement=True)

    def take_screenshot(self, move_mouse_to_safe_zone_before_screenshot: bool=False) -> np.ndarray:
        if win32api.GetAsyncKeyState(win32con.VK_CONTROL) < 0:
            print("breaking")
            self.release_key("alt")
            self.release_key("ctrl")
            self.game_state.is_buying_potions = False
            self.game_state.is_in_game = False
            self.game_state.is_in_monster_area = False
            sys.exit()

        # TODO https://github.com/BoboTiG/python-mss/issues/59

        self.activate_game_window() # Ensure that the game window is visible on the screen before we take the screenshot.

        if move_mouse_to_safe_zone_before_screenshot:
            self.move_mouse_to_safe_zone()
        
        with mss() as sct:
        #img: np.ndarray = np.array(self.game_info.sct.grab(self.game_info.game_window_screen_location))
            img: np.ndarray = np.array(sct.grab(self.game_info.game_window_screen_location), dtype=np.uint8)
            # Remove the fourth value after RGB. I dont know why its there and i dont need it.
            img: np.ndarray = np.delete(img, 3, axis=(2))

            if self.game_info.game_version == GameVersion.RESURRECTED:
                img = transform_image_from_resurrected_to_classic(self.game_info.c_functions, img)

        if self.game_state.is_in_game and self.game_state.is_checking_health is False and self.game_state.is_buying_potions is False:
            self.check_if_we_have_died(img)

            self.game_state.is_checking_health = True
            self.update_health_and_mana(img)
            self.check_health_and_mana()
            self.game_state.is_checking_health = False

        return img
    def get_skill_cooldown_in_seconds(self, skill: Skill, weaponset: Weaponset) -> float:
        if skill in (Skill.LIGHTNING, Skill.CHAIN_LIGHTNING):
            if weaponset == Weaponset.PRIMARY:
                cooldown_seconds = self.game_info.cooldown_seconds_primary_lightning
            else:
                cooldown_seconds = self.game_info.cooldown_seconds_secondary_lightning
        else:
            if weaponset == Weaponset.SECONDARY:
                cooldown_seconds = self.game_info.cooldown_seconds_secondary
            else:
                cooldown_seconds = self.game_info.cooldown_seconds_primary
                
        return cooldown_seconds
    def get_loot(self, img: np.ndarray, window_offsets: np.ndarray, colors: Union[list[ItemQuality], None] = None) -> list[Item]:
        items = get_font_symbols(
            self.game_info.c_functions,
            img,
            self.game_info.font_sprite_data,
            window_offsets=window_offsets,
            colors=colors
            )
        return items
    
    def get_health_and_mana(self, img: np.ndarray) -> Health_Mana:
        # If the mouse is currently hovering over the health or mana numbers then we are unable to read the numbers so we instead return the current numbers.
        # The numbers will just have to be updated another time.
        is_mouse_hovering_over_health_numbers: bool = False
        is_mouse_hovering_over_mana_numbers: bool = False

        if self.game_state.mouse_point.row > 468 and self.game_state.mouse_point.row < 507:
            if self.game_state.mouse_point.col < 120:
                is_mouse_hovering_over_health_numbers = True

            if self.game_state.mouse_point.col > 700:
                is_mouse_hovering_over_mana_numbers = True
                
        if is_mouse_hovering_over_health_numbers or is_mouse_hovering_over_mana_numbers:
            return self.game_state.health_mana

        life_mana_area = img[490: 506, :]
        items = self.get_loot(life_mana_area, self.game_info.window_offsets_life_mana, colors=[ItemQuality.COMMON])

        return Health_Mana(
            int(items[1].name),
            int(items[2].name),
            int(items[-2].name),
            int(items[-1].name)
        )
    def check_if_we_have_died(self, img: np.ndarray) -> None:
        has_died: bool = validate_page(img, self.game_info.validation_pixels["has_died"], 5)
        if has_died:
            time.sleep(0.1)
            self.press_key("esc")
            time.sleep(2)

            self.save_and_exit_game()
            self.enter_game()
            self.set_game_state()
            self.pickup_corpse()
            #self.reset_game()
            self.main_loop()
    def pickup_corpse(self) -> None:
        for act, point in zip(
            ACT,
            (
                Point(263, 418),
                Point(267, 380),
                Point(276, 400),
                Point(272, 387),
                Point(258, 407)
            )):

            if act == self.game_state.current_act:
                self.click(point, sleep_after_cursor_movement=True)
                break
        time.sleep(1)
    def get_next_two_frames(self) -> tuple[np.ndarray, np.ndarray]:
        img_1 = self.take_screenshot()

        img_2 = self.take_screenshot()
        while get_diff_count(self.game_info.c_functions, img_1, img_2) == 0:
            img_2 = self.take_screenshot()

        return (img_1, img_2)
    def scan_image_for_monsters(self, img_1: np.ndarray, img_2: np.ndarray) -> list[Point]:
        for img in (img_1, img_2):
            remove_areas_from_image(
                self.game_info.c_functions,
                img,
                [
                    self.game_info.health_globe_screen_location,
                    self.game_info.mana_globe_screen_location
                ]
            )
        
        img_transformed_1 = np.array(transform_image(img_1, self.game_info.transformation_array), dtype=np.uint8)
        img_transformed_2 = np.array(transform_image(img_2, self.game_info.transformation_array), dtype=np.uint8)

        match_rows, match_cols = detect_monsters(self.game_info.monster_detection, img_2, img_transformed_1, img_transformed_2, self.game_info.skill_bar_screen_location.top, 0)

        middle_points = []
        for row, col in zip(match_rows, match_cols):
            middle_points.append(Point(row + 10, col + 10)) # TODO Fix this. this is not really the middle point, but just 10 pixels off the top-left corner.
        return middle_points

    def attack_monsters(self, monster_middle_points: list[Point]) -> None:
        start_time = time.time()
        cooldown = self.get_skill_cooldown_in_seconds(self.game_info.primary_attack_skill, self.game_state.active_weaponset)
        
        in_combat = True
        in_combat_count = 0

        if Skill.STATIC_FIELD in self.game_info.keybindings:
            if len(monster_middle_points) > 4:
                for _ in range(2):
                    self.use_skill(Point(300, 300), Skill.STATIC_FIELD)
        
        while in_combat:
            start_time = time.time()
            
            for i, monster_middle_point in enumerate(monster_middle_points):
                self.use_skill(monster_middle_point, self.game_info.primary_attack_skill)
                
                if i == 2 or (len(monster_middle_points) == 2 and i == 1):
                    break
                
                time.sleep(cooldown)
                
            start_time_scan = time.time()
            remaining_cooldown = (cooldown - ((start_time_scan - start_time) % cooldown))
            # Sleep untill 0.08 seconds before the cooldown has run out.
            # This is approximately the time it takes to scan the screen for monsters.
            # We will then be ready with updated information right when the cooldown runs out
            # TODO Instead of doing this we should track the average scan time and use that as the sleep time.
            if remaining_cooldown - 0.08 > 0:
                time.sleep(remaining_cooldown -0.08)
            monster_middle_points = self.scan_image_for_monsters(*self.get_next_two_frames())
            
            # Wait untill the cooldown has run out
            remaining_cooldown = remaining_cooldown - (time.time() - start_time_scan)
            if remaining_cooldown > 0:
                time.sleep(remaining_cooldown)
            
            if len(monster_middle_points) <= 1:
                in_combat = False
            if in_combat_count >= 2:
                in_combat = False
            in_combat_count += 1
    def filter_items(self, items: list[Item]) -> tuple[list[Item], list[Item]]:
        items_walk: list[Item] = []
        items_telekinesis: list[Item] = []

        for item in items:

            # Skip if item is of the types below:
            # Catacombs Level 2 
            # v 1.14b
            # Difficulty: Hell
            # EXPANSION
            if item.quality == ItemQuality.UNIQUE and item.point.row in (17, 33, 49, 65):
                continue
            
            # Skip if item is of the types below:
            # Life: 2469 / 2469
            # Mana: 863 / 863
            if item.quality == ItemQuality.COMMON and item.point.row == 498:
                continue

            if self.is_item_wanted(item.name, item.quality):
                if item.name in self.game_info.items_that_can_be_picked_up_with_telekinesis or "Gold" in item.name:
                    if "Gold" in item.name:
                        items_telekinesis.append(item)
                    elif self.get_potion_belt_position(item.name)[0] != -1:
                        items_telekinesis.append(item)
                else:
                    items_walk.append(item)
                
        return items_walk, items_telekinesis
    def pickup_item(self, item: Item, is_potion_or_gold:bool=False, use_telekinesis: bool=False) -> None:
        print("pickup: ", item.name)

        if is_potion_or_gold:
            if use_telekinesis:
                self.use_skill(item.point, Skill.TELEKINESIS, sleep_after_cursor_movement=True)
            else:
                self.click(item.point, sleep_after_cursor_movement=True)

            if not "Gold" in item.name:
                potion_row, potion_col = self.get_potion_belt_position(item.name)
                self.game_state.belt[potion_row, potion_col] = item.name
        else:
            self.click(item.point, sleep_after_cursor_movement=True)
            
            counter = 1
            while self.is_moving() and counter < 50:
                counter += 1
            time.sleep(self.game_info.seconds_per_frame)
    def pickup_loot(self) -> None:
        self.hold_key(self.game_info.keybindings[GameInterfaceAction.ITEMS])
        
        # TODO This can be improved
        picked_up_potion = False
        for _ in range(5):
            img = self.take_screenshot(move_mouse_to_safe_zone_before_screenshot=True)
            remove_areas_from_image(
                self.game_info.c_functions,
                img,
                (
                    self.game_info.skill_bar_screen_location,
                    self.game_info.mini_panel_screen_location
                )
            )

            items = self.get_loot(img, self.game_info.window_offsets_all)
            items_walk, items_telekinesis = self.filter_items(items)

            if items_telekinesis:
                if "Potion" in items_telekinesis[0].name:
                    picked_up_potion = True
                self.pickup_item(items_telekinesis[0], is_potion_or_gold=True, use_telekinesis=Skill.TELEKINESIS in self.game_info.keybindings)
                time.sleep(self.game_info.seconds_per_frame * 4)
            else:
                break

        if picked_up_potion:
            self.move_mouse_to_safe_zone()
            self.update_belt()
            
        for _ in range(5):
            img = self.take_screenshot(move_mouse_to_safe_zone_before_screenshot=True)
            remove_areas_from_image(
                self.game_info.c_functions,
                img,
                (
                    self.game_info.skill_bar_screen_location,
                    self.game_info.mini_panel_screen_location
                )
            )
            items = self.get_loot(img, self.game_info.window_offsets_all)
            items_walk, items_telekinesis = self.filter_items(items)
            
            if items_walk:
                row_size, col_size = self.get_item_space(items_walk[0].name)
                if find_item_placement_in_cell_area(self.game_state.inventory, row_size, col_size)[0] != -1: # Test if we have space in the inventory for the item.
                    self.pickup_item(items_walk[0])
                    self.update_inventory()
                else:
                    print(f"No space in inventory for item {items_walk[0].name}. Moving items to stash.")
                    self.move_items_to_stash()
            else:
                break
                
        if np.count_nonzero(self.game_state.inventory == 0) < 8: # If we have less than 8 empty inventory cells left
            self.move_items_to_stash()
                        
        self.release_key(self.game_info.keybindings[GameInterfaceAction.ITEMS])
    def reset_game(self) -> None:
        print("Resetting game!")
        self.save_and_exit_game()
        self.main_loop()
    def attack_monsters_and_loot(self) -> None:
        monster_middle_points = self.scan_image_for_monsters(*self.get_next_two_frames())

        if len(monster_middle_points) > 1:
            self.attack_monsters(monster_middle_points)

            self.move_mouse(self.game_info.safe_mouse_position, sleep_after_movement=True) # Move the mouse to the far right corner so it doesn't block any loot text.

            if Skill.TELEPORT in self.game_info.keybindings:
                self.activate_skill(Skill.TELEPORT)

            self.hold_key(self.game_info.keybindings[GameInterfaceAction.ITEMS])
            time.sleep(self.game_info.seconds_per_frame * 3)
            self.pickup_loot()
    def open_menu(self) -> None:
        self.press_key("esc")
    def _save_and_exit_game(self) -> None:
        self.click(Point(260, 410))
    def enter_single_player_menu(self) -> None:
        self.click(Point(308, 320))
    def enter_options_menu(self) -> None:
        self.click(Point(230, 410))
    def enter_automap_menu(self) -> None:
        self.click(Point(260, 410))
    def enter_video_menu(self) -> None:
        self.click(Point(210, 410))
    def select_character(self) -> None:
        # By default we select the character in the upper left cell
        character_screen_point = Point(150, 100)

        # Scan the screen for our characters name so we can select it even it not the character in the upper left cell
        img = self.take_screenshot()
        items = self.get_loot(img, self.game_info.window_offsets_all, colors=[ItemQuality.UNIQUE]) # TODO Find another word instead of 'items'. Something like 'texts'
        for item in items:
            if item.name.lower() == self.game_info.character_name.lower():
                character_screen_point = item.point

        self.click(character_screen_point)
        time.sleep(0.12)
        self.press_key("enter")
    def enter_game_with_difficulty(self, difficulty: Difficulty) -> None:
        for key, _difficulty in zip(("r", "n", "h"), Difficulty):
            if difficulty == _difficulty:
                self.press_key(key)
    def do_action_and_validate(self, action_function: Callable, list_of_validation_pixels: list[tuple[ValidationPixel]], list_of_valid_count_minimum: list[int], *args, repeat_action: bool=True, repeat_frequency: int = 20) -> int:
        for i in range(200):
            if i == 0 or (i % repeat_frequency == 0 and repeat_action):
                action_function(*args)
            time.sleep(self.game_info.seconds_per_frame)

            # Loop through the validation pixels and return the state we ended up in after performing the action.
            for j, (validation_pixels, valid_count_minimum) in enumerate(zip(list_of_validation_pixels, list_of_valid_count_minimum)):
                if validate_page(self.take_screenshot(), validation_pixels, valid_count_minimum):
                    return j

        raise Exception("Could not perform action")
    
    def open_game_menu(self) -> None:
        # If we hold alt while pressing escape it will minimize the game window instead of opening up the game menu. Therefore we make sure we are not currently holding alt down.
        if self.game_info.keybindings[GameInterfaceAction.ITEMS] in self.game_state.held_keys:
            self.release_key(self.game_info.keybindings[GameInterfaceAction.ITEMS])
            time.sleep(self.game_info.seconds_per_frame)

        self.do_action_and_validate(self.open_menu, [self.game_info.validation_pixels["menu"]], [8])

    def save_and_exit_game(self) -> None:
        self.game_state.is_in_game = False
        self.game_state.is_in_monster_area = False

        self.open_game_menu()
        self.do_action_and_validate(self._save_and_exit_game, [self.game_info.validation_pixels["exit_game"]], [4])
        self.game_state.is_in_game = False
    def enter_game(self) -> None:
        """This function enters the single player menu, selects our character and then enters a game with the desired difficulty level (normal, nightmare or hell)
        """
        self.game_state.is_in_game = False
        self.do_action_and_validate(self.enter_single_player_menu, [self.game_info.validation_pixels["single_player_menu"]], [4])
        state = self.do_action_and_validate(self.select_character, [self.game_info.validation_pixels["difficulty_menu"], self.game_info.validation_pixels["in_game"]], [4,4])

        # If our character has not completed the game on normal difficulty yet then the character does not have to choose between normal, nightmare or hell
        # when entering the game. This means that it will already be in the game at this point.
        # Therefore we only perform the action with the difficulty menu if we are not already in the game
        if state == 0: # If the state is 0 after selecting the character then we need to select the desired difficulty
            self.do_action_and_validate(self.enter_game_with_difficulty, [self.game_info.validation_pixels["in_game"]], [4], self.game_info.game_difficulty, repeat_action=False)
    
    def enter_command(self, command: str) -> None:
        self.activate_ui_element(GameInterfaceAction.CHAT)
        pyautogui.typewrite(command)
        self.deactivate_ui_element(GameInterfaceAction.CHAT)
    
    def set_no_pickup(self) -> None:
        self.enter_command("/nopickup")

    def set_players(self) -> None:
        if self.game_info.players_count > 1:
            self.enter_command(f"/players{self.game_info.players_count}")

    def switch_lighting_quality(self) -> None:
        self.click(Point(210,400))

    def switch_automap_size(self) -> None:
        self.click(Point(170,400))

    def switch_automap_show_party(self) -> None:
        self.click(Point(310,400))

    def switch_automap_fade(self) -> None:
        self.click(Point(210,400))
        
    def set_video_options(self) -> None:
        """Set Lighting Quality to LOW. This will remove the rain effect which will make it easier to detect monsters.
        """

        self.open_game_menu()
        self.do_action_and_validate(self.enter_options_menu, [self.game_info.validation_pixels["options_menu"]], [4])
        self.do_action_and_validate(self.enter_video_menu, [self.game_info.validation_pixels["video_options_menu"]], [4])

        is_lighting_quality_low = validate_page(self.take_screenshot(), self.game_info.validation_pixels["lighting_quality_low"], 4)

        if is_lighting_quality_low is False:
            self.do_action_and_validate(self.switch_lighting_quality, [self.game_info.validation_pixels["lighting_quality_low"]], [4], repeat_frequency=2)

        self.press_key("esc")
        time.sleep(self.game_info.seconds_per_frame)

    def set_automap_options(self) -> None:
        self.open_game_menu()
        self.do_action_and_validate(self.enter_options_menu, [self.game_info.validation_pixels["options_menu"]], [4])
        self.do_action_and_validate(self.enter_automap_menu, [self.game_info.validation_pixels["automap_options_menu"]], [4])

        img = self.take_screenshot()

        is_automap_size_full = validate_page(img, self.game_info.validation_pixels["automap_size_full"], 4)
        is_automap_fade_no = validate_page(img, self.game_info.validation_pixels["automap_fade_no"], 4)
        is_automap_show_party_no = validate_page(img, self.game_info.validation_pixels["automap_show_party_no"], 4)

        if is_automap_size_full is False:
            self.switch_automap_size()
            time.sleep(self.game_info.seconds_per_frame)

        if is_automap_show_party_no is False:
            self.switch_automap_show_party()
            time.sleep(self.game_info.seconds_per_frame)

        if is_automap_fade_no is False:
            self.do_action_and_validate(self.switch_automap_fade, [self.game_info.validation_pixels["automap_fade_no"]], [4], repeat_frequency=2)

        self.press_key("esc")
        time.sleep(self.game_info.seconds_per_frame)

    def do_first_run_actions(self) -> None:

        if self.game_info.set_game_options:
            self.set_video_options()
            self.set_automap_options()

        self.update_inventory()
        if self.game_state.reserved_inventory_cells is None:
            self.game_state.reserved_inventory_cells = self.game_state.inventory

        # Set belt
        self.activate_ui_element(GameInterfaceAction.BELT)
        self.game_state.set_belt(get_num_belt_rows(self.take_screenshot()))
        self.update_belt()

        self.set_no_pickup()
        self.set_players()
    
    def set_game_state(self):
        self.game_state.initialize() # Resetting the state when we enter the game.
        self.game_state.is_in_game = True

        # Remove mercenary and other portraits
        self.press_key(self.game_info.keybindings[GameInterfaceAction.SHOW_PORTRAITS])
        time.sleep(self.game_info.seconds_per_frame)

        if self.game_state.is_first_run:
            self.do_first_run_actions()
            self.game_state.is_first_run = False
        
    def exit_and_re_enter_game(self) -> None:
        self.save_and_exit_game()
        self.enter_game()
        self.set_game_state()
    def start_diablo2(self) -> None:
        self.game_state.is_in_game = False
        started_game_from_script = False
        if self.is_game_started() is False:
            started_game_from_script = True

            if not os.path.exists(self.game_info.diablo2_exe_file_path):
                print(f'The specified path to the Diablo 2 executable "{self.game_info.diablo2_exe_file_path}" does not exist. Exiting program!')
                sys.exit()

            os.system(f'cmd /c ""{self.game_info.diablo2_exe_file_path}" -w"')
            while self.is_game_started() is False:
                time.sleep(0.01)

        self.game_info.game_window_handle = self.get_game_window_handle()
        self.game_info.game_window_screen_location = self.get_game_window_screen_location(started_game_from_script=started_game_from_script)
        self.activate_game_window()

        if started_game_from_script:
            self.do_action_and_validate(self.click, [self.game_info.validation_pixels["exit_game"]], [4], Point(500, 700)) # Click to skip the initial waiting screen.
    def take_and_open_screenshot(self) -> None:
        img = self.take_screenshot()
        path = os.path.join(self.game_info.root_folder, "img.png")
        cv2.imwrite(path, img)
        os.startfile(path)
    def _swap_weapons(self) -> None:
        self.press_key(self.game_info.keybindings["swap_weapons"])
    def swap_weapons(self) -> None:
        if self.game_state.active_weaponset == Weaponset.PRIMARY:
            validation_pixels = self.game_info.validation_pixels["secondary_weapons"]
        else:
            validation_pixels = self.game_info.validation_pixels["primary_weapons"]

        # We cannot swap weapons while in spell cooldown.
        remaining_cooldown = self.game_state.last_skill_used_time + self.game_state.last_skill_used_cooldown - time.time()
        if remaining_cooldown > 0:
            time.sleep(remaining_cooldown + self.game_info.seconds_per_frame)

        self.do_action_and_validate(self._swap_weapons, [validation_pixels], [20])

        if self.game_state.active_weaponset == Weaponset.PRIMARY:
            self.game_state.active_weaponset = Weaponset.SECONDARY
        else:
            self.game_state.active_weaponset = Weaponset.PRIMARY
    def get_act_1_town_waypoint_location(self) -> int:
        waypoint_locations_rows, waypoint_locations_cols = self._find_waypoint_on_map()
        waypoint_location = None
        if (waypoint_locations_rows == (247,247)).all() and (waypoint_locations_cols == (420,427)).all():
            waypoint_location = 1
        elif (waypoint_locations_rows == (287,287)).all() and (waypoint_locations_cols == (468,475)).all():
            waypoint_location = 2
        elif (waypoint_locations_rows == (279,279)).all() and (waypoint_locations_cols == (468,475)).all():
            waypoint_location = 3
        else:
            raise WaypointLocationError("Could not find waypoint location in act 1 town")
        return waypoint_location
    def _find_waypoint_on_map(self) -> tuple[np.ndarray, np.ndarray]:
        self.activate_ui_element(GameInterfaceAction.AUTOMAP)
        img = self.take_screenshot()

        return find_waypoint_on_map(self.game_info.c_functions, img)
    def walk_route(self, points: list[Point], wait_on_last_click: bool, validation_pixels=None, valid_count_minimum=None) -> None:
        for i, point in enumerate(points):
            if i == len(points) -1 and wait_on_last_click: # If it is the last click
                if Skill.TELEKINESIS in self.game_info.keybindings:
                    self.use_skill(point, Skill.TELEKINESIS, sleep_after_cursor_movement=True)
                else:
                    self.click(point, sleep_after_cursor_movement=True)
            else:
                self.click(point)
            while self.is_moving():
                pass

        if wait_on_last_click:
            counter = 0
            while validate_page(self.take_screenshot(), validation_pixels, valid_count_minimum) is False:
                time.sleep(0.01)
                if counter >= 100:
                    raise ValidationError("Could not validate")
                counter += 1

    def get_route_to_waypoint(self) -> list[Point]:
        current_zone = self.game_state.current_zone
        route_name = ""
        if current_zone == "Rogue Encampment":
            waypoint_location = self.get_act_1_town_waypoint_location()
            if waypoint_location == 1:
                route_name = "act_1_start_to_waypoint_1"
            elif waypoint_location == 2:
                route_name = "act_1_start_to_waypoint_2"
            elif waypoint_location == 3:
                route_name = "act_1_start_to_waypoint_3"
        elif current_zone == "Lut Gholein":
            route_name = "act_2_start_to_waypoint"
        elif current_zone == "Kurast Docks":
            route_name = "act_3_start_to_waypoint"
        elif current_zone == "The Pandemonium Fortress":
            route_name = "act_4_start_to_waypoint"
        elif current_zone == "Harrogath":
            route_name = "act_5_start_to_waypoint"

        if route_name == "":
            print(f"Could not get route to waypoint in zone {current_zone}! Exiting program!")
            sys.exit()

        return self.game_info.fixed_routes[route_name]
    def walk_to_waypoint(self) -> None:
        if self.game_state.current_act is None:
            self.update_current_act_and_zone()

        if Skill.TELEKINESIS in self.game_info.keybindings:
            self.activate_skill(Skill.TELEKINESIS)

        # Walk to the waypoint
        self.walk_route(self.get_route_to_waypoint(), wait_on_last_click=True, validation_pixels=self.game_info.validation_pixels["waypoint_menu"], valid_count_minimum=4)
    def walk_to_stash(self) -> None:
        if self.game_state.current_act is None:
            self.update_current_act_and_zone()

        if Skill.TELEKINESIS in self.game_info.keybindings:
            self.activate_skill(Skill.TELEKINESIS)

        # Get the route to the stash
        for town_zone, route_name in zip(("Rogue Encampment",     "Lut Gholein",          "Kurast Docks",         "The Pandemonium Fortress", "Harrogath"),
                                         ("act_1_start_to_stash", "act_2_start_to_stash", "act_3_start_to_stash", "act_4_start_to_stash",     "act_5_start_to_stash")):
            if town_zone == self.game_state.current_zone:
                route = self.game_info.fixed_routes[route_name]
                break
        
        # Walk to the stash
        self.walk_route(route, wait_on_last_click=True, validation_pixels=self.game_info.validation_pixels["stash"], valid_count_minimum=4)
    def get_isolated_automap(self) -> tuple[bool, Automap]:
        self.activate_ui_element(GameInterfaceAction.AUTOMAP)
        img = self.take_screenshot()

        return self._get_isolated_automap(img)

    def _get_isolated_automap(self, img: np.ndarray) -> tuple[bool, Automap]:
        zone_name = self.get_current_zone(img)

        if zone_name not in self.game_info.zone_mapping:
            print(f'zone "{zone_name}" not in zone mapping dictionary!')
            return (False, None)
        else:
            if self.game_info.zone_mapping[zone_name] not in self.game_info.area_mapping:
                print(f'Area "{self.game_info.zone_mapping[zone_name]}" is not loaded into memory!')
                return (False, None)

        remove_areas_from_image(
            self.game_info.c_functions,
            img, 
            (
                self.game_info.life_text_screen_location,
                self.game_info.mana_text_screen_location,

                self.game_info.life_numbers_screen_location,
                self.game_info.mana_numbers_screen_location,

                self.game_info.skill_bar_screen_location,
                self.game_info.mini_panel_screen_location,

                self.game_info.health_globe_screen_location,
                self.game_info.mana_globe_screen_location
            )
        )

        return (True, get_isolated_map(
            self.game_info.c_functions,
            img,
            zone_name,
            self.game_info.area_mapping,
            self.game_info.zone_mapping,
            self.game_info.transformation_array,
            self.game_info.window_offsets,
            self.game_info.middle_indices,
            self.game_info.window_offsets_zero
            ))
    def is_moving(self) -> bool:
        img1 = self.take_screenshot()
        time.sleep(self.game_info.seconds_per_frame)
        img2 = self.take_screenshot()

        return get_diff_percentage(self.game_info.c_functions,
        img1, img2) > 0.2 # While moving most of the screen will change between each frame. While standing still most of the screen will be stay the same.
    def get_cast_time(self) -> float:
        if self.game_state.active_weaponset == Weaponset.PRIMARY:
            return self.game_info.cast_seconds_primary
        else:
            return self.game_info.cast_seconds_secondary
    def walk_path(self, path_diffs: list[Point]) -> None:
        coordinates = get_path_coordinates(path_diffs)[0:3]
        cast_time = self.get_cast_time()
        
        for point in coordinates:
            if Skill.TELEPORT in self.game_info.keybindings:
                self.use_skill(point, Skill.TELEPORT)
                # Wait until teleport has moved our character. We will first scan for monsters after the teleport has moved us.
                time.sleep(cast_time)
            else:
                self.click(point)
                while self.is_moving(): # Wait until we stop moving
                    pass

            # Scan for monsters. If we find any then we attack them and loot. Otherwise we just keep moving
            self.attack_monsters_and_loot()
    def get_path_and_walk_it(
        self,
        map_white: np.ndarray,
        map_walked: np.ndarray,
        map_distances: np.ndarray,
        start_position: Point,
        use_wide_start: bool=False
        ) -> None:

        automap_mazed, un_walked_steps = fill_maze(self.game_info.c_functions, map_white, map_walked, 1050, start_position, use_wide_start=use_wide_start, wide_start_size=self.game_info.movement_wide_start_size)
        
        end_position = get_target_position(self.game_info.step_limit, automap_mazed, un_walked_steps)

        # Get the path to the chosen end-point
        path, path_diff = get_path_diagonally(automap_mazed, map_distances, end_position)

        self.walk_path(path_diff)
    def run(self, num_loops: int) -> None:
        
        (success, map_before_movement) = self.get_isolated_automap()
        if success is False:
            return

        stitched_image = map_before_movement.image

        walked = np.zeros(stitched_image.shape, dtype=np.uint8)
        distances = get_distances_to_edge(self.game_info.c_functions, stitched_image, self.game_info.square_roots)

        current_point = Point(0,0) # current_position
        total_movement = TotalMovement(0,0,0,0)
        start_position = Point(current_point.row + 286, current_point.col + 408)

        for i in range(num_loops):
            print('i: ', i)
            self.game_state.re_cast_buffs_if_expired()
            self.get_path_and_walk_it(stitched_image, walked, distances, start_position, use_wide_start = i == 0)

            (success, map_after_movement) = self.get_isolated_automap()
            if success is False:
                return

            diff = get_diffs(self.game_info.c_functions, map_before_movement, map_after_movement)

            current_point = adjust_point(diff, current_point)

            pad = get_pad_stats(total_movement, current_point)
            stitched_image = pad_image(stitched_image, pad)
            walked = pad_image(walked, pad)

            total_movement = adjust_total_movement(total_movement, current_point)

            current_base_point = get_draw_base_point(total_movement, current_point)
            previous_base_point = get_previous_base_point(current_base_point, diff)

            previous_mid = Point(previous_base_point.row + 286, previous_base_point.col + 408)
            current_mid = Point(current_base_point.row + 286, current_base_point.col + 408)

            stitched_image = draw_image(stitched_image, map_after_movement.image, current_base_point)

            distances = get_distances_to_edge(self.game_info.c_functions, stitched_image, self.game_info.square_roots)

            # Remove some pixels around where I came from as that position cannot be a wall.
            stitched_image[get_diamond_indices(5, row_offset=previous_mid.row, col_offset=previous_mid.col)] = 0 # This function doesn't work # TODO Check this

            # Fill the maze from our current position on the stitched maps
            automap_mazed, un_walked_steps = fill_maze(self.game_info.c_functions, stitched_image, walked, 600, current_mid, use_wide_start=True, wide_start_size=12)

            # Get path from our current position (from where we have mazed the map) to the previous position
            path, path_diff = get_path_diagonally(automap_mazed, distances, previous_mid)
            if len(path) > 1:
                
                # Mark the walked path in stitched_maps["walked"]
                radius = 25
                for point in path:
                    area = walked[
                        point.row - radius : point.row + radius,
                        point.col - radius : point.col + radius
                        ]

                    area[area == 0] = 140
                    
                current_mid = path[0]
            else:
                pass
                
            map_before_movement = map_after_movement
            start_position = current_mid

            print()
    def get_waypoint_point(self) -> Point:
        items = self.get_loot(self.take_screenshot(), self.game_info.window_offsets_all, colors=[ItemQuality.GREY, ItemQuality.COMMON])

        for item in items:
            if item.name == self.game_info.zone_to_farm.value:
                if item.quality == ItemQuality.GREY:
                    print(f'Your character does not have the waypoint to your specified farm zone "{self.game_info.zone_to_farm.value}" Exiting program!')
                    sys.exit()
                elif item.quality == ItemQuality.COMMON:
                    return item.point

        print(f'Could not find the waypoint text of your specified farm zone "{self.game_info.zone_to_farm.value}" Exiting program!')
        sys.exit()
    def ensure_mouse_is_out_of_area(self, area: Box) -> None:
        if is_point_in_area(self.game_state.mouse_point, area):
            self.move_mouse(Point(0, 0), sleep_after_movement=True)
    def waypoint_to_zone(self) -> None:
        # If we are already in the act of the zone, then that page will be selected by default. If not then we will need to select it first.
        if not is_in_zone_act(self.game_info.zone_to_farm, self.game_state.current_act, self.game_info.acts_waypointzones):
            self.click(get_zone_waypoint_act_point(self.game_info.zone_to_farm, self.game_info.acts_waypointzones)) # Click the tab for the act of the farm zone.
            time.sleep(0.2) # TODO Make a real waiting function
        
        self.ensure_mouse_is_out_of_area(self.game_info.waypoint_text_area)
        waypoint_point = self.get_waypoint_point()

        self.click(waypoint_point) # Go to the zone.

        # Wait until the waypoint menu is no longer open.
        counter = 0
        while validate_page(self.take_screenshot(), self.game_info.validation_pixels["waypoint_menu"], 4) is True:
            time.sleep(self.game_info.seconds_per_frame)

            counter += 1
            if counter > 200:
                raise ValueError("Could not enter zone from waypoint")
        time.sleep(self.game_info.seconds_per_frame)

        # Update state values after entering zone from waypoint
        self.game_state.is_in_monster_area = True
        self.game_state.current_act = get_waypoint_zone_act(self.game_info.zone_to_farm, self.game_info.acts_waypointzones)
        self.game_state.current_zone = self.game_info.zone_to_farm.value
    def main_loop(self) -> None:
        try:
            self.enter_game()
            self.set_game_state()
            self.toogle_health_and_mana_text()
            self.walk_to_waypoint()
            self.waypoint_to_zone()
            self.run(40)
            self.save_and_exit_game()
            self.main_loop() # Recursion.
        except Exception:
            print(traceback.format_exc())
            self.game_state.is_buying_potions = False
            self.release_key("ctrl")
            self.release_key("alt")
            self.reset_game()
    def activate_ui_element(self, ui_element: GameInterfaceAction) -> None:
        if ui_element not in self.game_state.ui_elements_toogled:
            self.game_state.ui_elements_toogled[ui_element] = False

        if self.game_state.ui_elements_toogled[ui_element] is False:
            self.press_key(self.game_info.keybindings[ui_element])
            self.game_state.ui_elements_toogled[ui_element] = True
            time.sleep(self.game_info.seconds_per_frame)
    def deactivate_ui_element(self, ui_element: GameInterfaceAction) -> None:
        if ui_element in self.game_state.ui_elements_toogled:
            if self.game_state.ui_elements_toogled[ui_element]:
                self.press_key(self.game_info.keybindings[ui_element])
                self.game_state.ui_elements_toogled[ui_element] = False
                time.sleep(self.game_info.seconds_per_frame)
    def place_items_in_stash(self) -> None:
        if self.game_info.game_version == GameVersion.CLASSIC:
            num_rows_in_stash = 8
            num_cols_in_stash = 6
        elif self.game_info.game_version == GameVersion.RESURRECTED:
            num_rows_in_stash = 10
            num_cols_in_stash = 10

        img = self.take_screenshot(move_mouse_to_safe_zone_before_screenshot=True)
        inventory_before_item_pickup = are_cells_filled(img, 4, 10, self.game_info.inventory_top_left_point, self.game_info.cells_empty_inventory)
        stash = are_cells_filled(img, num_rows_in_stash, num_cols_in_stash, self.game_info.stash_top_left_point, self.game_info.cells_empty_stash)

        for row in range(4):
            for col in range(10):
                if self.game_state.reserved_inventory_cells[row, col] == False:
                    if inventory_before_item_pickup[row, col]:

                        # Pickup item from iventory
                        item_screen_point = Point(
                            row * 29 + self.game_info.inventory_top_left_point.row + 14,
                            col * 29 + self.game_info.inventory_top_left_point.col + 14
                        )
                        self.click(item_screen_point, sleep_after_click=True)

                        # Take new screenshot and compare with the old screenshot to get the size of the item we picked up
                        img = self.take_screenshot(move_mouse_to_safe_zone_before_screenshot=True)
                        inventory_after_item_pickup = are_cells_filled(img, 4, 10, self.game_info.inventory_top_left_point, self.game_info.cells_empty_inventory)
                        item_size_rows, item_size_cols = get_item_size(inventory_before_item_pickup, inventory_after_item_pickup)

                        # Find where we can place the item in the stash
                        stash_row_placement, stash_col_placement = find_item_placement_in_cell_area(stash, item_size_rows, item_size_cols)
                        if stash_row_placement == -1: # If there is no available space in the stash.
                            print("No more space in stash. Stopping bot!")
                            self.save_and_exit_game()
                            sys.exit()

                        # Move the item to the stash
                        cell_screen_position_row = stash_row_placement * 29 + self.game_info.stash_top_left_point.row
                        cell_screen_position_col = stash_col_placement * 29 + self.game_info.stash_top_left_point.col

                        # When I have an item that is more than one cell large
                        # then the mouse is placed in the middle of the item, which then means that the top left corner
                        # may overlap with another item
                        # Therefore we offset the row and col
                        cell_screen_position_col += item_size_cols * 14
                        cell_screen_position_row += item_size_rows * 14

                        self.click(Point(cell_screen_position_row, cell_screen_position_col), sleep_after_click=True)

                        # Take a new screenshot and get check the new status of the inventory and stash
                        img = self.take_screenshot(move_mouse_to_safe_zone_before_screenshot=True)
                        inventory_before_item_pickup = are_cells_filled(img, 4, 10, self.game_info.inventory_top_left_point, self.game_info.cells_empty_inventory)
                        stash = are_cells_filled(img, num_rows_in_stash, num_cols_in_stash, self.game_info.stash_top_left_point, self.game_info.cells_empty_stash)
        
        time.sleep(self.game_info.seconds_per_frame)
        # Transfer gold to the stash
        self.click(Point(461, 493), sleep_after_cursor_movement=True, sleep_after_click=True) # TODO Is it the same place in Ressurrected version?
        self.press_key("enter")
        time.sleep(self.game_info.seconds_per_frame)

        # Press escape to close the stash
        time.sleep(self.game_info.seconds_per_frame)
        self.press_key("esc")
        time.sleep(self.game_info.seconds_per_frame*4)
    def move_items_to_stash(self) -> None:
        self.save_and_exit_game()
        self.enter_game()
        self.set_game_state()
        self.walk_to_stash()
        self.place_items_in_stash()
        self.update_inventory()
        self.save_and_exit_game()
        self.main_loop()
    def _get_inventory(self, img: np.ndarray) -> np.ndarray:
        return are_cells_filled(
            img,
            4,
            10,
            self.game_info.inventory_top_left_point,
            self.game_info.cells_empty_inventory
            )

    def get_inventory(self) -> np.ndarray:
        self.activate_ui_element(GameInterfaceAction.INVENTORY)
        return self._get_inventory(self.take_screenshot(move_mouse_to_safe_zone_before_screenshot=True))
    def update_inventory(self) -> None:
        self.game_state.inventory = self.get_inventory()
        self.deactivate_ui_element(GameInterfaceAction.INVENTORY)
    def get_items_in_belt(self, img: np.ndarray):

        # If our belt has fewer than 4 rows we move the top row coordinate down accordingly.
        top_left_row = self.game_info.belt_top_left_point.row + ((4 - self.game_state.belt.shape[0]) * (29+3))

        belt_cells = get_cell_information(
            img,
            self.game_state.belt.shape[0],
            4,
            Point(top_left_row, self.game_info.belt_top_left_point.col),
            cell_row_gap = 3,
            cell_col_gap = 2
            )
        
        belt_cells = belt_cells[:,:,0:-3,3:,:]

        belt_items = search_cells_for_items(belt_cells, self.game_info.consumable_items_indices_list, self.game_info.consumable_items_values_list, self.game_info.consumable_items_position_names)
        return belt_items
    def drink_potion(self, potion: str, potion_belt_column: int) -> None:
        self.press_key(str(potion_belt_column+1))
        if potion in self.game_info.healing_potions:
            self.game_state.healing_potion_last_consumed_time = time.time()
        elif potion in self.game_info.mana_potions:
            self.game_state.mana_potion_last_consumed_time = time.time()
        self.game_state.belt = update_belt_after_drinking_potion(self.game_state.belt, potion_belt_column)
    def check_health_mana_points(self, current_points: int, max_points: int, point_limit_soft: float, point_limit_hard: float, last_consumed_time: float, fill_time: float, belt: np.ndarray, potions: dict[str, int]) -> None:
        if self.game_state.is_in_monster_area and current_points / max_points < point_limit_hard:
            self.reset_game()

        if current_points / max_points < point_limit_soft and time.time() - last_consumed_time > fill_time:
            optimal_potion_column = get_optimal_potion_belt_column(current_points, max_points, belt, potions)

            if optimal_potion_column == -1 and time.time() - self.game_state.last_updated_belt_time > 5:
                self.update_belt()
                belt = self.game_state.belt
                optimal_potion_column = get_optimal_potion_belt_column(current_points, max_points, belt, potions)

            if optimal_potion_column != -1:
                potion_to_drink = self.game_state.belt[belt.shape[0]-1, optimal_potion_column]
                self.drink_potion(potion_to_drink, optimal_potion_column)
            else:
                self.game_state.is_buying_potions = True
                self.exit_and_re_enter_game()
                self.buy_potions()
    def check_health_and_mana(self) -> None:
        self.check_health_mana_points(self.game_state.health_mana.health_current, self.game_state.health_mana.health_max, self.game_info.health_limit, self.game_info.health_limit_hard, self.game_state.healing_potion_last_consumed_time, self.game_info.healing_potion_fill_time, self.game_state.belt, self.game_info.healing_potions)
        self.check_health_mana_points(self.game_state.health_mana.mana_current, self.game_state.health_mana.mana_max, self.game_info.mana_limit, -1, self.game_state.mana_potion_last_consumed_time, self.game_info.mana_potion_fill_time, self.game_state.belt, self.game_info.mana_potions)
    def update_belt(self) -> None:
        self.activate_ui_element(GameInterfaceAction.BELT)
        time.sleep(0.04)
        img = self.take_screenshot()

        self.game_state.belt[:] = None # Resetting the belt before we update it
        belt_items = self.get_items_in_belt(img)
        for belt_item in belt_items:
            self.game_state.belt[belt_item.point.row, belt_item.point.col] = belt_item.name
        self.game_state.last_updated_belt_time = time.time()
        
        self.deactivate_ui_element(GameInterfaceAction.BELT)
    def update_health_and_mana(self, img: np.ndarray) -> None:
        try:
            self.game_state.health_mana = self.get_health_and_mana(img)
        except Exception as exception:
            print("Exception when updating health and mana!")
            print(repr(exception))
    def is_item_wanted(self, item_name: str, item_quality: ItemQuality) -> bool:
        is_item_wanted: bool = False
        item_name = remove_prefixes_from_item_name(item_name)
        split_name = item_name.split(" ")
        if len(split_name) == 2 and split_name[1] == "Gold" and split_name[0].isdigit():
            amount_of_gold = int(split_name[0])
            is_item_wanted = amount_of_gold >= self.game_info.min_gold_to_pickup
        elif item_name in self.game_info.item_filter:
            is_item_wanted = self.game_info.item_filter[item_name][item_quality.value]
        else:
            print(f"Item: {item_name} is not in the filter_items dictionary")
            
        return is_item_wanted
    def get_item_space(self, item_name: str) -> tuple[int, int]:
        row_size = None
        col_size = None
        if "Gold" in item_name:
            row_size = 0
            col_size = 0
        else:
            item_name = item_name.replace("Superior ", "")
            row_size = self.game_info.item_filter[item_name]["row_size"]
            col_size = self.game_info.item_filter[item_name]["col_size"]
        
        return row_size, col_size
    def get_potion_type(self, potion_name: str) -> str:
        potion_type = "none"
        for _potion_type, potions in zip(("healing",                      "mana",                      "rejuvenation"),
                                         (self.game_info.healing_potions, self.game_info.mana_potions, self.game_info.rejuvenation_potions)):
            if potion_name in potions:
                potion_type = _potion_type
                break
        
        return potion_type
    def get_potion_belt_position(self, potion_name: str) -> tuple[int, int]:
        potion_type = self.get_potion_type(potion_name)

        # Get potion types in the belt
        belt_cols_type = []
        for col in range(4): # There are always 4 columns in the belt.
            if self.game_state.belt[0, col] is not None: # If the top row is not None, then the column is full and we cannot place more potions there.
                belt_cols_type.append("full_" + self.get_potion_type(self.game_state.belt[-1, col]))
            else:
                belt_cols_type.append(self.get_potion_type(self.game_state.belt[-1, col]))

        num_healing_potion_columns = len([type for type in belt_cols_type if type in ("healing", "rejuvenation", "full_healing", "full_rejuvenation")])
        num_mana_potion_columns = len([type for type in belt_cols_type if type in ("mana", "rejuvenation", "full_mana", "full_rejuvenation")])

        potion_row = -1
        potion_col = -1
        for col in filter(lambda i: belt_cols_type[i]==potion_type, range(len(belt_cols_type))):
            for row in filter(lambda row: self.game_state.belt[row,col] is None, range(self.game_state.belt.shape[0]-1, -1, -1)): 
                potion_row = row
                potion_col = col
                
                break
        
        if potion_row == -1 and (
                                   (potion_type in ("healing", "rejuvenation") and num_mana_potion_columns >= self.game_info.num_belt_columns_reserved_for_mana_potions) or 
                                   (potion_type in ("mana", "rejuvenation") and num_healing_potion_columns >= self.game_info.num_belt_columns_reserved_for_healing_potions)
                                ):
            for col_index, col_type in enumerate(belt_cols_type):
                if col_type == "none":
                    potion_row = self.game_state.belt.shape[0]-1
                    potion_col = col_index
                    break
        
        return potion_row, potion_col
    def get_current_zone(self, img: np.ndarray = None) -> str:
        # If we haven't given a screeshot to the function we take one.
        if img is None:
            self.activate_ui_element(GameInterfaceAction.AUTOMAP)
            img = self.take_screenshot(move_mouse_to_safe_zone_before_screenshot=True)

        zone_name_area = img[9: 9+16, :]
        items = self.get_loot(zone_name_area, self.game_info.window_offsets_zone_name, colors=[ItemQuality.UNIQUE])
        
        zone = None
        for item in items:
            if item.name in self.game_info.zones:
                zone = item.name
                break
                
        # If we couldn't find the zone we assume that we are still in the same zone.
        if zone is None:
            if self.game_state.current_zone is None:
                raise ValueError(f"Zone {zone} is not in set of zones {self.game_info.zones}")
            zone = self.game_state.current_zone
                
        return zone
    def update_current_act_and_zone(self) -> None:
        zone = self.get_current_zone()
        act = self.game_info.zones_acts[zone]
        self.game_state.current_act = act
        self.game_state.current_zone = zone
    def is_heath_and_mana_text_toogled(self) -> tuple[bool, bool]:
        """Checks if the health and mana text is toogled.
        Returns:
            tuple[bool, bool]: [description]
        """
        img = self.take_screenshot(move_mouse_to_safe_zone_before_screenshot=True)

        life_mana_area = img[490: 506, :]
        items = self.get_loot(life_mana_area, self.game_info.window_offsets_life_mana, colors=[ItemQuality.COMMON])
        is_life_toogled = is_item_in_items(items, "Life") or is_item_in_items(items, "ife")
        is_mana_toogled = is_item_in_items(items, "Mana")
        return is_life_toogled, is_mana_toogled
    def toogle_health_and_mana_text(self) -> None:
        """Toogles the text above our health and mana globes
        Above the health globe it says "Life <current amount of hp>" and above the mana globe it says "Mana <current amount of mana>"
        This needs to be toogled as we use to to read our health and mana numbers.
        """
        if self.game_state.is_life_text_toogled is False or self.game_state.is_mana_text_toogled is False: # If it is already toogled then we don't check again
            is_life_text_toogled, is_mana_text_toogled = self.is_heath_and_mana_text_toogled()

            self.hold_key("ctrl")
            for is_text_toogled, box in zip((is_life_text_toogled,                        is_mana_text_toogled),
                                                 (self.game_info.health_globe_screen_location, self.game_info.mana_globe_screen_location)):
                if is_text_toogled is False:
                    point = get_middle_of_box(box)
                    self.move_mouse(point, sleep_after_movement=True)
                    self.click(point, sleep_after_click=True)
            self.release_key("ctrl")

            self.game_state.is_life_text_toogled = True
            self.game_state.is_mana_text_toogled = True
    def get_game_window_screen_location(self, started_game_from_script: bool=False) -> dict[str, int]:
        self.activate_game_window()

        # We identify the location of the game window by searching the entire screen for a specific part of the game window.
        # To avoid the cursor overlapping that part of the game window we first move it out of the way.
        _move_mouse(Point(0, 0))

        no_match = {'top': -600, 'left': -1, 'width': 800, 'height': 600}

        if started_game_from_script:
            image_name = "0.png"
        else:
            image_name = "1.png"

        match_row = np.array(cv2.imread(os.path.join(self.game_info.root_folder, "initialization_data", "start_screens", image_name))[-1, :, 0], dtype=np.int32)

        if started_game_from_script is False:
            game_window_screen_location = get_game_window_location(self.game_info.c_functions, np.array(get_full_screenshot()[:, :, 0], dtype=np.int32), match_row)
        else:
            c = 0
            while (game_window_screen_location := get_game_window_location(self.game_info.c_functions, np.array(get_full_screenshot()[:, :, 0], dtype=np.int32), match_row)) == no_match and c < 20:
                time.sleep(0.05)
                c += 1
        
        if game_window_screen_location == no_match:
            print("Could not find game window. Exiting program!")
            sys.exit()

        return game_window_screen_location
    def is_game_started(self) -> bool:
        return self.get_game_window_handle() != 0
    def get_game_window_handle(self) -> int:
        game_window_handle = None
        if self.game_info.game_version == GameVersion.CLASSIC:
            game_window_handle = win32gui.FindWindow(None, "Diablo II")
            if game_window_handle != 0:
                if win32gui.GetClassName(game_window_handle) == "CabinetWClass":
                    print("You have a folder named 'Diablo II' open in file explorer. Please close this folder as the script cannot find the real Diablo 2 window otherwise")
                    sys.exit()

        elif self.game_info.game_version == GameVersion.RESURRECTED:
            game_window_handle = win32gui.FindWindow(None, "Diablo II: Resurrected")
        return game_window_handle
    def is_game_window_active(self) -> bool:
        return win32gui.GetForegroundWindow() == self.game_info.game_window_handle
    def activate_game_window(self) -> None:
        """Ensures that the Diablo 2 game window is the active window.
           This brings the Diablo 2 game window to the front of the desktop.
           This ensures that any action that uses the gui (screenshots, mouse clicks, keyboard presses) are performed correctly.
        """

        # Check if we are already in the process of activating the game window.
        # This check is needed to avoid an infinite recursive loop as the hold_key and release_key functions that we call from
        # this function will also call this function.
        if self.game_state.is_activating_game_window is False:

            if self.is_game_window_active() is False:
                # Set this so that the hold_key and release_key functions doesn't call this function leading to an infinite loop
                self.game_state.is_activating_game_window = True
                # When running this function from an interactive Python session I somehow cannot bring the window to the front without holding the alt key.
                self.hold_key('alt')
                win32gui.SetForegroundWindow(self.game_info.game_window_handle)
                self.release_key("alt")

                start_loop_time = time.time()
                while self.is_game_window_active() is False:
                    time.sleep(0.01)
                    if time.time() - start_loop_time > 5:
                        raise GameWindowActivationError("Could not activate game window")
                time.sleep(0.04) # Added little extra sleep in case the win32gui is pre emptive. 
                self.game_state.is_activating_game_window = False

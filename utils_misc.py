import ctypes
import os
import random
import sys
from ctypes import c_void_p

import cv2
import numpy as np
import win32api
from mss import mss

from dataclasses_diablo import (Automap, Box, CellItem, Color, Diff,
                                FontSpriteData, Item, MapSpriteData, Pad,
                                Point, TotalMovement, ValidationPixel)
from enums_diablo import (ACT, ItemQuality, WayPointActSelectionPoints,
                          WaypointZone)
from utils_data_loading import (load_font_mapping, load_palettes,
                                read_byte_range)
from utils_data_transformations import flatten_arrays, flatten_indices


def get_middle_point_of_best_matching_template(img: np.ndarray, templates: tuple[np.ndarray]) -> Point:
    if img.dtype != np.uint8:
        img = img.astype(np.uint8)
    best_middle_row = None
    best_middle_col = None
    best_result = 0.0
    for template in templates:
        result = cv2.matchTemplate(img, template, cv2.TM_CCOEFF_NORMED)
        matches = np.where(result > 0.5)
        if len(matches[0]) > 0:
            # Get the top left coordinate of the match
            average_row = np.average(matches[0])
            average_col = np.average(matches[1])

            # Get the middle of the match
            middle_row = int(average_row + (template.shape[0] / 2))
            middle_col = int(average_col + (template.shape[1] / 2))

            best_result_for_this_template = np.max(result)
            if best_result_for_this_template > best_result:
                best_result = best_result_for_this_template
                best_middle_row = middle_row
                best_middle_col = middle_col

    return Point(best_middle_row, best_middle_col)
def get_best_potion(items: list[CellItem], type: str) -> CellItem:
    best_potion = None
    best_potion_ranking = -1

    if type == "healing":
        potions_rankings = {"Minor Healing Potion": 0,
                            "Light Healing Potion": 1,
                            "Healing Potion": 2,
                            "Greater Healing Potion": 3,
                            "Super Healing Potion": 4}
    elif type == "mana":
        potions_rankings = {"Minor Mana Potion": 0,
                            "Light Mana Potion": 1,
                            "Mana Potion": 2,
                            "Greater Mana Potion": 3,
                            "Super Mana Potion": 4}

    for item in items:
        if item.name in potions_rankings:
            if potions_rankings[item.name] > best_potion_ranking:
                best_potion_ranking = potions_rankings[item.name]
                best_potion = item
    return best_potion
def select_random_spot(positions: tuple[np.ndarray, np.ndarray], un_walked_steps: np.ndarray) -> tuple[Point, int]:
    positions_dict: dict = {}
    
    for i in range(len(positions[0])):
        un_walked_steps_count = un_walked_steps[positions[0][i], positions[1][i]]
        if un_walked_steps_count not in positions_dict:
            positions_dict[un_walked_steps_count] = []
        positions_dict[un_walked_steps_count].append((positions[0][i], positions[1][i]))


    keys = list(positions_dict.keys())
    keys.sort(reverse=True)

    positions = []
    best_unwalked_counts = []
    i = 0
    while True:
        if i == len(keys) or keys[0] - keys[i] > 50:
            break
        
        positions.extend(positions_dict[keys[i]])
        for k in range(len(positions_dict[keys[i]])):
            best_unwalked_counts.append(keys[i])

        i += 1

    random_num = random.randint(0, len(positions)-1)
    row = positions[random_num][0]
    col = positions[random_num][1]
    best_unwalked_count = best_unwalked_counts[random_num]

    return(
        Point(
            int(row),
            int(col)
        ),
        int(best_unwalked_count)
    )

def get_window_sums(c_functions: ctypes.CDLL, arr: np.ndarray, num_rows: int, num_cols: int) -> np.ndarray:
    window_sums_temp = np.empty(arr.shape, dtype=np.int32)
    window_sums = np.empty((arr.shape[0] - (num_rows - 1), arr.shape[1] - (num_cols - 1)), dtype=np.int32)

    c_functions.get_window_sums(
        int(arr.shape[0]),
        int(arr.shape[1]),
        c_void_p(arr.ctypes.data),
        num_rows,
        num_cols,
        c_void_p(window_sums_temp.ctypes.data),
        c_void_p(window_sums.ctypes.data)
    )

    return window_sums
def get_optimal_potion_belt_column(current_points: int, max_points: int, belt: np.ndarray, potions: dict[str, int]) -> int:
    missing_points = max_points - current_points
    optimal_potion_column = -1
    optimal_potion_distance_to_max_points = 1000000
    
    for i in range(4):
        potion = belt[-1, i]
        
        points = -1
        if potion in potions:
            points = potions[potion]
        elif potion == "Rejuvenation Potion":
            points = int(max_points * 0.35)
        elif potion == "Full Rejuvenation Potion":
            points = max_points

        if points > 0:
            distance_to_max_points = abs(missing_points - points)
            if distance_to_max_points < optimal_potion_distance_to_max_points:
                optimal_potion_distance_to_max_points = distance_to_max_points
                optimal_potion_column = i
                
    return optimal_potion_column
def validate_page(img: np.ndarray, validation_pixels: tuple[ValidationPixel], valid_count_minimum: int) -> bool:
    valid_count = 0
    for pixel in validation_pixels:
        if img[pixel.point.row, pixel.point.col].tolist() == [pixel.color.blue, pixel.color.green, pixel.color.red]:
            valid_count += 1

    if valid_count >= valid_count_minimum:
        return True
    return False
def get_diamond_indices(size: int, row_offset: int=None, col_offset: int=None, offset_around_middle: bool=True) -> tuple[np.ndarray, np.ndarray]:
    a = np.arange(size)
    b = np.minimum(a,a[::-1])
    rows, cols = np.where((b[:,None]+b)>=(size-1)//2)
    if row_offset is not None:
        if offset_around_middle:
            row_offset -= size//2
        rows += row_offset
    if col_offset is not None:
        if offset_around_middle:
            col_offset -= size//2
        cols += col_offset
    return rows, cols
def get_middle_of_box(box: Box) -> Point:
    return Point(
        int((box.top + box.bottom) / 2),
        int((box.left + box.right) / 2)
    )
def remove_areas_from_image(c_functions: ctypes.CDLL, img: np.ndarray, rectangles: list[Box]) -> None:
    """ Removes areas from an image

    Args:
        img (np.ndarray): A numpy array of minimum 2 dimensions
        rectangles (list[Box]): tuple of Box dataclasses
    """
    for rectangle in rectangles:
        c_functions.empty_rectangle_in_image_array(
            img.shape[0],
            img.shape[1],
            img.shape[2],
            c_void_p(img.ctypes.data),
            rectangle.left,
            rectangle.right,
            rectangle.top,
            rectangle.bottom
            )
def get_zone_waypoint_act_point(zone: WaypointZone, acts_waypointzones: dict[ACT, set[WaypointZone]]) -> Point:
    """Gets the screen position of the act-tab in the waypoint selection menu of the act of a zone.
    """
    for act, act_waypoint_selection_point in zip(ACT, WayPointActSelectionPoints):
        if zone in acts_waypointzones[act]:
            return act_waypoint_selection_point.value

    raise ValueError(f"zone {zone} not in list of waypoints")
def is_in_zone_act(zone: WaypointZone, current_act: ACT, acts_waypointzones: dict[ACT, set[WaypointZone]]) -> bool:
    """Checks if we are in the same act as a zone
    """
    for act in ACT:
        if current_act == act and zone in acts_waypointzones[act]:
            return True
    return False
def get_waypoint_zone_act(zone: WaypointZone, acts_waypoint_zones: dict[ACT, WaypointZone]) -> ACT:
    for act in ACT:
        if zone in acts_waypoint_zones[act]:
            return act
    return None
def update_belt_after_drinking_potion(belt: np.ndarray, potion_belt_column: int) -> None:
    for i in range(belt.shape[0] - 1, 0, -1):
        belt[i, potion_belt_column] = belt[i-1, potion_belt_column]
        
    belt[0, potion_belt_column] = None
    return belt
def get_enum(enum, enum_value, enum_category_name: str):
    for enum_item in enum:
        if enum_item.value == enum_value:
            return enum_item

    # If we reach this point it means that the enum_value is not a valid value.
    enum_values = []
    for enum_item in enum:
        enum_values.append(str(enum_item.value))
    
    enum_values_names = "\n\t".join(enum_values)

    print(f'"{enum_value}" is not a valid "{enum_category_name}"! Please use one of these values instead:\n\t{enum_values_names}\nExiting program!')
    sys.exit()
def remove_prefixes_from_item_name(name: str) -> str:
    prefixes = ("Superior", "Crude")
    for prefix in prefixes:
        if name.startswith(prefix):
            name = name[len(prefix):]
    name = name.strip()
    return name
def get_distances_to_edge(c_functions: ctypes.CDLL, img: np.ndarray, square_roots: np.ndarray) -> np.ndarray:
    num_rows = img.shape[0]
    num_cols = img.shape[1]
    size = int(num_rows * num_cols)

    distances = np.empty(size, dtype=np.float64)
    c_functions.get_distances_to_edge(
        c_void_p(distances.ctypes.data),
        c_void_p(img.ctypes.data),
        c_void_p(square_roots.ctypes.data),
        num_rows,
        num_cols)

    distances.shape = (num_rows, num_cols)
    return distances

def get_path_diagonally(img: np.ndarray, distances: np.ndarray, end: Point) -> tuple[list[Point], list[Point]]:
    current_row = end.row
    current_col = end.col

    current_position_num_steps_to_start_position = img[current_row, current_col]
    path = [Point(current_row, current_col)]
    while current_position_num_steps_to_start_position > 2:

        directions = (
            (1,1),      # Southeast
            (-1,-1),    # Northwest
            (-1,1),     # Northeast
            (1,-1),     # Southwest
            (-1,0),     # North
            (0,1),      # East
            (1,0),      # South
            (0,-1)      # West
            )

        best_improvement = -1
        best_adjacent_row = 0
        best_adjacent_col = 0
        distance_to_edge = -1

        # Check each direction for a coordinate that is closer to the end position.
        for (adjacent_row, adjacent_col) in directions:
            # Walls = 0
            # If the coordinate is not a wall and it is closer to the end position than our current position.
            adjacent_position_num_steps_to_start_position = img[current_row+adjacent_row, current_col+adjacent_col]
            if 0 < adjacent_position_num_steps_to_start_position < current_position_num_steps_to_start_position:
                step_improvement = current_position_num_steps_to_start_position - adjacent_position_num_steps_to_start_position
                distance_to_edge = distances[current_row+adjacent_row, current_col+adjacent_col]

                improvement = step_improvement + distance_to_edge

                if improvement >= best_improvement:
                    best_improvement = improvement
                    best_adjacent_row = adjacent_row
                    best_adjacent_col = adjacent_col

        if best_improvement != -1:
            current_row += best_adjacent_row
            current_col += best_adjacent_col
            current_position_num_steps_to_start_position = img[current_row, current_col]
            path.append(Point(current_row, current_col))

    path.reverse()
    
    path_diff = []
    for i in range(len(path) - 1):
        path_diff.append(
            Point(
                path[i + 1].row - path[i].row,
                path[i + 1].col - path[i].col
            )
        )
    
    return path, path_diff
def get_num_belt_rows(img: np.ndarray) -> int:
    """Get the number of rows in the characters belt. 

    Args:
        img (np.ndarray(shape=(600,800,3), dtype=uint8)): Image of game screen with the belt open.

    Returns:
        int: Number of rows in the characters belt. Can be 1,2,3,4
    """
    for i in range(4):
        row = 465 + 33*i

        is_belt_row = (img[row:row+25, 544, :] == (4,4,4)).all()
        if is_belt_row:
            return 4-i
    return 1
def find_waypoint_on_map(c_functions: ctypes.CDLL, img: np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    waypoint_locations_rows = np.zeros(2, dtype=np.int32)
    waypoint_locations_cols = np.zeros(2, dtype=np.int32)

    c_functions.find_waypoint_on_map(
        c_void_p(waypoint_locations_rows.ctypes.data),
        c_void_p(waypoint_locations_cols.ctypes.data),
        img.shape[0],
        img.shape[1],
        img.shape[2],
        c_void_p(img.ctypes.data),
    )

    return waypoint_locations_rows, waypoint_locations_cols
def get_diff_percentage(c_functions: ctypes.CDLL, arr_1: np.ndarray, arr_2: np.ndarray) -> float:
    diff_count = get_diff_count(c_functions, arr_1, arr_2)
    return diff_count / arr_1.size

def get_diff_count(c_functions: ctypes.CDLL, arr_1: np.ndarray, arr_2: np.ndarray) -> int:
    if arr_1.shape != arr_2.shape:
        raise ValueError("arr_1 and arr_2 must be the same shape!")
    
    diff_count = np.zeros(1, dtype=np.int32)
    c_functions.get_diff_count(
        c_void_p(diff_count.ctypes.data),
        c_void_p(arr_1.ctypes.data),
        c_void_p(arr_2.ctypes.data),
        int(arr_1.size)
    )

    return diff_count[0]
def create_transformation_array(arr: np.ndarray) -> np.ndarray:
    max_pixel_value = 255+1
    transformation_array = np.zeros((max_pixel_value, max_pixel_value, max_pixel_value, 1), dtype=np.uint32)
    values_old = tuple([tuple(arr[:,0]), tuple(arr[:,1]), tuple(arr[:,2])])
    values_new = np.arange(256)
    values_new.shape = (256, 1)

    transformation_array[values_old] = values_new

    return transformation_array
def create_transformation_array_back(arr: np.ndarray) -> np.ndarray:
    max_pixel_value = 255+1
    transformation_array = np.zeros((max_pixel_value, 3), dtype=np.uint32)
    values_old = np.arange(256)
    values_old.shape = (1,256)
    values_new = arr

    transformation_array[values_old] = values_new
    return transformation_array
def transform_image_back(img: np.ndarray, index_array: np.ndarray) -> np.ndarray:
    img = index_array[img]
    img.shape = (600, 800, 3)
    img = img.astype(np.uint8)
    
    return img
def transform_image(img: np.ndarray, index_array: np.ndarray) -> np.ndarray:
    original_shape_0 = img.shape[0]
    original_shape_1 = img.shape[1]
    img.shape = (img.shape[0] * img.shape[1], 3)
    img = index_array[img[:,0], img[:,1], img[:,2]]
    img.shape = (original_shape_0, original_shape_1)
    
    return img
def get_transformation_arrays(root_folder: str, act: str) -> tuple[np.ndarray, np.ndarray]:
    palettes = load_palettes(os.path.join(root_folder, "initialization_data", "palettes"))
    transformation_array = create_transformation_array(palettes[act])
    transformation_array_back = create_transformation_array_back(palettes[act])

    return transformation_array, transformation_array_back
def get_sprite_offsets(
    c_functions: ctypes.CDLL,
    img: np.ndarray,
    window_offsets: np.ndarray,
    middle_indices: np.ndarray,
    sprite_values: np.ndarray,
    sprite_indices: np.ndarray,
    sprite_indices_length_counter: np.ndarray,
    num_sprites: int
    ) -> int:

    result = np.zeros(1, dtype=np.int32)
    c_functions.get_sprite_offsets(
        c_void_p(result.ctypes.data),
        c_void_p(img.ctypes.data),
        c_void_p(window_offsets.ctypes.data),
        c_void_p(middle_indices.ctypes.data),
        c_void_p(sprite_values.ctypes.data),
        c_void_p(sprite_indices.ctypes.data),
        c_void_p(sprite_indices_length_counter.ctypes.data),
        num_sprites)

    return int(result[0])
def find_matching_sprites(
    c_functions: ctypes.CDLL,
    img: np.ndarray,
    window_offsets: np.ndarray,
    sprite_values: np.ndarray,
    sprite_indices: np.ndarray,
    sprite_indices_length_counter: np.ndarray,
    modulus: np.ndarray,
    modulo_indices: np.ndarray,
    num_sprites: int
    ) -> tuple[np.ndarray, np.ndarray]:
    
    max_matches = 15000
    sprite_counts = np.zeros(num_sprites, dtype=np.int32)
    window_indices = np.empty(max_matches, dtype=np.int32)
    c_functions.find_matching_sprites(
        c_void_p(sprite_counts.ctypes.data),
        c_void_p(window_indices.ctypes.data),
        c_void_p(img.ctypes.data),
        c_void_p(window_offsets.ctypes.data),
        c_void_p(sprite_values.ctypes.data),
        c_void_p(sprite_indices.ctypes.data),
        c_void_p(sprite_indices_length_counter.ctypes.data),
        c_void_p(modulus.ctypes.data),
        c_void_p(modulo_indices.ctypes.data),
        num_sprites,
        max_matches)
    window_indices = window_indices[0: np.sum(sprite_counts)]

    return sprite_counts, window_indices
def get_sprite_counts_and_window_indices(
    c_functions: ctypes.CDLL,
    img: np.ndarray,
    transformation_array: np.ndarray,
    window_offsets: np.ndarray,
    middle_indices: np.ndarray,
    sprite_values: np.ndarray,
    sprite_indices: np.ndarray,
    sprite_indices_length_counter: np.ndarray,
    modulus: np.ndarray,
    modulo_indices: np.ndarray,
    num_sprites: int
    ) -> tuple[np.ndarray, np.ndarray]:

    img = transform_image(img, transformation_array)
    img.shape = (600*800)

    best_offset = get_sprite_offsets(c_functions, img, window_offsets, middle_indices, sprite_values, sprite_indices, sprite_indices_length_counter, num_sprites)

    best_window_offsets = window_offsets[best_offset * 13916: (best_offset * 13916) + 13916]

    sprite_counts, window_indices = find_matching_sprites(c_functions, img, best_window_offsets, sprite_values, sprite_indices, sprite_indices_length_counter, modulus, modulo_indices, num_sprites)

    return sprite_counts, window_indices
def draw_map_sprites(
    c_functions: ctypes.CDLL,
    sprite_counts: np.ndarray,
    window_indices: np.ndarray,
    window_offsets: np.ndarray,
    sprite_indices: np.ndarray,
    sprite_indices_length_counter: np.ndarray,
    sprite_indices_length_counter_walkable: np.ndarray,
    sprite_indices_walkable: np.ndarray
    ) -> np.ndarray:

    isolated_map = np.zeros(600 * 800, dtype=np.uint8)
    c_functions.draw_matching_sprites(
        c_void_p(isolated_map.ctypes.data),
        c_void_p(sprite_counts.ctypes.data),
        c_void_p(window_indices.ctypes.data),
        c_void_p(window_offsets.ctypes.data),
        c_void_p(sprite_indices.ctypes.data),
        c_void_p(sprite_indices_length_counter.ctypes.data),
        c_void_p(sprite_indices_walkable.ctypes.data),
        c_void_p(sprite_indices_length_counter_walkable.ctypes.data),
        sprite_counts.shape[0])

    isolated_map.shape = (600, 800)

    return isolated_map
def get_isolated_map(
    c_functions: ctypes.CDLL,
    img: np.ndarray,
    zone_name: str,
    area_mapping: dict[str, MapSpriteData],
    zone_mapping: dict[str, str],
    transformation_array: np.ndarray,
    window_offsets: np.ndarray,
    middle_indices: np.ndarray,
    window_offsets_zero: np.ndarray
    ) -> Automap:

    area_name = zone_mapping[zone_name]
    map_sprite_data = area_mapping[area_name]
    
    sprite_counts, window_indices = get_sprite_counts_and_window_indices(
        c_functions,
        img,
        transformation_array,
        window_offsets,
        middle_indices,
        map_sprite_data.sprite_values_input,
        map_sprite_data.sprite_indices_input,
        map_sprite_data.sprite_indices_length_counter_input,
        map_sprite_data.modulus,
        map_sprite_data.modulo_indices,
        map_sprite_data.num_sprites
        )

    isolated_map = draw_map_sprites(
        c_functions,
        sprite_counts,
        window_indices,
        window_offsets_zero,
        map_sprite_data.sprite_indices_output,
        map_sprite_data.sprite_indices_length_counter_output,
        map_sprite_data.sprite_indices_length_counter_walkable,
        map_sprite_data.sprite_indices_walkable
        )

    rows = np.array(window_indices / 98, dtype=np.int32)
    cols = np.array(window_indices % 98, dtype=np.int32)

    return Automap(sprite_counts, window_indices, rows, cols, isolated_map)
def get_diffs(
    c_functions: ctypes.CDLL,
    map: Automap,
    map_: Automap
    ) -> Diff:

    result = np.zeros(2, dtype=np.int32)
    c_functions.panorama(
        c_void_p(result.ctypes.data),
        c_void_p(map.rows.ctypes.data),
        c_void_p(map.cols.ctypes.data),
        c_void_p(map_.rows.ctypes.data),
        c_void_p(map_.cols.ctypes.data),
        c_void_p(map.sprite_counts.ctypes.data),
        c_void_p(map_.sprite_counts.ctypes.data),
        map.sprite_counts.shape[0]
        )

    row = result[0]
    col = result[1]
    return Diff(row * 4, col * 8)
def pad_file_name(file_name: str, length: int) -> str:
    base_name, extension = os.path.splitext(file_name)
    return "".join(["0" for _ in range(length - len(base_name))]) + base_name + extension
def pad_id(id: str, length: int) -> str:
    return "".join(["0" for _ in range(length - len(id))]) + id
def get_path_coordinates(path_diffs: list[Point]) -> list[Point]:
    # 290, 405 # TODO Calculate if everything is in scale.
    middle_row = 290
    middle_col = 400

    scale = 10 # TODO Am I even sure that this is correct? Could there perhaps be different scaling for vertical and horizontal?

    # scale diffs
    for i in range(len(path_diffs)):
        path_diffs[i].row *= scale
        path_diffs[i].col *= scale
    
    row_min = 0 + (scale + 1)
    row_max = 552 - (scale + 1)
    col_min = 0 + (scale + 1)
    col_max = 800 - (scale + 1)

    coordinates = []

    missing_last_jump = True
    row = 0
    col = 0
    final_index = len(path_diffs) - 1
    for i, path_diff in enumerate(path_diffs):
        row += path_diff.row
        col += path_diff.col

        next_col = col + middle_col
        next_row = row + middle_row

        if (next_row < row_min or
            next_row > row_max or 
            next_col < col_min or 
            next_col > col_max):
            
            coordinates.append(Point(next_row, next_col))

            if i == final_index:
                missing_last_jump = False

            row = 0
            col = 0

    if missing_last_jump:
        next_col = (col) + middle_col
        next_row = (row) + middle_row
        coordinates.append(Point(next_row, next_col))
            
    return coordinates
def bytes_to_color_maps(bytes: np.ndarray) -> list[np.ndarray]:
    color_maps = []
    for i in range(0, len(bytes), 256):
        color_maps.append(np.array(bytes[i:i + 256]))
    return color_maps
def get_symbol_width(img: np.ndarray) -> int:
    col_sums = np.sum(img, axis=0)
    for i, col_sum in enumerate(col_sums):
        if col_sum == 0:
            return i
    return img.shape[1]
def get_font_transformation_arrays(root_folder: str) -> tuple[np.ndarray, np.ndarray]:
    # Get transformation arrays
    transformation_array, transformation_array_back = get_transformation_arrays(root_folder, "ACT1")

    # Some pixels have a different value in some acts.
    # Here we set those values to the same as it is in act 1.
    transformation_array[32,48,52] = 184
    transformation_array[20,32,36] = 42

    # ACT 2 unique:
    transformation_array[92,136,156] = 211
    transformation_array[68,92,104] = 24
    transformation_array[44,64,72] = 56
    transformation_array[72,112,128] = 204
    transformation_array[36,52,60] = 184

    # ACT 3 rune:
    transformation_array[4,28,32] = 43

    # ACT 4 rune:
    transformation_array[20,132,184] = 11
    transformation_array[8,88,152] = 9
    transformation_array[12,104,164] = 81

    # ACT 4 set:
    transformation_array[0,124,0] = 122
    transformation_array[8,100,24] = 122
    transformation_array[20,124,40] = 122

    # ACT 5 set:
    transformation_array[28,196,12] = transformation_array[0,252,24]
    transformation_array[0,132,0] = transformation_array[8,100,24]
    transformation_array[0,52,0] = transformation_array[16,36,4]
    transformation_array[12,72,24] = transformation_array[16,36,4]

    return transformation_array, transformation_array_back
def get_file_paths_and_symbol_indices(root_folder: str) -> tuple[list[str], dict[int, str]]:
    folder_path = os.path.join(root_folder, "initialization_data", "font_symbols")
    font_mapping = load_font_mapping(root_folder)
    file_paths = []

    symbol_indices = {}
    for i, (symbol, symbol_id) in enumerate(font_mapping.items()):
        file_paths.append(os.path.join(folder_path, f"{symbol_id}.png"))
        symbol_indices[i] = symbol

    return file_paths, symbol_indices
def transform_color_map_list_to_dict(color_maps: list[np.ndarray]) -> dict[ItemQuality, np.ndarray]:
    # The indices of the color maps we want to use
    indices = {
        ItemQuality.SET: 2,
        ItemQuality.MAGIC: 3,
        ItemQuality.UNIQUE: 4,
        ItemQuality.GREY: 5,
        ItemQuality.RUNE: 8,
        ItemQuality.RARE: 9
    }
    dict = {}
    for name, index in indices.items():
        dict[name] = color_maps[index]
    return dict
def get_symbols_colors(root_folder: str, symbols_white: list[np.ndarray]) -> dict[ItemQuality, list[np.ndarray]]:
    # Load colormaps and transform the font sprites in to that color.
    # This enables us to check for other colors than white
    bytes = read_byte_range(os.path.join(root_folder, "initialization_data", "palette", "ACT1", "Pal.PL2"), 439847, 3328)
    color_maps = bytes_to_color_maps(bytes)
    color_maps = transform_color_map_list_to_dict(color_maps)

    # Manual changes to set items
    color_maps[ItemQuality.SET][21] = 113
    color_maps[ItemQuality.SET][22] = 113
    color_maps[ItemQuality.SET][25] = 122
    color_maps[ItemQuality.SET][28] = 122
    color_maps[ItemQuality.SET][29] = 122
    color_maps[ItemQuality.SET][30] = 122

    # Create dictionary to hold data for each of the font colors
    color_maps_images = {}

    for color_name, color_map in color_maps.items():
        color_maps_images[color_name] = [color_map[image] for image in symbols_white]

    # The .png files stored in the repository are of the "Common" color, so we do not need to transform them. Instead we add this color directly.
    color_maps_images[ItemQuality.COMMON] = symbols_white

    return color_maps_images

def sort_values(values, sort_indices: np.ndarray) -> list:
    return [values[i] for i in sort_indices]

def load_font_data(root_folder: str) -> FontSpriteData:

    symbol_row_size = 16
    symbol_col_size = 14

    transformation_array, transformation_array_back = get_font_transformation_arrays(root_folder)
    file_paths, symbol_indices = get_file_paths_and_symbol_indices(root_folder)

    symbols_white = [transform_image(cv2.imread(file_path), transformation_array) for file_path in file_paths]
    num_symbols = len(symbols_white)
    symbols_widths = np.array([get_symbol_width(symbol) for symbol in symbols_white], dtype=np.uint32)

    symbols = get_symbols_colors(root_folder, symbols_white)

    # The indices are the same for all the font colors. Only the values are different
    indices = [np.where((symbol != 0)) for symbol in symbols[ItemQuality.COMMON]]

    # Get the indices of the "core" of each font symbol. The core indices are also the same for all the font colors
    indices_core = [np.where((symbol == 31)) for symbol in symbols[ItemQuality.COMMON]]

    # Get how many pixels of the "core" color we have in each font symbol
    symbols_core_color_counts = np.array([row_indices.shape[0] for (row_indices, col_indices) in indices_core])

    # Change the values in the dictionary to only contain the values of the non-zero pixels
    for color_name, symbols_color in symbols.items():
        symbols[color_name] = [image[row_indices, col_indices] for image, (row_indices, col_indices) in zip(symbols_color, indices)]

    # Sort the arrays after how many of the core color each font symbol has.
    sort_indices = symbols_core_color_counts.argsort() # Get the sorted indices.
    symbols_core_color_counts = np.array(sort_values(symbols_core_color_counts, sort_indices))
    indices_core = sort_values(indices_core, sort_indices)
    indices = sort_values(indices, sort_indices)

    for color_name, images_values in symbols.items():
        images_values_sorted = [images_values[i] for i in sort_indices] # Sort the values
        symbols[color_name] = flatten_arrays(images_values_sorted)

    indices_flat = flatten_indices(indices, 800 + symbol_col_size)
    indices_core_flat = flatten_indices(indices_core, 800 + symbol_col_size)

    # Get the total amount of pixels in each font symbol
    indices_counts = np.array([row_indices.shape[0] for (row_indices, col_indices) in indices])

    return FontSpriteData(
        symbol_indices,
        sort_indices,
        symbols_widths,
        transformation_array,
        transformation_array_back,
        symbols_core_color_counts,
        indices_flat,
        indices_counts,
        num_symbols,
        indices_core_flat,
        symbols,
        symbol_row_size,
        symbol_col_size
    )

def get_font_symbols(
    c_functions: ctypes.CDLL,
    img: np.ndarray,
    font_sprite_data: FontSpriteData,
    window_offsets: np.ndarray = None,
    colors: list[ItemQuality] = None
    ) -> list[Item]:

    img = transform_image(img, font_sprite_data.transformation_array)

    # Pad the right side so that we can find symbols near the edge.
    img = np.pad(
        img,
        (
            (0, 0),
            (0, font_sprite_data.symbol_col_size)
        ), mode="constant")

    row_size = img.shape[0]
    col_size = img.shape[1]
    image_size = int(row_size * col_size)

    if window_offsets is None:
        window_offsets = get_window_offsets(
            c_functions,
            0,
            row_size,
            0,
            col_size,
            font_sprite_data.symbol_row_size,
            font_sprite_data.symbol_col_size
            )

    num_windows = window_offsets.shape[0]
    
    img.shape = (row_size * col_size)

    if colors is None:
        colors = font_sprite_data.symbols_colors.keys()

    core_colors = {
        ItemQuality.RUNE: [11],
        ItemQuality.COMMON: [31],
        ItemQuality.RARE: [106],
        ItemQuality.SET: [132],
        ItemQuality.MAGIC: [148],
        ItemQuality.UNIQUE: [211],
        ItemQuality.GREY: [198]
        }

    core_colors_values = []
    for color in colors:
        for color_value in core_colors[color]:
            core_colors_values.append(color_value)

    core_colors = np.array(core_colors_values)
    core_colors_size = core_colors.shape[0]

    img_core = np.empty(image_size, dtype=np.int32)
    c_functions.transform_image(
        c_void_p(img_core.ctypes.data),
        c_void_p(img.ctypes.data),
        c_void_p(core_colors.ctypes.data),
        image_size,
        core_colors_size)

    img_core = img_core.reshape((row_size, col_size))

    window_sums_core = get_window_sums(c_functions, img_core, font_sprite_data.symbol_row_size, font_sprite_data.symbol_col_size)
    window_sums_core.shape = ((window_sums_core.shape[0] * window_sums_core.shape[1]))

    window_sums_core_sorted_indices = np.empty(window_sums_core.shape[0], dtype=np.int32)
    c_functions.argsort(c_void_p(window_sums_core_sorted_indices.ctypes.data), c_void_p(window_sums_core.ctypes.data), window_sums_core.shape[0], font_sprite_data.symbol_row_size * font_sprite_data.symbol_col_size + 1)

    sorted_window_sums_core = window_sums_core[window_sums_core_sorted_indices]
    sorted_window_offsets = window_offsets[window_sums_core_sorted_indices]

    window_offsets_symbol_indices = np.empty(font_sprite_data.num_symbols, dtype=np.int32)
    c_functions.get_window_offsets_symbol_indices(
        c_void_p(window_offsets_symbol_indices.ctypes.data),
        c_void_p(sorted_window_sums_core.ctypes.data),
        c_void_p(font_sprite_data.symbols_core_color_counts.ctypes.data),
        num_windows,
        font_sprite_data.num_symbols)

    # Match core symbols
    max_matches = 100000
    sprite_counts_core = np.zeros(font_sprite_data.num_symbols, dtype=np.int32)
    window_indices_core = np.empty(max_matches, dtype=np.int32)
    c_functions.find_symbols_core(
        c_void_p(sprite_counts_core.ctypes.data),
        c_void_p(window_indices_core.ctypes.data),
        c_void_p(img_core.ctypes.data),
        c_void_p(sorted_window_offsets.ctypes.data),
        c_void_p(font_sprite_data.indices_core_flat.ctypes.data),
        c_void_p(font_sprite_data.symbols_core_color_counts.ctypes.data),
        c_void_p(window_offsets_symbol_indices.ctypes.data),
        font_sprite_data.num_symbols,
        num_windows,
        max_matches)

    items = []
    for color in colors:
        # Match full symbols
        max_matches = 10000
        sprite_counts = np.zeros(font_sprite_data.num_symbols, dtype=np.int32)
        window_indices = np.empty(max_matches, dtype=np.int32)
        c_functions.find_symbols(
            c_void_p(sprite_counts.ctypes.data),
            c_void_p(window_indices.ctypes.data),
            c_void_p(img.ctypes.data),
            c_void_p(sorted_window_offsets.ctypes.data),
            c_void_p(window_indices_core.ctypes.data),
            c_void_p(sprite_counts_core.ctypes.data),
            c_void_p(font_sprite_data.symbols_colors[color].ctypes.data),
            c_void_p(font_sprite_data.indices_flat.ctypes.data),
            c_void_p(font_sprite_data.indices_counts.ctypes.data),
            font_sprite_data.num_symbols,
            num_windows,
            max_matches)
        
        num_sprites_found = np.sum(sprite_counts)
        window_indices = window_indices[0: num_sprites_found]

        # Transform window offsets into rows and cols
        rows = np.array(sorted_window_offsets[window_indices] / col_size, dtype=np.int32)
        cols = np.array(sorted_window_offsets[window_indices] % col_size, dtype=np.int32)

        # Split symbols into rows
        row_symbols = {}
        sprite_index_offset = 0
        for sprite_id, sprite_count in enumerate(sprite_counts):
            for i in range(sprite_index_offset, sprite_index_offset + sprite_count, 1):
                if rows[i] not in row_symbols:
                    row_symbols[rows[i]] = []
                row_symbols[rows[i]].append((font_sprite_data.sort_indices[sprite_id], font_sprite_data.symbols_widths[font_sprite_data.sort_indices[sprite_id]], cols[i]))

            sprite_index_offset += sprite_count

        for row, symbols in row_symbols.items():
            symbols.sort(key=lambda x: x[2])

        for row, symbols in row_symbols.items():
            item_name = []
            previous_col = symbols[0][2]
            previous_width = symbols[0][1]
            for (id, width, col) in symbols:
                if col - (previous_col + previous_width) >= 10:
                    items.append(
                        Item(
                            "".join(item_name),
                            color,
                            Point(
                                int(row + 8),
                                int(previous_col - int((len(item_name) / 2) * 5))
                            )
                        )
                    )

                    item_name = []
                elif col - (previous_col + previous_width) > 3:
                    item_name.append(" ")

                previous_col = col
                previous_width = width

                item_name.append(font_sprite_data.symbol_indices[id])

            items.append(
                Item(
                    "".join(item_name),
                    color,
                    Point(
                        int(row + 8),
                        int(previous_col - int((len(item_name) / 2) * 5))
                    )
                )
            )

    return items
def get_window_offsets(
    c_functions: ctypes.CDLL,
    row_min: int,
    row_max: int,
    col_min: int,
    col_max: int,
    window_row_size: int,
    window_col_size: int
    ) -> np.ndarray:

    num_windows = (row_max - row_min - window_row_size + 1) * (col_max - col_min - window_col_size + 1)
    
    result = np.empty(num_windows, dtype=np.int32)
    c_functions.get_window_offsets_(
        c_void_p(result.ctypes.data),
        row_max - row_min,
        col_max - col_min,
        window_row_size,
        window_col_size)

    return result
def is_item_in_items(items: list[str], item_name: str) -> bool:
    for item in items:
        if item.name == item_name:
            return True
    return False

def adjust_total_movement(total_movement: TotalMovement, point: Point) -> TotalMovement:
    total_movement.up = abs(min(-total_movement.up, point.row))
    total_movement.down = max(total_movement.down, point.row)
    total_movement.left = abs(min(-total_movement.left, point.col))
    total_movement.right = max(total_movement.right, point.col)

    return total_movement
def adjust_point(diff: Diff, point: Point) -> Point:
    point.row += diff.row
    point.col += diff.col

    return point    
def get_pad_stats(total_movement: TotalMovement, point: Point) -> Pad:
    top = 0
    bottom = 0
    left = 0
    right = 0

    if point.row > total_movement.down:
        bottom = point.row - total_movement.down
    
    if point.col > total_movement.right:
        right = point.col - total_movement.right

    if point.row < -total_movement.up:
        top = abs(point.row) - total_movement.up

    if point.col < -total_movement.left:
        left = abs(point.col) - total_movement.left

    return Pad(top, bottom, left, right)
def pad_image(img: np.ndarray, pad: Pad) -> np.ndarray:
    img = np.pad(img, ((pad.top, pad.bottom), (pad.left, pad.right)), mode="constant")
    return img
def get_draw_base_point(total_movement: TotalMovement, point: Point) -> Point:
    row = total_movement.up + point.row
    col = total_movement.left + point.col
    return Point(row, col)
def draw_image(stitched_img: np.ndarray, img: np.ndarray, draw_base_point: Point) -> np.ndarray:
    row_min = draw_base_point.row
    row_max = draw_base_point.row + img.shape[0]

    col_min = draw_base_point.col
    col_max = draw_base_point.col + img.shape[1]

    stitched_img[row_min: row_max, col_min: col_max] = img | stitched_img[row_min: row_max, col_min: col_max]

    return stitched_img
def get_target_position(step_limit: int, automap_mazed: np.ndarray, un_walked_steps: np.ndarray) -> Point:
    limits = []
    thresholds = []
    
    limit = step_limit
    threshold = 0.8
    while limit >= 50:
        limits.append(limit)
        thresholds.append(threshold)
        limit -= 45
        threshold += 0.05
        
    limit = step_limit + 100
    threshold = 0.55
    while limit <= 600:
        limits.append(limit)
        thresholds.append(threshold)
        limit += 100
        threshold -= 0.07

    # Select random place to go to within limit distance
    for i, limit in enumerate(limits):
        positions = np.where(automap_mazed == limit)
        point, best_unwalked_count = select_random_spot(positions, un_walked_steps)
        
        if (best_unwalked_count / limit) > thresholds[i]:
            break

    return point
def get_previous_base_point(current_base_point: Point, diff: Diff) -> Point:
    if diff.row >= 0:
        row = current_base_point.row - diff.row
    else:
        row = current_base_point.row + abs(diff.row)

    if diff.col >= 0:
        col = current_base_point.col - diff.col
    else:
        col = current_base_point.col + abs(diff.col)

    return Point(row, col)
def get_full_screenshot() -> np.ndarray:
    with mss() as sct:
        return np.array(sct.grab(sct.monitors[0]), dtype=np.uint8)
def get_game_window_location(c_functions: ctypes.CDLL, screen: np.ndarray, match_row: np.ndarray) -> dict[str, int]:
    num_rows = int(screen.shape[0])
    num_cols = int(screen.shape[1])

    result = np.empty(2, dtype=np.int32)
    c_functions.get_game_window_location(
        num_rows,
        num_cols,
        c_void_p(screen.ctypes.data),
        c_void_p(match_row.ctypes.data),
        c_void_p(result.ctypes.data)
        )
    row = int(result[0] - 599)
    col = int(result[1])

    game_window_screen_location = {'top': row, 'left': col, 'width': 800, 'height': 600}
    return game_window_screen_location
def _move_mouse(point: Point) -> None:
    win32api.SetCursorPos((point.col, point.row))
def transform_image_from_resurrected_to_classic(c_functions: ctypes.CDLL, large_img: np.ndarray) -> np.ndarray:
    """
    In Diablo2 Classic the native resolution is 600x800 (600 rows and 800 columns)
    This bot is originally built to play Diablo 2 classic at 600x800 resolution.
    Diablo2 Resurrected does not have a 600x800 resolution option.
    Diablo2 Resurrected does though have a 1200x1600 resolution option which is a direct upscaling of the original 600x800.
    This means that when we run Diablo2 Resurrected with classic graphics at 1200x1600 resolution, every pixel is duplicated
    4 times like this:
    Diablo2 Classic with 600x800:
    1,2
    3,4
    Diablo2 Resurrected with 1200x1600:
    1,1,2,2
    1,1,2,2
    3,3,4,4
    3,3,4,4
    
    In this function we remove all rows and columns with odd index numbers, the result of which transforms the image back to the
    original format, which then lets the bot work as before without further changes :)
    """
    small_img = np.empty(
        (
            int(large_img.shape[0] / 2),
            int(large_img.shape[1] / 2),
            large_img.shape[2],
        ), dtype=np.uint8)

    c_functions.transform_image_from_resurrected_to_classic(
        large_img.shape[0],
        large_img.shape[1],
        large_img.shape[2],
        c_void_p(large_img.ctypes.data),
        c_void_p(small_img.ctypes.data)
    )

    return small_img
def fill_maze(c_functions: ctypes.CDLL, automap_white: np.ndarray, walked: np.ndarray, limit: int, start: Point, use_wide_start: bool = False, wide_start_size: int = 0) -> tuple[np.ndarray, np.ndarray]:
    num_rows = automap_white.shape[0]
    num_cols = automap_white.shape[1]
    size = num_rows * num_cols

    use_wide_start = 1 if use_wide_start else 0
    
    map_filled = np.empty(size, dtype=np.int32)
    un_walked_steps = np.empty(size, dtype=np.int32)

    c_functions.fill_maze(
        num_rows,
        num_cols,
        c_void_p(map_filled.ctypes.data),
        c_void_p(un_walked_steps.ctypes.data),
        c_void_p(automap_white.ctypes.data),
        c_void_p(walked.ctypes.data),
        use_wide_start,
        wide_start_size,
        limit,
        int(start.row),
        int(start.col)
        )

    map_filled = map_filled.reshape((num_rows, num_cols))
    un_walked_steps = un_walked_steps.reshape((num_rows, num_cols))

    return map_filled, un_walked_steps
def create_validation_pixels(img: np.ndarray, offset_row: int, offset_col: int) -> list[ValidationPixel]:
    validation_pixels = []
    for row in range(1, img.shape[0], 10):
        for col in range(1, img.shape[1], 10):
            validation_pixels.append(
                ValidationPixel(
                    Point(
                        offset_row + row,
                        offset_col + col,
                    ),
                    Color(
                        img[row, col, 0],
                        img[row, col, 1],
                        img[row, col, 2]
                    )
                )
            )

    return validation_pixels

def detect_monsters(monster_detection: ctypes.CDLL, img: np.ndarray, img_transformed: np.ndarray, img_transformed_2: np.ndarray, skill_bar_row: int, draw_matches: int):
    max_num_matches = 100
    match_rows = np.empty(max_num_matches, dtype=np.int32)
    match_cols = np.empty(max_num_matches, dtype=np.int32)
    match_sprite_ids = np.empty(max_num_matches, dtype=np.int32)
    match_palette_ids = np.empty(max_num_matches, dtype=np.int32)
    num_matches_found = np.zeros(1, dtype=np.int32)

    # TODO There is a memory error: OSError: exception: access violation reading 0x000001C03F0F8000

    try:
        monster_detection.match_sprites(
            int(skill_bar_row),
            int(img_transformed.shape[1]),
            c_void_p(img.ctypes.data),
            c_void_p(img_transformed.ctypes.data),
            c_void_p(img_transformed_2.ctypes.data),
            c_void_p(match_rows.ctypes.data),
            c_void_p(match_cols.ctypes.data),
            c_void_p(match_sprite_ids.ctypes.data),
            c_void_p(match_palette_ids.ctypes.data),
            c_void_p(num_matches_found.ctypes.data),
            max_num_matches,
            draw_matches
        )
    except:
        pass
        
    num_matches_found = num_matches_found[0]

    match_rows = match_rows[:num_matches_found]
    match_cols = match_cols[:num_matches_found]
    match_sprite_ids = match_sprite_ids[:num_matches_found]
    match_palette_ids = match_palette_ids[:num_matches_found]

    return match_rows, match_cols
def is_point_in_area(p: Point, a: Box) -> bool:
    return (
        p.row >= a.top and
        p.row <= a.bottom and
        p.col >= a.left and
        p.col <= a.right
    )
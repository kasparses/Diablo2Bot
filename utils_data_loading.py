from dataclasses_diablo import Color, Point, ValidationPixel
import numpy as np
import os
import cv2

def read_byte_range(file_path: str, start_byte: int, byte_count: int) -> np.ndarray:
    dtype = np.dtype('B')
    with open(file_path, "rb") as f:
        bytes = np.fromfile(f, dtype, offset=start_byte, count=byte_count)
    return bytes

def load_palettes(folder_path: str) -> dict[str, np.ndarray]:
    files = {}
    for file_name in os.listdir(folder_path):
        file_path = os.path.join(folder_path, file_name)
        img = cv2.imread(file_path)
        img.shape = (256,3)
        files[file_name[0:-4]] = img

    return files

def load_validation_pixels_files(folder_path: str) -> dict[str, list[ValidationPixel]]:
    validation_pixels = {}

    for file_name in os.listdir(folder_path):
        file_path = os.path.join(folder_path, file_name)
        file_name_without_extension = os.path.splitext(file_name)[0]

        validation_pixels[file_name_without_extension] = load_validation_pixels_file(file_path)

    return validation_pixels

def load_validation_pixels_file(file_path: str) -> list[ValidationPixel]:
    
    validation_pixels = []

    with open(file_path, "r") as file:
        lines = file.readlines()

        for line in lines:
            line_split = line.split(",")
            validation_pixels.append(
                ValidationPixel(
                    Point(
                        int(line_split[0]),
                        int(line_split[1]),
                    ),
                    Color(
                        int(line_split[2]),
                        int(line_split[3]),
                        int(line_split[4].replace("\n", ""))
                    )
                )
            )

    return validation_pixels

def read_level_sprites_table(file_path: str, num_sprites: int) -> dict[str, tuple[list[int], list[int]]]:
    table_sprites = {}
    with open(file_path, "r") as file:
        lines = [line[:-1] for line in file.readlines()] # Read lines and remove \n char

        for line in lines[1:]:
            columns = line.split(",")
            name = columns[0]
            has_sprite_ids = [i for i, has_sprite_id in enumerate(columns[1: num_sprites + 1]) if has_sprite_id == "1"]

            ignore_sprite_ids = []
            for sprite_id in has_sprite_ids:
                if columns[1 + sprite_id + num_sprites] == "1":
                    ignore_sprite_ids.append(1)
                else:
                    ignore_sprite_ids.append(0)
            
            table_sprites[name] = (has_sprite_ids, ignore_sprite_ids)

    return table_sprites

def load_font_mapping(root_folder: str) -> dict[str, str]:
    font_mapping = {}
    with open(os.path.join(root_folder, "initialization_data", "font_mapping.txt"), "r") as file:
        lines = [line[:-1] for line in file.readlines()]
        for line in lines:
            id, symbol, *rest = line.split(":")
            if id:
                font_mapping[symbol] = id
    return font_mapping

def load_fixed_routes(root_folder: str) -> dict[str, list[Point]]:
    # Read files containing fixed routes         
    routes = {}
    folder_path = os.path.join(root_folder, "initialization_data", "fixed_routes")

    for file_name in os.listdir(folder_path):
        file_path = os.path.join(folder_path, file_name)
        file_name_without_extension = os.path.splitext(file_name)[0]
        
        routes[file_name_without_extension] = []
        with open(file_path, "r") as file:
            for line in file.readlines():
                line = line.replace("\n", "")
                row, col = line.split(',')
                routes[file_name_without_extension].append(Point(int(row), int(col)))

    return routes
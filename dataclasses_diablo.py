from dataclasses import dataclass
from typing import Any

import numpy as np


@dataclass
class Health_Mana:
    health_current: int
    health_max: int
    mana_current: int
    mana_max: int

@dataclass
class Point:
    row: int
    col: int

@dataclass
class Item:
    name: str
    quality: Any # ItemQuality
    point: Point

@dataclass
class Box:
    left: int
    right: int
    top: int
    bottom: int

@dataclass()
class CellItem:
    point: Point
    name: str

@dataclass
class Automap:
    sprite_counts: np.ndarray
    window_indices: np.ndarray
    rows: np.ndarray
    cols: np.ndarray
    image: np.ndarray

@dataclass
class Diff:
    row: int
    col: int

@dataclass
class TotalMovement:
    up: int
    down: int
    left: int
    right: int

@dataclass
class Pad:
    top: int
    bottom: int
    left: int
    right: int

@dataclass
class Color:
    blue: int
    green: int
    red: int

@dataclass
class ValidationPixel:
    point: Point
    color: Color

@dataclass
class MapSpriteData:
    num_sprites: int
    sprite_indices_input: np.ndarray
    sprite_values_input: np.ndarray
    sprite_indices_length_counter_input: np.ndarray
    sprite_indices_output: np.ndarray
    sprite_indices_length_counter_output: np.ndarray
    sprite_indices_length_counter_walkable: np.ndarray
    sprite_indices_walkable: np.ndarray
    modulus: np.ndarray
    modulo_indices: np.ndarray

@dataclass
class FontSpriteData:
    symbol_indices: dict[int, str]
    sort_indices: np.ndarray
    symbols_widths: np.ndarray
    transformation_array: np.ndarray
    transformation_array_back: np.ndarray
    symbols_core_color_counts: np.ndarray
    indices_flat: np.ndarray
    indices_counts: np.ndarray
    num_symbols: int
    indices_core_flat: np.ndarray
    symbols_colors: dict[Any, np.ndarray] # dict[ItemQuality, np.ndarray]
    symbol_row_size: int
    symbol_col_size: int
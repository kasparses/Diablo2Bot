import numpy as np

def flatten_sprite_values(values: list[np.ndarray], ignore_sprite: list[int]) -> np.ndarray:
    num_pixels = sum([values.shape[0] for values in values])
    sprite_values = np.zeros(num_pixels, dtype=np.uint32)
    offset = 0
    for i, values_ in enumerate(values):
        if ignore_sprite[i] == 0:
            num_values = values_.shape[0]

            sprite_values[offset: offset + num_values] = values_
            offset += num_values
    
    return sprite_values

def flatten_sprite_indices(indices: list[tuple[np.ndarray, np.ndarray]], ignore_sprite: list[int]) -> np.ndarray:
    num_pixels = sum([indices_[0].shape[0] for indices_ in indices])
    sprite_indices = np.zeros(num_pixels, dtype=np.uint32)

    offset = 0
    for i, (row_indices, col_indices) in enumerate(indices):
        if ignore_sprite[i] == 0:
            flat_indices = (row_indices * 800) + col_indices

            num_values = row_indices.shape[0]

            sprite_indices[offset: offset + num_values] = flat_indices
            offset += num_values

    return sprite_indices

def get_modulo_indices(indices: list[tuple[np.ndarray, np.ndarray]], ignore_sprite: list[int]) -> np.ndarray:
    num_pixels = sum([indices_[0].shape[0] for indices_ in indices])
    modulo_indices = np.zeros(num_pixels, dtype=np.uint32)

    offset = 0
    for i, (row_indices, col_indices) in enumerate(indices):
        if ignore_sprite[i] == 0:

            flat_indices = ((row_indices % 4) * 8) + col_indices % 8
            
            num_values = row_indices.shape[0]
            modulo_indices[offset: offset + num_values] = flat_indices
            offset += num_values

    return modulo_indices
def get_sprite_indices_lengths(indices: list[tuple[np.ndarray, np.ndarray]], ignore_sprite: list[int]) -> np.ndarray:
    sprite_indices_length = np.zeros(len(indices), dtype=np.uint32)
    for i in range(len(indices)):
        if ignore_sprite[i] == 0:
            sprite_indices_length[i] = indices[i][0].shape[0]

    return sprite_indices_length

def flatten_indices(indices: list[tuple[np.ndarray, np.ndarray]], width: int) -> np.ndarray:
    num_total_indices = sum([row_indices.shape[0] for (row_indices, col_indices) in indices])
    flat_indices = np.zeros(num_total_indices, dtype=np.uint32)

    offset = 0
    for row_indices, col_indices in indices:
        num_indices = row_indices.shape[0]
        flat_indices[offset: offset + num_indices] = (row_indices * width) + col_indices
        offset += num_indices

    return flat_indices
    
def flatten_arrays(arrays: list[np.ndarray]) -> np.ndarray:
    total_size = sum([array.shape[0] for array in arrays])
    flat_array = np.zeros(total_size, dtype=np.uint32)

    offset = 0
    for array in arrays:
        size = array.shape[0]
        flat_array[offset: offset + size] = array
        offset += size

    return flat_array
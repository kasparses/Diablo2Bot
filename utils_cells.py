import numpy as np

from dataclasses_diablo import CellItem, Point

def get_cell_information(
    img: np.ndarray,
    num_rows: int,
    num_cols: int,
    cell_group_top_left_point: Point,
    cell_row_gap: int = 0,
    cell_col_gap: int = 0
    ) -> np.ndarray:

    cells: np.ndarray = np.zeros((num_rows, num_cols, 27,27,3))
    
    for cell_row_num in range(num_rows):
        for cell_col_num in range(num_cols):
            cell_top_left_row = cell_group_top_left_point.row + (cell_row_num * 29 + cell_row_num * cell_row_gap)
            cell_top_left_col = cell_group_top_left_point.col + (cell_col_num * 29 + cell_col_num * cell_col_gap)
            
            cell = img[
                cell_top_left_row + 2 : cell_top_left_row + 29,
                cell_top_left_col + 2 : cell_top_left_col + 29
                ]

            cells[cell_row_num, cell_col_num] = cell

    cells = cells.astype(np.int32)
    return cells


def search_cells_for_items(cells: np.ndarray, items_indices: list[tuple[np.ndarray, np.ndarray]], items_values: list[np.ndarray], items_position_names: dict[int, str]) -> list[CellItem]:
    found_items = []
    for i in range(len(items_indices)): # Loop through all known consumable items
        # Get the indices of the item and ignore the background indices.
        cells_item_indices = cells[:, :, items_indices[i][0], items_indices[i][1], :]

        # Compare the value of the item to the values in our belt cells. 
        comparison: np.ndarray = (cells_item_indices[:,:,:,0] == items_values[i][:,0]) & \
                                 (cells_item_indices[:,:,:,1] == items_values[i][:,1]) & \
                                 (cells_item_indices[:,:,:,2] == items_values[i][:,2])

        num_pixels_to_be_compared = comparison.shape[2]
        comparison = comparison.reshape((comparison.shape[0] * comparison.shape[1], comparison.shape[2]))

        # Check if any of our cells matched the consumable item
        num_identical_pixels = np.count_nonzero(comparison, axis=(1))
        for k in range(num_identical_pixels.shape[0]):
            if num_identical_pixels[k] / num_pixels_to_be_compared >= 0.8:
                found_items.append(
                    CellItem(
                        Point(
                            int(k / cells.shape[1]),
                            k % cells.shape[1],
                        ),
                        items_position_names[i]
                        )
                    )

    return found_items

def are_cells_filled(img: np.ndarray, num_rows: int, num_cols: int, top_left_point: Point, empty_cells: np.ndarray) -> np.ndarray:
    cells = get_cell_information(img, num_rows, num_cols, top_left_point)
    cell_empty_count = np.count_nonzero(((cells == empty_cells)), axis=(2,3,4))
    return np.where(cell_empty_count > 2000, False, True)

def get_item_size(area_before_item_pickup: np.ndarray, area_after_item_pickup: np.ndarray) -> tuple[int, int]:
    changed_rows = set()
    changed_cols = set()
    for row in range(area_before_item_pickup.shape[0]):
        for col in range(area_before_item_pickup.shape[1]):
            if area_after_item_pickup[row, col] != area_before_item_pickup[row, col]:
                changed_rows.add(row)
                changed_cols.add(col)

    return len(changed_rows), len(changed_cols)

def find_item_placement_in_cell_area(cell_area: np.ndarray, item_row_size: int, item_col_size: int) -> tuple[int, int]:
    for row in range(cell_area.shape[0] - item_row_size +1):
        for col in range(cell_area.shape[1] - item_col_size +1):

            is_space = True
            for item_row in range(item_row_size):
                for item_col in range(item_col_size):
                    if cell_area[row + item_row, col + item_col]:
                        is_space = False

            if is_space:
                return row, col
    return -1, -1
from functions import Functions

import cv2
import os
import numpy as np


def test_initialization_files(functions: Functions):
    cells_empty_inventory = cv2.imread(os.path.join(functions.game_info.root_folder, "tests", "data", "cells_empty_inventory.png"))
    cells_empty_stash = cv2.imread(os.path.join(functions.game_info.root_folder, "tests", "data", "cells_empty_stash.png"))

    inventory = functions._get_inventory(cells_empty_inventory)
    empty_inventory = np.zeros(shape=(4,10), dtype=np.bool8)
    assert (inventory == empty_inventory).all()

    stash = functions._get_stash(cells_empty_stash)
    empty_stash = np.zeros(shape=(8,6), dtype=np.bool8)
    assert (stash == empty_stash).all()
    





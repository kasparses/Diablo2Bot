# Generate loot file
import os
import json
import argparse
from enums_diablo import ItemQuality

def create_item_filter(item_categories: dict, wanted_items: dict):
    item_filter: dict = {}

    # Create item filter with all items set to False.
    for item_type in item_categories: # Helms, Daggers, ...
        for item_difficulty in item_categories[item_type]: # Normal, Exceptional, Elite
            for item_name in item_categories[item_type][item_difficulty]:
                row_size: int = item_categories[item_type][item_difficulty][item_name]['row_size']
                col_size: int = item_categories[item_type][item_difficulty][item_name]['col_size']

                item_desire = {
                    ItemQuality.GREY.value: False,
                    ItemQuality.COMMON.value: False,
                    ItemQuality.MAGIC.value: False,
                    ItemQuality.RARE.value: False,
                    ItemQuality.SET.value: False,
                    ItemQuality.UNIQUE.value: False,
                    ItemQuality.RUNE.value: False,
                    "row_size": row_size,
                    "col_size": col_size
                    }

                item_filter[item_name] = item_desire

    # Adjust item filter
    for item_type in wanted_items: # Helms, Daggers, ...
        for item_difficulty in wanted_items[item_type]: # Normal, Exceptional, Elite
            for item_name in item_categories[item_type][item_difficulty]:
                for wanted_quality in wanted_items[item_type][item_difficulty]:
                    item_filter[item_name][wanted_quality] = True

    return item_filter

parser = argparse.ArgumentParser(description='Item filter creator program')
parser.add_argument("profile_name")
args = parser.parse_args()
profile_name: str = args.profile_name

root_folder: str = os.path.dirname(os.path.abspath(__file__))

# Load the item_categories json file
with open(os.path.join(root_folder, "initialization_data", "items", "item_categories_names_sizes.json")) as file:
    item_categories: dict = json.load(file)

# Load the loot_filter_creator json file
with open(os.path.join(root_folder, "profiles", profile_name, "loot_profile_creator.json")) as file:
    wanted_items: dict = json.load(file)

item_filter = create_item_filter(item_categories, wanted_items)

# Write the item_filter json file
with open(os.path.join(root_folder, "profiles", profile_name, "item_filter.json"), 'w') as file_path:
    json.dump(item_filter, file_path, indent=4)
import argparse
import json
import os

from enums_diablo import Skill
from functions import Functions
from game_info import GameInfo
from game_state import Buff, GameState
from utils_misc import get_enum
from utils_validations import validate_profile_bool, validate_profile_integer


def load_variables(profile_name: str, **kwargs) -> tuple[Functions, GameInfo, GameState]:
    # Load the profile json file
    root_folder: str = os.path.dirname(os.path.abspath(__file__))
    with open(os.path.join(root_folder, "profiles", profile_name, "profile.json"), encoding="utf8") as file:
        profile: dict = json.load(file)
    profile["profile_name"] = profile_name

    game_info = GameInfo(profile, **kwargs)

    game_state: GameState = GameState(game_info)
    functions: Functions = Functions(game_info, game_state)
    game_state.functions = functions

    buffs: list[Buff] = []
    for buff in profile["buffs"]:
        validate_profile_integer(buff["duration"], "buff_duration")
        validate_profile_bool(buff["only_castable_on_secondary_weaponset"], "buff_only_castable_on_secondary_weaponset")

        buffs.append(
            Buff(
                functions,
                get_enum(
                    Skill,
                    buff["skill"],
                    "skill"
                ),
            buff["duration"],
            only_castable_on_secondary_weaponset = buff["only_castable_on_secondary_weaponset"]
            )
        )


    game_state.buffs = buffs

    return functions, game_info, game_state


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Diablo 2 bot runner program')
    parser.add_argument("profile_name")
    args = parser.parse_args()
    profile_name: str = args.profile_name

    functions, game_info, game_state = load_variables(profile_name)
    functions.start_diablo2()
    functions.main_loop()

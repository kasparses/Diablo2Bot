import time
from typing import Union

import numpy as np

from dataclasses_diablo import Box, Health_Mana, Point
from enums_diablo import ACT, Skill, Weaponset
from game_info import GameInfo


class GameState:
    def __init__(self, game_info: GameInfo) -> None:

        # Set state variables that persists through games.
        # These variables will be the same in all the games that we enter in this session.
        self.game_info = game_info
        self.buffs: list[Buff] = []
        self.functions = None
        self.is_life_text_toogled: bool = False
        self.is_mana_text_toogled: bool = False
        self.held_keys: set = set() # A set of which keys we are currently holding.
        self.is_buying_potions: bool = False

        self.is_first_run: bool = True

        self.reserved_inventory_cells = None
        self.belt = None

        self.inventory: np.ndarray = np.empty([4, 10], dtype=object)

        self.initialize()
    def set_belt(self, num_belt_rows: int) -> None:
        self.belt: np.ndarray = np.empty([num_belt_rows, 4], dtype=object)
    def initialize(self) -> None:
        """This function sets state variables that are unique for every game.
           They get reset every time we enter a new game.
        """
        self.current_act: Union[ACT, None] = None
        self.current_zone: Union[str, None] = None

        self.health_mana: Health_Mana = Health_Mana(-1, -1, -1, -1)

        self.healing_potion_last_consumed_time: float = -1.0
        self.mana_potion_last_consumed_time: float = -1.0

        self.last_updated_belt_time: float = -1.0

        self.last_skill_used_time: float = 0.0
        self.last_skill_used_cooldown: float = 0.0
        self.active_skill: Skill = None
        self.active_skill_secondary_weaponset: Skill = None
        self.active_weaponset: Weaponset = Weaponset.PRIMARY

        self.mouse_point: Point = Point(0, 0)

        self.ui_elements_toogled = {}

        self.is_in_game: bool = False
        self.is_checking_health: bool = False
        self.is_activating_game_window: bool = False
        self.is_in_monster_area: bool = False
        
        self.reset_buffs()
    def re_cast_buffs_if_expired(self) -> None:
        for buff in self.buffs:
            if buff.is_expired():
                buff.refresh(self)
        if self.active_weaponset == Weaponset.SECONDARY:
            self.functions.do_action_and_validate(self.functions.swap_weapons, [self.game_info.validation_pixels["primary_weapons"]], [20])
            self.active_weaponset = Weaponset.PRIMARY
    def reset_buffs(self) -> None:
        for buff in self.buffs:
            buff.last_updated_time = None

class Buff:
    def __init__(self, functions, skill: Skill, duration: int, only_castable_on_secondary_weaponset: bool=False) -> None:
        self.functions = functions
        self.skill: Skill = skill
        self.duration: int = duration # If the time since the last update is above the refresh_limit then we fetch the value again.
        self.only_castable_on_secondary_weaponset: bool = only_castable_on_secondary_weaponset
        self.last_updated_time: Union[float, None] = None
    def is_expired(self) -> bool:
        is_expired: bool = False
        if self.last_updated_time is None:
            is_expired = True
        else:
            current_time: float = time.time()
            if current_time - self.last_updated_time > self.duration:
                is_expired = True
            
        return is_expired
    def refresh(self, game_state: GameState) -> None:
        if self.only_castable_on_secondary_weaponset:
            if game_state.active_weaponset == Weaponset.PRIMARY:
                self.functions.swap_weapons()
        else:
            if game_state.active_weaponset == Weaponset.SECONDARY:
                self.functions.swap_weapons()

        self.functions.use_skill(Point(300, 300), self.skill)
        self.last_updated_time = time.time()
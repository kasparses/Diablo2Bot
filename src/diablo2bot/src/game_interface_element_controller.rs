use std::collections::{HashMap, HashSet};

use crate::{
    enums::{errors::BotError, game_interface_element::GameInterfaceElement, state::State},
    game::Game,
    image::Image,
    output_controller::{map_key_code, OutputController},
    state_validator::{wait_for_enum_state, wait_while_in_enum_state},
    utils::sleep_frame,
};

enum EndState {
    Activated,
    Deactivated,
}

pub struct GameInterfaceElementController {
    active_elements: HashSet<GameInterfaceElement>,
    game_interface_element_keybindings: HashMap<GameInterfaceElement, enigo::Key>,
}

impl GameInterfaceElementController {
    pub fn new(
        game_interface_element_string_keybindings: &HashMap<GameInterfaceElement, String>,
    ) -> Self {
        let mut active_elements = HashSet::new();
        active_elements.insert(GameInterfaceElement::Portraits); // Portraits are enabled by default every time we enter a new game.

        let mut game_interface_element_keybindings = HashMap::new();

        for (game_interface_action, keybinding) in game_interface_element_string_keybindings {
            let key = map_key_code(keybinding);
            game_interface_element_keybindings.insert(*game_interface_action, key);
        }

        Self {
            active_elements,
            game_interface_element_keybindings,
        }
    }

    fn toogle_element(
        &self,
        element: GameInterfaceElement,
        end_state: &EndState,
        output_controller: &mut OutputController,
    ) {
        match element {
            GameInterfaceElement::Items => match end_state {
                EndState::Activated => {
                    output_controller.hold_key(self.game_interface_element_keybindings[&element]);
                }
                EndState::Deactivated => {
                    output_controller
                        .release_key(self.game_interface_element_keybindings[&element]);
                }
            },
            _ => {
                output_controller.click_key(self.game_interface_element_keybindings[&element]);
            }
        }
    }

    pub fn activate_element(
        g: &mut Game,
        element: GameInterfaceElement,
    ) -> Result<Option<Image>, BotError> {
        if g.game_interface_element_controller
            .active_elements
            .contains(&element)
        {
            return Ok(None);
        }

        g.game_interface_element_controller.toogle_element(
            element,
            &EndState::Activated,
            &mut g.output_controller,
        );

        match element {
            GameInterfaceElement::Inventory => {
                g.game_interface_element_controller
                    .active_elements
                    .insert(element);

                return Ok(Some(wait_for_enum_state(
                    g,
                    State::InventoryOpen,
                    g.bot_settings.max_frames_to_wait_for_ui_action,
                )?));
            }
            GameInterfaceElement::Belt => {
                g.game_interface_element_controller
                    .active_elements
                    .insert(element);

                return Ok(Some(wait_for_enum_state(
                    g,
                    State::BeltOpen,
                    g.bot_settings.max_frames_to_wait_for_ui_action,
                )?));
            }
            _ => {
                sleep_frame();
            }
        }

        g.game_interface_element_controller
            .active_elements
            .insert(element);

        Ok(None)
    }

    pub fn deactivate_element(
        g: &mut Game,
        element: GameInterfaceElement,
    ) -> Result<Option<Image>, BotError> {
        if !g
            .game_interface_element_controller
            .active_elements
            .contains(&element)
        {
            return Ok(None);
        }

        g.game_interface_element_controller.toogle_element(
            element,
            &EndState::Deactivated,
            &mut g.output_controller,
        );

        match element {
            GameInterfaceElement::Inventory => {
                wait_while_in_enum_state(g, State::InventoryOpen, 100)?;
            }
            GameInterfaceElement::Belt => {
                wait_while_in_enum_state(g, State::BeltOpen, 100)?;
            }
            _ => {
                sleep_frame();
            }
        }

        g.game_interface_element_controller
            .active_elements
            .remove(&element);

        Ok(None)
    }
}

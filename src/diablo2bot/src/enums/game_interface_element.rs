use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum GameInterfaceElement {
    Automap,
    Inventory,
    Belt,
    Items,
    Portraits,
    Chat,
}

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Quality {
    Grey,
    Common,
    Magic,
    Rare,
    Set,
    Unique,
    Rune,
}

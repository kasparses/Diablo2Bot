use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum OperatingSystem {
    Windows,
    Linux,
}

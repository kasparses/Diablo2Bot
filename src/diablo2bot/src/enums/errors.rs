// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html

use std::{fmt, string::FromUtf8Error};
use strum::IntoEnumIterator;

use crate::{health_mana::Points, image::Image, state_validator::is_in_enum_state};

use super::state::State;

#[derive(Debug)]
pub enum BotError {
    HealthPointsUnderHardLimit(HealthPointsUnderHardLimitError),
    LowHealthManaAndNoPotionInBelt(LowHealthManaAndNoPotionInBeltError),
    CharacterHasDied(CharacterHasDiedError),
    WrongGameState(WrongGameStateError),
    LowInventorySpace(LowInventorySpaceError),
    MovedToTownZone(MovedToTownZoneError),
    CouldNotGetPath(CouldNotGetPathError),
    ArchiveError(ArchiveError),
}

#[derive(Debug)]
pub enum ArchiveError {
    IoError(std::io::Error),
    MpqFileFromUtf8Error(MpqFileFromUtf8Error),
}

#[derive(Debug)]
pub struct HealthPointsUnderHardLimitError {
    pub points: Points,
    pub point_limit_hard: f32,
}

#[derive(Debug)]
pub struct LowHealthManaAndNoPotionInBeltError;

#[derive(Debug)]
pub struct CharacterHasDiedError;

#[allow(dead_code)] // Allow dead code as the field is only used for logging
#[derive(Debug)]
pub struct WrongGameStateError {
    current_game_states: Vec<State>,
}

impl WrongGameStateError {
    pub fn new(img: &Image) -> Self {
        let current_game_states = State::iter()
            .filter(|s| is_in_enum_state(*s, img))
            .collect();

        Self {
            current_game_states,
        }
    }
}

#[derive(Debug)]
pub struct MpqFileFromUtf8Error {
    file_path: String,
    from_utf8_error: FromUtf8Error,
}

impl MpqFileFromUtf8Error {
    pub fn new(file_path: &str, from_utf8_error: FromUtf8Error) -> Self {
        Self {
            file_path: file_path.to_string(),
            from_utf8_error,
        }
    }
}

#[derive(Debug)]
pub struct LowInventorySpaceError;

#[derive(Debug)]
pub struct LowStashSpaceError;

#[derive(Debug)]
pub struct MovedToTownZoneError;

#[derive(Debug)]
pub struct CouldNotConnectMapsError;

#[derive(Debug)]
pub struct CouldNotGetPathError;

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HealthPointsUnderHardLimit(e) => write!(f, "{}", e),
            Self::LowHealthManaAndNoPotionInBelt(e) => write!(f, "{}", e),
            Self::CharacterHasDied(e) => write!(f, "{}", e),
            Self::WrongGameState(e) => write!(f, "{}", e),
            Self::LowInventorySpace(e) => write!(f, "{}", e),
            Self::MovedToTownZone(e) => write!(f, "{}", e),
            Self::CouldNotGetPath(e) => write!(f, "{}", e),
            Self::ArchiveError(e) => match e {
                ArchiveError::IoError(e) => write!(f, "{}", e),
                ArchiveError::MpqFileFromUtf8Error(e) => write!(f, "{}", e),
            },
        }
    }
}

impl fmt::Display for MpqFileFromUtf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mpq file \"{}\" could not be converted to utf8 due to error: {}",
            self.file_path,
            self.from_utf8_error.utf8_error()
        )
    }
}

impl fmt::Display for HealthPointsUnderHardLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Health points are below the hard limit.\n\tCurrent points: {}\n\tMax points: {}\n\tCurrent ratio: {}\n\tlimit: {}",
        self.points.current,
        self.points.max,
        self.points.current as f32 / self.points.max as f32,
        self.point_limit_hard)
    }
}

impl fmt::Display for LowHealthManaAndNoPotionInBeltError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Low health and mana, and no potion available in the belt."
        )
    }
}

impl fmt::Display for CharacterHasDiedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The character has died.")
    }
}

impl fmt::Display for WrongGameStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The game state is not as expected.")
    }
}

impl fmt::Display for LowInventorySpaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Low inventory space.")
    }
}

impl fmt::Display for MovedToTownZoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The character has moved to a town zone.")
    }
}

impl fmt::Display for CouldNotGetPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to find a path to the chosen destination.")
    }
}

impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error reading from mpq archive")
    }
}

impl From<MovedToTownZoneError> for BotError {
    fn from(error: MovedToTownZoneError) -> Self {
        Self::MovedToTownZone(error)
    }
}

impl From<CouldNotGetPathError> for BotError {
    fn from(error: CouldNotGetPathError) -> Self {
        Self::CouldNotGetPath(error)
    }
}

impl From<WrongGameStateError> for BotError {
    fn from(error: WrongGameStateError) -> Self {
        Self::WrongGameState(error)
    }
}

impl From<ArchiveError> for BotError {
    fn from(error: ArchiveError) -> Self {
        Self::ArchiveError(error)
    }
}

impl From<LowHealthManaAndNoPotionInBeltError> for BotError {
    fn from(error: LowHealthManaAndNoPotionInBeltError) -> Self {
        Self::LowHealthManaAndNoPotionInBelt(error)
    }
}

impl From<HealthPointsUnderHardLimitError> for BotError {
    fn from(error: HealthPointsUnderHardLimitError) -> Self {
        Self::HealthPointsUnderHardLimit(error)
    }
}

impl From<std::io::Error> for ArchiveError {
    fn from(error: std::io::Error) -> Self {
        ArchiveError::IoError(error)
    }
}

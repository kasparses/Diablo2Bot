use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    DT,
    NU,
    WL,
    GH,
    A1,
    A2,
    BL,
    SC,
    S1,
    S2,
    S3,
    S4,
    DD,
    KB,
    SQ,
    RN,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::DT => "DT",
            Self::NU => "NU",
            Self::WL => "WL",
            Self::GH => "GH",
            Self::A1 => "A1",
            Self::A2 => "A2",
            Self::BL => "BL",
            Self::SC => "SC",
            Self::S1 => "S1",
            Self::S2 => "S2",
            Self::S3 => "S3",
            Self::S4 => "S4",
            Self::DD => "DD",
            Self::KB => "KB",
            Self::SQ => "SQ",
            Self::RN => "RN",
        };

        write!(f, "{s}")
    }
}

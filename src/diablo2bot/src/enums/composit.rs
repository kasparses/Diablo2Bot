use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Composit {
    HD,
    TR,
    LG,
    RA,
    LA,
    RH,
    LH,
    SH,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
}

impl fmt::Display for Composit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::HD => "HD",
            Self::TR => "TR",
            Self::LG => "LG",
            Self::RA => "RA",
            Self::LA => "LA",
            Self::RH => "RH",
            Self::LH => "LH",
            Self::SH => "SH",
            Self::S1 => "S1",
            Self::S2 => "S2",
            Self::S3 => "S3",
            Self::S4 => "S4",
            Self::S5 => "S5",
            Self::S6 => "S6",
            Self::S7 => "S7",
            Self::S8 => "S8",
        };

        write!(f, "{s}")
    }
}

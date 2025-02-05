#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RejuvenationPotionType {
    Rejuvenation,
    FullRejuvenation,
}

impl RejuvenationPotionType {
    pub fn get_points(self, max_points: u32) -> u32 {
        match self {
            Self::Rejuvenation => (max_points as f32 * 0.35) as u32,
            Self::FullRejuvenation => max_points,
        }
    }
}

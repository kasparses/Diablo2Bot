use crate::{
    box_u16::BoxU16,
    constants::game_window_areas::{LIFE_TEXT_AREA, MANA_TEXT_AREA},
    enums::{
        belt_item_type::HealthManaPotionType,
        errors::{HealthPointsUnderHardLimitError, LowHealthManaAndNoPotionInBeltError},
        potion_points_type::PotionPointsType,
    },
    font_matcher::FontMatcher,
    game::Game,
    matrix::Matrix,
};

#[derive(Debug, Clone, Copy)]
pub struct Points {
    pub current: u32,
    pub max: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct HealthPoints(Points);

#[derive(Clone, Copy, Debug)]
pub struct ManaPoints(Points);

pub fn get_health(matrix: &Matrix, font_symbol_matcher: &FontMatcher) -> Option<HealthPoints> {
    match get_points(matrix, font_symbol_matcher, LIFE_TEXT_AREA) {
        Some(points) => Some(HealthPoints(points)),
        None => None,
    }
}

pub fn get_mana(matrix: &Matrix, font_symbol_matcher: &FontMatcher) -> Option<ManaPoints> {
    match get_points(matrix, font_symbol_matcher, MANA_TEXT_AREA) {
        Some(points) => Some(ManaPoints(points)),
        None => None,
    }
}

pub fn get_points_and_drink_potions_if_points_under_soft_limit(
    g: &mut Game,
    matrix: &Matrix,
) -> Result<Vec<HealthManaPotionType>, LowHealthManaAndNoPotionInBeltError> {
    let health_points = get_health(matrix, &g.font_symbol_matcher);
    let mana_points = get_mana(matrix, &g.font_symbol_matcher);

    drink_potions_if_points_under_soft_limit(g, health_points, mana_points)
}

pub fn drink_potions_if_points_under_soft_limit(
    g: &mut Game,
    health_points: Option<HealthPoints>,
    mana_points: Option<ManaPoints>,
) -> Result<Vec<HealthManaPotionType>, LowHealthManaAndNoPotionInBeltError> {
    let mut drunk_potions = Vec::new();

    if let Some(health_points) = health_points {
        drunk_potions.push(g.belt.drink_potions_if_points_under_soft_limit(
            &mut g.output_controller,
            g.profile.character_class,
            health_points.0,
            g.profile.health_limit,
            PotionPointsType::Health,
        )?);
    }

    if let Some(mana_points) = mana_points {
        drunk_potions.push(g.belt.drink_potions_if_points_under_soft_limit(
            &mut g.output_controller,
            g.profile.character_class,
            mana_points.0,
            g.profile.mana_limit,
            PotionPointsType::Mana,
        )?);
    }

    Ok(drunk_potions.into_iter().filter_map(|x| x).collect())
}

pub fn check_health_hard_limit(
    points: HealthPoints,
    limit: f32,
) -> Result<(), HealthPointsUnderHardLimitError> {
    let points = points.0;

    let ratio = points.current as f32 / points.max as f32;

    let under_hard_limit = ratio < limit;

    match under_hard_limit {
        true => Err(HealthPointsUnderHardLimitError {
            points,
            point_limit_hard: limit,
        }),
        false => Ok(()),
    }
}

fn get_points(
    matrix: &Matrix,
    font_symbol_matcher: &FontMatcher,
    window_area: BoxU16,
) -> Option<Points> {
    let points_text_area_matrix = matrix.get_sub_matrix2(window_area);
    let text = font_symbol_matcher.match_image_items(&points_text_area_matrix);

    if !text.is_empty() {
        return parse_text(&text[0].name);
    }

    None
}

fn parse_text(text: &str) -> Option<Points> {
    let parts: Vec<&str> = text.split_whitespace().collect();

    if parts.len() != 4 || parts[2] != "/" {
        return None;
    }

    let current = parts[1].parse().ok()?;
    let max = parts[3].parse().ok()?;

    Some(Points { current, max })
}

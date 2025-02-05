use crate::{
    attack_monsters::attack_monsters_and_loot,
    constants::spells::TELEPORT,
    enums::{click_type::ClickType, errors::BotError, weapon_set::WeaponSet},
    game::Game,
    point_i32::PointI32,
    point_u16::PointU16,
    wait_while_moving,
    zone_traveller::ZoneTraveller,
};

pub fn get_path_and_walk_it(
    g: &mut Game,
    zone_traveller: &mut ZoneTraveller,
) -> Result<(), BotError> {
    let path = zone_traveller.get_path(&g.pixel_palette, &mut g.logger)?;

    let path_diffs = get_path_diffs(&path);
    let path_coordinates = get_path_coordinates(&path_diffs);

    walk_path(
        g,
        &path_coordinates[..(g
            .bot_settings
            .movement_settings
            .max_movements_before_automap_path_refresh as usize)
            .min(path_coordinates.len())],
    )
}

fn walk_path(g: &mut Game, path_coordinates: &[PointU16]) -> Result<(), BotError> {
    for point in path_coordinates {
        if g.spell_caster.has_skill(TELEPORT) {
            g.spell_caster.use_skill(
                TELEPORT,
                *point,
                true,
                WeaponSet::Primary,
                &mut g.output_controller,
            );

            // Wait until teleport has moved our character. We will first scan for monsters after the teleport has moved us.
            g.spell_caster.sleep_cast_time(WeaponSet::Primary);
        } else {
            g.output_controller
                .click_mouse(*point, ClickType::Left, true, true);
            wait_while_moving(g);
        }

        // Scan for monsters. If we find any then we attack them and loot. Otherwise we just keep moving
        attack_monsters_and_loot(g)?;
    }

    Ok(())
}

fn get_path_diffs(path: &[PointU16]) -> Vec<PointI32> {
    let mut path_diffs = Vec::new();

    for i in 0..path.len() - 1 {
        let row = i32::from(path[i + 1].row) - i32::from(path[i].row);
        let col = i32::from(path[i + 1].col) - i32::from(path[i].col);

        path_diffs.push(PointI32 { row, col })
    }

    path_diffs
}

fn get_path_coordinates(path_diffs: &[PointI32]) -> Vec<PointU16> {
    let middle_row = 290;
    let middle_col = 400;
    let row_scale = 40;
    let col_scale = 40;

    let path_diffs: Vec<PointI32> = path_diffs
        .iter()
        .map(|p| PointI32 {
            row: p.row * row_scale,
            col: p.col * col_scale,
        })
        .collect();

    let row_min = 0;
    let row_max = 552;
    let col_min = 0;
    let col_max = 800;

    let round_row = |row: i32| {
        if row < row_min {
            return 0;
        }

        if row >= row_max {
            return row_max - 1;
        }

        row
    };

    let round_col = |col: i32| {
        if col < col_min {
            return 0;
        }

        if col >= col_max {
            return col_max - 1;
        }

        col
    };

    let mut coordinates = Vec::new();

    let mut row = 0;
    let mut col = 0;

    let mut missing_final_jump = true;

    for path_diff in &path_diffs {
        missing_final_jump = true;

        row += path_diff.row;
        col += path_diff.col;

        let screen_row = row + middle_row;
        let screen_col = col + middle_col;

        if screen_row < row_min
            || screen_row >= row_max
            || screen_col < col_min
            || screen_col >= col_max
        {
            coordinates.push(PointU16::new(
                round_row(screen_row) as u16,
                round_col(screen_col) as u16,
            ));

            missing_final_jump = false;
        }
    }

    if missing_final_jump {
        let screen_row = row + middle_row;
        let screen_col = col + middle_col;

        coordinates.push(PointU16::new(
            round_row(screen_row) as u16,
            round_col(screen_col) as u16,
        ));
    }

    coordinates
}

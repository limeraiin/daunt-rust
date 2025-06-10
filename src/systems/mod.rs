use crate::prelude::*;

mod map_render;
mod entity_render;
mod player_input;
mod collisions;
mod pathfinding;
mod end_turn;
mod movement;
mod hud;
mod tooltips;
mod dash;
mod fireball;
mod fireball_effects;
mod mana_regeneration;
mod targeting_highlights;
mod targeting_cursor;
mod debug_coordinates;
mod wave_management;
mod combat;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .flush()
        .add_system(targeting_highlights::targeting_highlights_system())
        .flush()
        .add_system(targeting_cursor::targeting_cursor_system())
        .add_system(fireball_effects::fireball_effects_system())
        .flush()
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .add_system(debug_coordinates::debug_coordinates_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(dash::dash_system())
        .add_system(dash::dash_to_point_system())
        .add_system(fireball::fireball_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(mana_regeneration::mana_regeneration_system())
        .flush()
        .add_system(wave_management::wave_management_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .flush()
        .add_system(targeting_highlights::targeting_highlights_system())
        .flush()
        .add_system(targeting_cursor::targeting_cursor_system())
        .add_system(fireball_effects::fireball_effects_system())
        .flush()
        .add_system(hud::hud_system())
        .add_system(debug_coordinates::debug_coordinates_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(pathfinding::pathfinding_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .flush()
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
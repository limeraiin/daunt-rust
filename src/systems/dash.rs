use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(CanDash)]
#[read_component(Mana)]
#[read_component(Point)]
pub fn dash(
    entity: &Entity,
    want_dash: &WantsToUseDash,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if let Ok(entry) = ecs.entry_ref(want_dash.entity) {
        if let (Ok(dash_ability), Ok(mana), Ok(current_pos)) = (
            entry.get_component::<CanDash>(),
            entry.get_component::<Mana>(),
            entry.get_component::<Point>()
        ) {
            if mana.current >= dash_ability.cost {
                let mut final_destination = *current_pos;
                
                for i in 1..=dash_ability.range {
                    let test_pos = *current_pos + (want_dash.direction * i);
                    
                    if map.can_enter_tile(test_pos) {
                        final_destination = test_pos;
                    } else {
                        break;
                    }
                }
                
                if final_destination != *current_pos {
                    let new_mana = Mana {
                        current: mana.current - dash_ability.cost,
                        max: mana.max,
                    };
                    commands.add_component(want_dash.entity, new_mana);
                    commands.add_component(want_dash.entity, final_destination);

                    if entry.get_component::<Player>().is_ok() {
                        camera.on_player_move(final_destination);
                    }
                }
            }
        }
    }
    commands.remove(*entity);
}

#[system(for_each)]
#[read_component(Player)]
#[read_component(CanDash)]
#[read_component(Mana)]
#[read_component(Point)]
pub fn dash_to_point(
    entity: &Entity,
    want_dash: &WantsToUseDashToPoint,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if let Ok(entry) = ecs.entry_ref(want_dash.entity) {
        if let (Ok(dash_ability), Ok(mana), Ok(current_pos)) = (
            entry.get_component::<CanDash>(),
            entry.get_component::<Mana>(),
            entry.get_component::<Point>()
        ) {
            if mana.current >= dash_ability.cost {
                let distance = DistanceAlg::Pythagoras.distance2d(*current_pos, want_dash.target);
                if distance <= dash_ability.range as f32 && 
                   map.can_enter_tile(want_dash.target) &&
                   has_clear_path_for_dash(map, *current_pos, want_dash.target) {
                    let new_mana = Mana {
                        current: mana.current - dash_ability.cost,
                        max: mana.max,
                    };
                    commands.add_component(want_dash.entity, new_mana);
                    commands.add_component(want_dash.entity, want_dash.target);
                    
                    if entry.get_component::<Player>().is_ok() {
                        camera.on_player_move(want_dash.target);
                    }
                }
            }
        }
    }
    commands.remove(*entity);
}

fn has_clear_path_for_dash(map: &Map, start: Point, end: Point) -> bool {
    let mut current = start;
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();
    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        if current != start {
            if current == end {
                return true;
            }
        
            if !map.can_enter_tile(current) {
                return false;
            }
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            current.x += sx;
        }
        if e2 < dx {
            err += dx;
            current.y += sy;
        }

        // Safety check to prevent infinite loops
        if current == end {
            return true;
        }
    }
}
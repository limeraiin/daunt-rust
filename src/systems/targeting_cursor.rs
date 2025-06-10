use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(CanDash)]
#[read_component(CanCastFireball)]
#[read_component(Mana)]
pub fn targeting_cursor(
    ecs: &SubWorld,
    #[resource] targeting_state: &TargetingState,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
    #[resource] map: &Map
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(4);
    
    if targeting_state.is_targeting() && 
       mouse_pos.x >= 0 && mouse_pos.x < DISPLAY_WIDTH && 
       mouse_pos.y >= 0 && mouse_pos.y < DISPLAY_HEIGHT {
        
        let offset = Point::new(camera.left_x, camera.top_y);
        let world_mouse_pos = *mouse_pos + offset;
        
        if let Some((player_pos, dash_ability, _fireball_ability, mana)) = get_player_data(ecs) {
            match targeting_state {
                TargetingState::SelectingDashTarget => {
                    let distance = DistanceAlg::Pythagoras.distance2d(player_pos, world_mouse_pos);
                    let in_range = distance <= dash_ability.range as f32 && distance > 0.0;
                    let is_valid_tile = map.can_enter_tile(world_mouse_pos);
                    let clear_path = is_valid_tile && has_clear_path_for_dash(map, player_pos, world_mouse_pos);
                    let has_mana = mana.current >= dash_ability.cost;
                    
                    let (cursor_char, cursor_color) = if !has_mana {
                        ('M', ColorPair::new(BLUE, BLACK))
                    } else if !is_valid_tile || !in_range || !clear_path {
                        ('X', ColorPair::new(RED, BLACK))
                    } else {
                        ('X', ColorPair::new(GREEN, BLACK))
                    };
                    
                    draw_batch.set(*mouse_pos, cursor_color, to_cp437(cursor_char));
                }
                
                TargetingState::SelectingFireballTarget => {
                }
                
                _ => {}
            }
        }
    }
    
    draw_batch.submit(25000).expect("Batch error");
}

fn get_player_data(ecs: &SubWorld) -> Option<(Point, CanDash, CanCastFireball, Mana)> {
    let mut player_query = <(&Point, &CanDash, &CanCastFireball, &Mana)>::query()
        .filter(component::<Player>());
    player_query.iter(ecs).nth(0).map(|(p, d, f, m)| (*p, *d, *f, *m))
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

        if current == end {
            return true;
        }
    }
}
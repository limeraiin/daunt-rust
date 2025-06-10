use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(CanDash)]
#[read_component(CanCastFireball)]
#[read_component(Mana)]
pub fn targeting_highlights(
    ecs: &SubWorld,
    #[resource] targeting_state: &TargetingState,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
    #[resource] map: &Map
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(3);
    let offset = Point::new(camera.left_x, camera.top_y);
    
    let world_mouse_pos = *mouse_pos + offset;
    
    
    if !targeting_state.is_targeting() {
        draw_batch.submit(7000).expect("Batch error");
        return;
    }

    let mut player_query = <(&Point, &CanDash, &CanCastFireball, &Mana)>::query()
        .filter(component::<Player>());
    
    if let Some((player_pos, dash_ability, fireball_ability, mana)) = player_query.iter(ecs).nth(0) {
        match targeting_state {
            TargetingState::SelectingDashTarget => {
                if mana.current >= dash_ability.cost {
                    for y in (player_pos.y - dash_ability.range)..=(player_pos.y + dash_ability.range) {
                        for x in (player_pos.x - dash_ability.range)..=(player_pos.x + dash_ability.range) {
                            let target_pos = Point::new(x, y);
                            let distance = DistanceAlg::Pythagoras.distance2d(*player_pos, target_pos);
                            
                            if distance <= dash_ability.range as f32 && distance > 0.0 {
                                if map.can_enter_tile(target_pos) && 
                                   has_clear_path_for_dash(map, *player_pos, target_pos) {
                                    let screen_pos = target_pos - offset;
                                    if screen_pos.x >= 0 && screen_pos.x < DISPLAY_WIDTH && 
                                       screen_pos.y >= 0 && screen_pos.y < DISPLAY_HEIGHT {
                                        draw_batch.set(
                                            screen_pos,
                                            ColorPair::new(CYAN, BLACK),
                                            to_cp437('·')
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            TargetingState::SelectingFireballTarget => {
                if mana.current >= fireball_ability.cost {
                    for y in (player_pos.y - fireball_ability.range)..=(player_pos.y + fireball_ability.range) {
                        for x in (player_pos.x - fireball_ability.range)..=(player_pos.x + fireball_ability.range) {
                            let target_pos = Point::new(x, y);
                            let distance = DistanceAlg::Pythagoras.distance2d(*player_pos, target_pos);
                            
                            if distance <= fireball_ability.range as f32 && distance > 0.0 {
                                if map.can_enter_tile(target_pos) && 
                                   has_line_of_sight(map, *player_pos, target_pos) {
                                    let screen_pos = target_pos - offset;
                                    if screen_pos.x >= 0 && screen_pos.x < DISPLAY_WIDTH && 
                                       screen_pos.y >= 0 && screen_pos.y < DISPLAY_HEIGHT {
                                        
                                        let blast_distance = DistanceAlg::Pythagoras.distance2d(world_mouse_pos, target_pos);
                                        
                                        let (color, glyph) = if blast_distance <= 1.5 {
                                            if map.can_enter_tile(world_mouse_pos) && 
                                               has_line_of_sight(map, *player_pos, world_mouse_pos) {
                                                (ColorPair::new(ORANGE, BLACK), to_cp437('*'))
                                            } else {
                                                (ColorPair::new(RED, BLACK), to_cp437('x'))
                                            }
                                        } else {
                                            (ColorPair::new(YELLOW, BLACK), to_cp437('·'))
                                        };
                                        
                                        draw_batch.set(screen_pos, color, glyph);
                                    }
                                }
                            }
                        }
                    }
                    
                    if map.can_enter_tile(world_mouse_pos) && 
                       has_line_of_sight(map, *player_pos, world_mouse_pos) {
                        let distance = DistanceAlg::Pythagoras.distance2d(*player_pos, world_mouse_pos);
                        if distance <= fireball_ability.range as f32 {
                            draw_line_of_sight(*player_pos, world_mouse_pos, offset, &mut draw_batch);
                        }
                    }
                }
            }
            
            _ => {}
        }
    }
    
    draw_batch.submit(7000).expect("Batch error");
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

fn has_line_of_sight(map: &Map, start: Point, end: Point) -> bool {
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

fn draw_line_of_sight(start: Point, end: Point, camera_offset: Point, draw_batch: &mut DrawBatch) {
    let mut current = start;
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();
    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        if current != start && current != end {
            let screen_pos = current - camera_offset;
            if screen_pos.x >= 0 && screen_pos.x < DISPLAY_WIDTH && 
               screen_pos.y >= 0 && screen_pos.y < DISPLAY_HEIGHT {
                draw_batch.set(
                    screen_pos,
                    ColorPair::new(GREEN, BLACK),
                    to_cp437('.')
                );
            }
        }

        if current == end {
            break;
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
    }
}
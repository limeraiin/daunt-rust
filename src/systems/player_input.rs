use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(CanDash)]
#[read_component(CanCastFireball)]
#[read_component(Mana)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn player_input(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] mouse_buttons: &Option<(i32, i32, bool, bool, bool)>,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
    #[resource] turn_state: &mut TurnState,
    #[resource] targeting_state: &mut TargetingState,
    #[resource] map: &Map
) {
    let mut players = <(Entity, &Point, &CanDash, &CanCastFireball, &Mana)>::query()
        .filter(component::<Player>());

    if let Some((player_entity, player_point, dash_ability, fireball_ability, mana)) = players.iter(ecs).nth(0) {
        let player_entity = *player_entity;
        let player_point = *player_point;

        if let Some((_, _, left_click, right_click, _)) = *mouse_buttons {
            // Right-click cancels targeting
            if right_click {
                if targeting_state.is_targeting() {
                    *targeting_state = TargetingState::None;
                }
                return;
            }
            
            if left_click && targeting_state.is_targeting() {
                let offset = Point::new(camera.left_x, camera.top_y);
                let world_target = *mouse_pos + offset;
                
                if !map.can_enter_tile(world_target) {
                    return;
                }
                
                match *targeting_state {
                    TargetingState::SelectingDashTarget => {
                        if mana.current >= dash_ability.cost {
                            let distance = DistanceAlg::Pythagoras.distance2d(player_point, world_target);
                            
                            if distance <= dash_ability.range as f32 {
                                if has_clear_path(map, player_point, world_target) {
                                    commands.push(((), WantsToUseDashToPoint {
                                        entity: player_entity,
                                        target: world_target,
                                    }));
                                    
                                    *turn_state = TurnState::PlayerTurn;
                                    *targeting_state = TargetingState::None;
                                }
                            }
                        }
                    }
                    
                    TargetingState::SelectingFireballTarget => {
                        if mana.current >= fireball_ability.cost {
                            let distance = DistanceAlg::Pythagoras.distance2d(player_point, world_target);
                            
                            if distance <= fireball_ability.range as f32 && 
                               has_line_of_sight(map, player_point, world_target) {
                                commands.push(((), WantsToUseFireball {
                                    entity: player_entity,
                                    target: world_target,
                                }));
                                
                                *turn_state = TurnState::PlayerTurn;
                                *targeting_state = TargetingState::None;
                            }
                        }
                    }
                    
                    _ => {}
                }
            }
        }

        if let Some(key) = *key {
            if targeting_state.is_targeting() {
                match key {
                    VirtualKeyCode::Escape => {
                        *targeting_state = TargetingState::None;
                        return;
                    }
                    _ => {}
                }
                return;
            }

            match key {
                VirtualKeyCode::Left | VirtualKeyCode::Right | 
                VirtualKeyCode::Up | VirtualKeyCode::Down => {
                    let delta = match key {
                        VirtualKeyCode::Left => Point::new(-1, 0),
                        VirtualKeyCode::Right => Point::new(1, 0),
                        VirtualKeyCode::Up => Point::new(0, -1),
                        VirtualKeyCode::Down => Point::new(0, 1),   
                        _ => Point::new(0, 0),
                    };

                    let mut monsters = <(Entity, &Point)>::query().filter(component::<Enemy>());
                    let destination = player_point + delta;

                    let mut hit_something = false;
                    monsters.iter(ecs).for_each(|(enemy_entity, enemy_point)| {
                        if *enemy_point == destination {
                            hit_something = true;
                            commands.push(((), WantsToAttack {
                                attacker: player_entity, 
                                victim: *enemy_entity
                            }));
                        }
                    });

                    if !hit_something {
                        commands.push(((), WantsToMove{
                            entity: player_entity, 
                            destination: destination
                        }));
                    }
                    
                    *turn_state = TurnState::PlayerTurn;
                }
                
                VirtualKeyCode::Space => {
                    *turn_state = TurnState::PlayerTurn;
                }
                
                VirtualKeyCode::D => {
                    if mana.current >= dash_ability.cost {
                        *targeting_state = TargetingState::SelectingDashTarget;
                    }
                }
                
                VirtualKeyCode::F => {
                    if mana.current >= fireball_ability.cost {
                        *targeting_state = TargetingState::SelectingFireballTarget;
                    }
                }
                
                _ => {}
            }
        }
    }
}

fn has_clear_path(map: &Map, start: Point, end: Point) -> bool {
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

// Bresenham's algorithm
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
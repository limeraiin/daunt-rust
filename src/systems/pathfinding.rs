use crate::prelude::*;
use std::collections::VecDeque;

#[system]
#[read_component(Point)]
#[read_component(FollowsPlayer)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn pathfinding(
    ecs: &SubWorld, 
    commands: &mut CommandBuffer,
    #[resource] map: &Map
) {
    let (player_pos, player_entity) = {
        let mut player_query = <(Entity, &Point)>::query().filter(component::<Player>());
        if let Some((entity, pos)) = player_query.iter(ecs).nth(0) {
            (*pos, *entity)
        } else {
            return;
        }
    };
    
    let flow_field = create_flow_field(map, player_pos);
    
    let enemy_positions: Vec<Point> = <&Point>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .copied()
        .collect();
    
    let mut enemy_query = <(Entity, &Point, &FollowsPlayer)>::query()
        .filter(component::<Enemy>());
    
    let enemies_to_process: Vec<(Entity, Point, i32)> = enemy_query
        .iter(ecs)
        .map(|(entity, pos, follower)| (*entity, *pos, follower.move_timer))
        .collect();
    
    for (entity, pos, move_timer) in enemies_to_process {
        if move_timer <= 0 {
            let new_follower = FollowsPlayer { move_timer: 1 };
            commands.add_component(entity, new_follower);
            
            let distance_to_player = (pos.x - player_pos.x).abs() + (pos.y - player_pos.y).abs();
            if distance_to_player == 1 {
                commands.push(((), WantsToAttack {
                    attacker: entity,
                    victim: player_entity,
                }));
            } else {
                if let Some(best_move) = find_best_move(&flow_field, map, pos, &enemy_positions, player_pos) {
                    commands.push(((), WantsToMove {
                        entity,
                        destination: best_move,
                    }));
                }
            }
        } else {
            let new_follower = FollowsPlayer {
                move_timer: move_timer - 1,
            };
            commands.add_component(entity, new_follower);
        }
    }
}

fn create_flow_field(map: &Map, player_pos: Point) -> Vec<Vec<i32>> {
    let mut distances = vec![vec![-1; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize];
    let mut queue = VecDeque::new();
    
    let target_positions = [
        Point::new(player_pos.x, player_pos.y - 1),
        Point::new(player_pos.x + 1, player_pos.y),
        Point::new(player_pos.x, player_pos.y + 1),
        Point::new(player_pos.x - 1, player_pos.y),
    ];
    
    distances[player_pos.x as usize][player_pos.y as usize] = -2;
    
    for target_pos in target_positions.iter() {
        if map.in_bounds(*target_pos) && map.can_enter_tile(*target_pos) {
            distances[target_pos.x as usize][target_pos.y as usize] = 0;
            queue.push_back(*target_pos);
        }
    }
    
    let directions = [
        Point::new(-1, -1), Point::new(0, -1), Point::new(1, -1),
        Point::new(-1,  0),                     Point::new(1,  0),
        Point::new(-1,  1), Point::new(0,  1), Point::new(1,  1),
    ];
    
    while let Some(current) = queue.pop_front() {
        let current_distance = distances[current.x as usize][current.y as usize];
        
        for direction in directions.iter() {
            let next_pos = current + *direction;
            
            if map.in_bounds(next_pos) && map.can_enter_tile(next_pos) {
                let next_x = next_pos.x as usize;
                let next_y = next_pos.y as usize;
                
                if distances[next_x][next_y] == -1 {
                    distances[next_x][next_y] = current_distance + 1;
                    queue.push_back(next_pos);
                }
            }
        }
    }
    
    distances
}

fn find_best_move(
    flow_field: &Vec<Vec<i32>>, 
    map: &Map, 
    enemy_pos: Point, 
    enemy_positions: &[Point],
    player_pos: Point
) -> Option<Point> {
    let current_distance = flow_field[enemy_pos.x as usize][enemy_pos.y as usize];
    
    if current_distance == -1 {
        return None;
    }
    
    if current_distance == 0 {
        return None;
    }
    
    let directions = [
        Point::new(-1, -1), Point::new(0, -1), Point::new(1, -1),
        Point::new(-1,  0),                     Point::new(1,  0),
        Point::new(-1,  1), Point::new(0,  1), Point::new(1,  1),
    ];
    
    let mut best_move = None;
    let mut best_distance = current_distance;
    
    for direction in directions.iter() {
        let next_pos = enemy_pos + *direction;
        
        if map.in_bounds(next_pos) && 
           map.can_enter_tile(next_pos) && 
           !enemy_positions.contains(&next_pos) &&
           next_pos != player_pos {
            
            let next_distance = flow_field[next_pos.x as usize][next_pos.y as usize];
            
            if next_distance != -1 && next_distance != -2 && next_distance < best_distance {
                best_distance = next_distance;
                best_move = Some(next_pos);
            }
        }
    }
    
    best_move
}
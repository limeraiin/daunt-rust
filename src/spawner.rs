use crate::prelude::*;

pub fn spawn_player(ecs : &mut World, pos : Point) {
    ecs.push(
        (Player, 
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph : to_cp437('@')
            },
            Health{ current: 15, max: 15 },
            Mana{ current: 8, max: 8 },
            CanDash{ cost: 4, range: 4 },
            CanCastFireball{ cost: 5, damage: 3, range: 6 },
        )
    );
}

pub fn spawn_wave_monsters(ecs: &mut World, map: &Map, wave_number: i32) -> i32 {
    let mut rng = RandomNumberGenerator::new();
    let mut enemies_spawned = 0;
    
    // Find valid spawn positions
    let mut spawn_positions = Vec::new();
    for y in 1..SCREEN_HEIGHT-1 {
        for x in 1..SCREEN_WIDTH-1 {
            let pos = Point::new(x, y);
            if map.can_enter_tile(pos) {
                spawn_positions.push(pos);
            }
        }
    }
    
    // Shuffle the spawn positions
    for i in 0..spawn_positions.len() {
        let j = rng.range(0, spawn_positions.len());
        spawn_positions.swap(i, j);
    }
    
    let mut spawn_index = 0;
    
    match wave_number {
        1 => {
            for _ in 0..3 {
                if spawn_index < spawn_positions.len() {
                    spawn_monster_by_type(ecs, EnemyType::Weak, spawn_positions[spawn_index]);
                    spawn_index += 1;
                    enemies_spawned += 1;
                }
            }
        }
        2 => {
            for _ in 0..2 {
                if spawn_index < spawn_positions.len() {
                    spawn_monster_by_type(ecs, EnemyType::Weak, spawn_positions[spawn_index]);
                    spawn_index += 1;
                    enemies_spawned += 1;
                }
            }
            for _ in 0..2 {
                if spawn_index < spawn_positions.len() {
                    spawn_monster_by_type(ecs, EnemyType::Medium, spawn_positions[spawn_index]);
                    spawn_index += 1;
                    enemies_spawned += 1;
                }
            }
        }
        3 => {
            for _ in 0..3 {
                if spawn_index < spawn_positions.len() {
                    spawn_monster_by_type(ecs, EnemyType::Medium, spawn_positions[spawn_index]);
                    spawn_index += 1;
                    enemies_spawned += 1;
                }
            }
            if spawn_index < spawn_positions.len() {
                spawn_monster_by_type(ecs, EnemyType::Boss, spawn_positions[spawn_index]);
                enemies_spawned += 1;
            }
        }
        _ => {
            for _ in 0..wave_number {
                if spawn_index < spawn_positions.len() {
                    let enemy_type = match rng.range(0, 3) {
                        0 => EnemyType::Weak,
                        1 => EnemyType::Medium,
                        _ => EnemyType::Boss,
                    };
                    spawn_monster_by_type(ecs, enemy_type, spawn_positions[spawn_index]);
                    spawn_index += 1;
                    enemies_spawned += 1;
                }
            }
        }
    }
    
    enemies_spawned
}

fn spawn_monster_by_type(ecs: &mut World, enemy_type: EnemyType, pos: Point) {
    let (hp, name, glyph, color) = match enemy_type {
        EnemyType::Weak => (2, "Goblin".to_string(), to_cp437('g'), ColorPair::new(WHITE, BLACK)),
        EnemyType::Medium => (5, "Orc".to_string(), to_cp437('O'), ColorPair::new(WHITE, BLACK)),
        EnemyType::Boss => (12, "Troll".to_string(), to_cp437('E'), ColorPair::new(WHITE, BLACK)),
    };

    ecs.push(
        (Enemy,
            pos,
            Render{
                color,
                glyph,
            },
            FollowsPlayer{ move_timer: 0 },
            Health{current: hp, max: hp},
            Name(name),
            EnemyStats{ enemy_type },
        )
    );
}
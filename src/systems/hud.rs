use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Mana)]
#[read_component(CanDash)]
#[read_component(CanCastFireball)]
#[read_component(Player)]
pub fn hud(
    ecs: &SubWorld,
    #[resource] targeting_state: &TargetingState,
    #[resource] wave_manager: &WaveManager
) {
    let mut player_query = <(&Health, &Mana, &CanDash, &CanCastFireball)>::query()
        .filter(component::<Player>());
    
    if let Some((player_health, player_mana, dash_ability, fireball_ability)) = 
        player_query.iter(ecs).nth(0) {
        
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(2);
        
        if wave_manager.current_wave <= 3 {
            if wave_manager.wave_active {
                draw_batch.print_color_centered(1, 
                    format!("Wave {} - Enemies: {}", wave_manager.current_wave, wave_manager.enemies_remaining),
                    ColorPair::new(YELLOW, BLACK)
                );
            } else {
                draw_batch.print_color_centered(1, 
                    format!("Wave {} incoming in {} turns", wave_manager.current_wave, wave_manager.spawn_timer),
                    ColorPair::new(CYAN, BLACK)
                );
            }
        } else {
            draw_batch.print_color_centered(1, 
                "All waves completed! Victory!",
                ColorPair::new(GREEN, BLACK)
            );
        }
        
        match targeting_state {
            TargetingState::None => {
                if wave_manager.current_wave <= 3 {
                    draw_batch.print_centered(2, "Survive the waves! Cursor keys to move, SPACE to wait.");
                }
            }
            TargetingState::SelectingDashTarget => {
                draw_batch.print_color_centered(2, 
                    "DASH MODE: Click to dash to target position",
                    ColorPair::new(CYAN, BLACK)
                );
                draw_batch.print_color_centered(3, 
                    "ESC or Right-click to cancel",
                    ColorPair::new(WHITE, BLACK)
                );
            }
            TargetingState::SelectingFireballTarget => {
                draw_batch.print_color_centered(2, 
                    "FIREBALL MODE: Click to cast at target",
                    ColorPair::new(YELLOW, BLACK)
                );
                draw_batch.print_color_centered(3, 
                    "ESC or Right-click to cancel",
                    ColorPair::new(WHITE, BLACK)
                );
            }
        }
        
        let health_y = if targeting_state.is_targeting() { 5 } else { 4 };
        let mana_y = if targeting_state.is_targeting() { 7 } else { 6 };
        let skills_y = if targeting_state.is_targeting() { 9 } else { 8 };
        
        draw_batch.bar_horizontal(
            Point::new(0, health_y),
            SCREEN_WIDTH * 2,
            player_health.current,
            player_health.max,
            ColorPair::new(RED, BLACK)
        );
        draw_batch.print_color_centered(
            health_y - 1,
            format!(" Health: {} / {} ", player_health.current, player_health.max),
            ColorPair::new(WHITE, RED)
        );
        
        draw_batch.bar_horizontal(
            Point::new(0, mana_y),
            SCREEN_WIDTH * 2,
            player_mana.current,
            player_mana.max,
            ColorPair::new(BLUE, BLACK)
        );
        draw_batch.print_color_centered(
            mana_y - 1,
            format!(" Mana: {} / {} ", player_mana.current, player_mana.max),
            ColorPair::new(WHITE, BLUE)
        );
        
        if !targeting_state.is_targeting() {
            draw_batch.print_centered(skills_y, "Skills:");
            
            let dash_color = if player_mana.current >= dash_ability.cost {
                ColorPair::new(WHITE, BLACK)
            } else {
                ColorPair::new(GRAY, BLACK)
            };
            draw_batch.print_color(
                Point::new(5, skills_y + 1),
                format!("(D) Dash - Cost: {} mana", dash_ability.cost),
                dash_color
            );
            
            let fireball_color = if player_mana.current >= fireball_ability.cost {
                ColorPair::new(WHITE, BLACK)
            } else {
                ColorPair::new(GRAY, BLACK)
            };
            draw_batch.print_color(
                Point::new(5, skills_y + 2),
                format!("(F) Fireball - Cost: {} mana", fireball_ability.cost),
                fireball_color
            );
            
            draw_batch.print_centered(skills_y + 4, "Press D or F to select skills");
            draw_batch.print_centered(skills_y + 5, "Move with arrow keys, SPACE to wait, attack by bumping onto enemies");
        } else {
            match targeting_state {
                TargetingState::SelectingDashTarget => {
                    draw_batch.print_color_centered(skills_y, 
                        format!("DASH: Range {} tiles, Cost {} mana", dash_ability.range, dash_ability.cost),
                        ColorPair::new(CYAN, BLACK)
                    );
                    draw_batch.print_centered(skills_y + 1, "Cyan dots show valid dash destinations");
                    draw_batch.print_centered(skills_y + 2, "Click exactly where you want to dash");
                    draw_batch.print_color_centered(skills_y + 3, 
                        "Cursor: Green=valid | Red=invalid | Blue M=no mana",
                        ColorPair::new(WHITE, BLACK)
                    );
                }
                TargetingState::SelectingFireballTarget => {
                    draw_batch.print_color_centered(skills_y, 
                        format!("FIREBALL: Range {} tiles, Damage {}, Cost {} mana", 
                            fireball_ability.range, fireball_ability.damage, fireball_ability.cost),
                        ColorPair::new(YELLOW, BLACK)
                    );
                    draw_batch.print_centered(skills_y + 1, "Yellow dots: valid targets | Orange: blast area");
                    draw_batch.print_centered(skills_y + 2, "Green line shows line of sight to cursor");
                    draw_batch.print_color_centered(skills_y + 3, 
                        "Cursor: Green=valid | Red=invalid | Blue M=no mana",
                        ColorPair::new(WHITE, BLACK)
                    );
                }
                _ => {}
            }
        }
        
        draw_batch.submit(10000).expect("Batch error");
    }
}
use crate::prelude::*;

#[system]
#[read_component(Enemy)]
pub fn wave_management(
    ecs: &SubWorld,
    #[resource] wave_manager: &mut WaveManager,
) {
    if wave_manager.wave_active {
        let enemy_count = <&Enemy>::query().iter(ecs).count();
        wave_manager.enemies_remaining = enemy_count as i32;
        
        if enemy_count == 0 {
            wave_manager.wave_active = false;
            wave_manager.spawn_timer = 2;
            wave_manager.current_wave += 1;
        }
    } else {
        if wave_manager.spawn_timer > 0 {
            wave_manager.spawn_timer -= 1;
        }
    }
}
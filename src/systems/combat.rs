use crate::prelude::*;

#[system(for_each)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Name)]
pub fn combat(
    entity: &Entity,
    want_attack: &WantsToAttack,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if let Ok(victim_entry) = ecs.entry_ref(want_attack.victim) {
        if let Ok(victim_health) = victim_entry.get_component::<Health>() {
            let damage = 1; // Everyone does 1 damage
            
            let new_health = Health {
                current: (victim_health.current - damage).max(0),
                max: victim_health.max,
            };
            
            if new_health.current <= 0 {
                commands.remove(want_attack.victim);
            } else {
                commands.add_component(want_attack.victim, new_health);
            }
        }
    }
    commands.remove(*entity);
}
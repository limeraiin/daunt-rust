use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Mana)]
pub fn mana_regeneration(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_query = <(Entity, &Mana)>::query().filter(component::<Player>());
    
    if let Some((player_entity, player_mana)) = player_query.iter(ecs).nth(0) {
        // Regenerate 1 mana per turn, but not above max
        if player_mana.current < player_mana.max {
            let new_mana = Mana {
                current: (player_mana.current + 1).min(player_mana.max),
                max: player_mana.max,
            };
            commands.add_component(*player_entity, new_mana);
        }
    }
}
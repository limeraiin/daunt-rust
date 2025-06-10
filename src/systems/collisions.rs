use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    let mut player_entity = None;
    
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());
    players.iter(ecs).for_each(|(entity, pos)| {
        player_pos = *pos;
        player_entity = Some(*entity);
    });
    
    if let Some(player_entity) = player_entity {
        let mut enemies = <(Entity, &Point)>::query()
            .filter(component::<Enemy>());
        enemies
            .iter(ecs)
            .filter(|(_, pos)| **pos == player_pos)
            .for_each(|(enemy_entity, _)| {
                commands.push(((), WantsToAttack {
                    attacker: *enemy_entity,
                    victim: player_entity
                }));
            });
    }
}
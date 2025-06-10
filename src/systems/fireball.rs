use crate::prelude::*;

#[system(for_each)]
#[read_component(CanCastFireball)]
#[read_component(Mana)]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Enemy)]
#[read_component(Player)]
pub fn fireball(
    entity: &Entity,
    want_fireball: &WantsToUseFireball,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    if let Ok(entry) = ecs.entry_ref(want_fireball.entity) {
        if let (Ok(fireball_ability), Ok(mana), Ok(caster_pos)) = (
            entry.get_component::<CanCastFireball>(),
            entry.get_component::<Mana>(),
            entry.get_component::<Point>()
        ) {
            let distance = DistanceAlg::Pythagoras.distance2d(*caster_pos, want_fireball.target);
            
            if mana.current >= fireball_ability.cost && distance <= fireball_ability.range as f32 {
                let new_mana = Mana {
                    current: mana.current - fireball_ability.cost,
                    max: mana.max,
                };
                commands.add_component(want_fireball.entity, new_mana);
                
                commands.push((
                    FireballEffect {
                        center: want_fireball.target,
                        radius: 1,
                        damage: fireball_ability.damage,
                        duration: 3,
                    },
                ));
                
                let mut enemies_to_damage = Vec::new();
                let mut enemies_query = <(Entity, &Point, &Health)>::query()
                    .filter(component::<Enemy>());
                
                for (enemy_entity, enemy_pos, enemy_health) in enemies_query.iter(ecs) {
                    let blast_distance = DistanceAlg::Pythagoras
                        .distance2d(want_fireball.target, *enemy_pos);
                    
                    if blast_distance <= 1.5 {
                        enemies_to_damage.push((*enemy_entity, *enemy_health));
                    }
                }
                
                for (enemy_entity, enemy_health) in enemies_to_damage {
                    let new_health = Health {
                        current: (enemy_health.current - fireball_ability.damage).max(0),
                        max: enemy_health.max,
                    };
                    
                    if new_health.current <= 0 {
                        commands.remove(enemy_entity);
                    } else {
                        commands.add_component(enemy_entity, new_health);
                    }
                }
            }
        }
    }
    
    commands.remove(*entity);
}
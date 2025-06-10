use crate::prelude::*;

#[system(for_each)]
pub fn fireball_effects(
    entity: &Entity,
    effect: &FireballEffect,
    #[resource] camera: &Camera,
    commands: &mut CommandBuffer
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    
    for y in (effect.center.y - effect.radius)..=(effect.center.y + effect.radius) {
        for x in (effect.center.x - effect.radius)..=(effect.center.x + effect.radius) {
            let pos = Point::new(x, y);
            let distance = DistanceAlg::Pythagoras.distance2d(effect.center, pos);
            
            if distance <= effect.radius as f32 {
                let screen_pos = pos - offset;
                // Position is on screen?
                if screen_pos.x >= 0 && screen_pos.x < DISPLAY_WIDTH && 
                   screen_pos.y >= 0 && screen_pos.y < DISPLAY_HEIGHT {
                    draw_batch.set(
                        screen_pos,
                        ColorPair::new(YELLOW, RED),
                        to_cp437('*')
                    );
                }
            }
        }
    }
    
    draw_batch.submit(6000).expect("Batch error");
    commands.remove(*entity);
}
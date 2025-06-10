use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
pub fn debug_coordinates(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
    #[resource] targeting_state: &TargetingState,
    #[resource] map: &Map
) {
    if !targeting_state.is_targeting() {
        return;
    }
    
    let mut player_query = <&Point>::query().filter(component::<Player>());
    
    if let Some(player_pos) = player_query.iter(ecs).nth(0) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(2);
        
        let offset = Point::new(camera.left_x, camera.top_y);
        let world_mouse = *mouse_pos + offset;
        
        let is_valid_tile = map.in_bounds(world_mouse);
        let can_enter = if is_valid_tile { map.can_enter_tile(world_mouse) } else { false };
        
        draw_batch.print(
            Point::new(1, 14),
            format!("Mouse Screen: ({}, {})", mouse_pos.x, mouse_pos.y)
        );
        draw_batch.print(
            Point::new(1, 15),
            format!("Mouse World: ({}, {})", world_mouse.x, world_mouse.y)
        );
        draw_batch.print(
            Point::new(1, 16),
            format!("Player: ({}, {})", player_pos.x, player_pos.y)
        );
        draw_batch.print(
            Point::new(1, 17),
            format!("Camera: ({}, {})", camera.left_x, camera.top_y)
        );
        draw_batch.print(
            Point::new(1, 18),
            format!("Valid Tile: {}, Can Enter: {}", is_valid_tile, can_enter)
        );
        
        if *targeting_state == TargetingState::SelectingFireballTarget {
            let has_los = has_line_of_sight(map, *player_pos, world_mouse);
            let distance = DistanceAlg::Pythagoras.distance2d(*player_pos, world_mouse);
            
            draw_batch.print(
                Point::new(1, 19),
                format!("Line of Sight: {}", has_los)
            );
            draw_batch.print(
                Point::new(1, 20),
                format!("Distance: {:.2}", distance)
            );
        }
        
        draw_batch.print(
            Point::new(1, 21),
            format!("Display: {}x{} | Screen: {}x{}", DISPLAY_WIDTH, DISPLAY_HEIGHT, SCREEN_WIDTH, SCREEN_HEIGHT)
        );
        
        let at_edge = mouse_pos.x == 0 || mouse_pos.x == DISPLAY_WIDTH - 1 || 
                     mouse_pos.y == 0 || mouse_pos.y == DISPLAY_HEIGHT - 1;
        draw_batch.print(
            Point::new(1, 22),
            format!("At edge: {} | Bounds: 0-{}, 0-{}", at_edge, DISPLAY_WIDTH-1, DISPLAY_HEIGHT-1)
        );
        
        draw_batch.print(
            Point::new(1, 23),
            format!("Targeting: {} | Mouse in bounds: {}", 
                targeting_state.is_targeting(),
                mouse_pos.x >= 0 && mouse_pos.x < DISPLAY_WIDTH && mouse_pos.y >= 0 && mouse_pos.y < DISPLAY_HEIGHT)
        );
        
        draw_batch.submit(11000).expect("Batch error");
    }
}

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
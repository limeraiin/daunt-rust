#![warn(clippy::pedantic)]

mod components;
mod spawner;
mod map;
mod map_builder;
mod systems;
mod camera;
mod turn_state;
mod targeting_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::turn_state::*;
    pub use crate::targeting_state::*;
    pub use smallvec::SmallVec;
}

use prelude::*;

struct State {
    ecs : World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        
        spawn_player(&mut ecs, map_builder.player_start);
        
        let wave_manager = WaveManager {
            current_wave: 1,
            enemies_remaining: 0,
            wave_active: false,
            spawn_timer: 2,
        };
        
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(TargetingState::None);
        resources.insert(wave_manager);
        
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
        ctx.set_active_console(4);
        ctx.cls();
        
        self.resources.insert(ctx.key);
        
        let (mouse_x, mouse_y) = ctx.mouse_pos();
        let mouse_tile_pos = Point::new(
            (mouse_x as i32).min(DISPLAY_WIDTH - 1).max(0),
            (mouse_y as i32).min(DISPLAY_HEIGHT - 1).max(0)
        );
        
        self.resources.insert(mouse_tile_pos);
        
        let input = INPUT.lock();
        let left_click = input.is_mouse_button_pressed(0);
        let right_click = input.is_mouse_button_pressed(1);
        
        let mouse_buttons = if left_click || right_click {
            Some((
                mouse_tile_pos.x,
                mouse_tile_pos.y,
                left_click,
                right_click,
                false
            ))
        } else {
            None
        };
        self.resources.insert(mouse_buttons);
        
        let should_spawn_wave = {
            let wave_manager = self.resources.get::<WaveManager>().unwrap();
            !wave_manager.wave_active && wave_manager.spawn_timer <= 0 && wave_manager.current_wave <= 3
        };
        
        if should_spawn_wave {
            let wave_number = self.resources.get::<WaveManager>().unwrap().current_wave;
            let enemies_spawned = {
                let map = self.resources.get::<Map>().unwrap();
                spawn_wave_monsters(&mut self.ecs, &*map, wave_number)
            };
            
            let mut wave_manager = self.resources.get_mut::<WaveManager>().unwrap();
            wave_manager.wave_active = true;
            wave_manager.enemies_remaining = enemies_spawned;
        }
        
        ctx.set_active_console(0);
        
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self.input_systems.execute(
                &mut self.ecs, &mut self.resources
            ),
            TurnState::PlayerTurn => {
                self.player_systems.execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => {
                self.monster_systems.execute(&mut self.ecs, &mut self.resources)
            }
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, 
            "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, 
            "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, 
            "terminal8x8.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, 
            "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, 
            "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
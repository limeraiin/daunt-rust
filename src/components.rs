pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color : ColorPair,
    pub glyph : FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity : Entity,
    pub destination : Point
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mana {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WantsToAttack {
    pub attacker : Entity,
    pub victim : Entity
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanDash {
    pub cost: i32,
    pub range: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanCastFireball {
    pub cost: i32,
    pub damage: i32,
    pub range: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToUseDash {
    pub entity: Entity,
    pub direction: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToUseDashToPoint {
    pub entity: Entity,
    pub target: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToUseFireball {
    pub entity: Entity,
    pub target: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FireballEffect {
    pub center: Point,
    pub radius: i32,
    pub damage: i32,
    pub duration: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaveManager {
    pub current_wave: i32,
    pub enemies_remaining: i32,
    pub wave_active: bool,
    pub spawn_timer: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnemyType {
    Weak,
    Medium,
    Boss,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnemyStats {
    pub enemy_type: EnemyType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FollowsPlayer {
    pub move_timer: i32,
}
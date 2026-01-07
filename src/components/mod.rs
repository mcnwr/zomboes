use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub damage: f32,
}

#[derive(Component)]
pub struct Zombie {
    pub money_reward: u32,
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}
